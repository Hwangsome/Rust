# 3. Sized 与 `?Sized`：泛型参数的默认约束

> - **所属章节**：第 13 章 · Understanding Size in Rust
> - **Cargo package**：`chapter14`
> - **运行方式**：`cargo run -p chapter14`
> - **代码位置**：`chapters/chapter14/src/topic_03_sized_and_optionally_sized_trait.rs`
> - **Lab**：`chapters/chapter14/src/lab.rs`
> - **上一篇**：[2. 对 Unsized 类型的引用（胖指针）](./2-对Unsized的引用.md)
> - **下一篇**：[4. `?Sized` 与泛型参数的配合](./4-?Sized与泛型参数.md)
> - **关键词**：`Sized`、`?Sized`、marker trait、泛型约束、隐式 bound、E0277

---

## 这一节解决什么问题

你现在知道了 DST 不能按值传递，只能通过胖指针访问。但问题来了：Rust 的泛型是怎么知道某个 `T` 是不是 DST？如何在类型系统层面**表达"这个 T 必须有固定大小"**？

答案是：`Sized` trait。

每个泛型参数默认都带一个**隐式约束** `T: Sized`，编译器悄悄给你加了它。这就是为什么你写 `fn foo<T>(x: T)` 时，`T` 不能是 `str` 或 `dyn Trait`。

而当你确实需要让泛型接受 DST（比如你想写一个对 `str` 和 `[T]` 都能用的工具函数），就需要用 `?Sized` 来**取消这个隐式约束**。

这一节要彻底理解：

- `Sized` 是一个特殊的 marker trait，不能手动 impl
- 所有泛型参数默认都有 `T: Sized` 隐式约束
- `?Sized` 的含义是"T 可能是 Sized，也可能不是"（放开约束）
- 放开后，T 只能通过 `&T`、`Box<T>` 等指针访问，不能按值传递

---

## 一分钟结论

- `Sized` trait 标记"类型大小在编译期已知"，所有非 DST 类型都自动实现它
- 泛型参数默认隐式加了 `T: Sized`，所以 `fn f<T>(x: T)` 等价于 `fn f<T: Sized>(x: T)`
- `?Sized` 表示"选择性放开 Sized 约束"，让泛型可以接受 DST
- 写 `T: ?Sized` 时参数**必须**通过引用 `&T` 或 `Box<T>` 等传递，不能按值传
- `str`、`[T]`、`dyn Trait` 都**不**实现 `Sized`
- `where T: ?Sized` 是唯一放开默认 Sized 约束的语法

---

## 与其他语言对比

| 概念 | C++ | Java | Rust |
|-----|-----|------|------|
| 泛型类型约束 | `template<typename T>` 无约束 | `<T>` 基本无大小约束（都是引用） | `<T>` 隐式 `T: Sized`，必须手动放开 |
| 变长类型 | 模板特化 | 都是引用，无问题 | 需要 `?Sized` 才能处理 DST |
| 泛型函数收参 | 值语义复制或引用 | 始终是引用语义 | 默认值语义（受 Sized），`&T` 才能接 DST |

Java 程序员不会有这个问题，因为 Java 泛型的 `T` 始终是引用类型，大小固定（引用就是指针，8字节）。Rust 的泛型可以按值拿到 T 的所有权，这要求 T 的大小在编译期固定。

---

## 核心概念与心智模型

### `Sized` 是一个 marker trait，不是你能 impl 的 trait

```rust
// 下面是 Sized 的（概念性）定义——你在标准库里看到的
pub trait Sized {}
// 特点：
// 1. 你不能手动 impl Sized for MyType，编译器自动推断
// 2. 所有大小固定的类型都自动满足 Sized
// 3. str、[T]、dyn Trait 自动不满足 Sized
```

### 隐式约束：编译器替你加了什么

```rust
// 你写的：
fn print_value<T>(x: T) {
    // ...
}

// 编译器实际理解的：
fn print_value<T: Sized>(x: T) {
    // ...
}

// 所以这个调用会失败：
// print_value("hello".to_string().as_str() as str); // 无法按值传 str
```

### `?Sized` 放开约束的语义

```
T: Sized    →  T 必须是固定大小（默认）
T: ?Sized   →  T 可以是固定大小，也可以不是（放开限制）

注意：?Sized 是在约束上加"?"，不是一个独立 trait
```

```
带 Sized 约束（默认）              带 ?Sized（放开后）
┌─────────────────────┐           ┌─────────────────────┐
│  T 的范围：          │           │  T 的范围：          │
│  i32 ✓              │           │  i32 ✓              │
│  String ✓           │           │  String ✓           │
│  Vec<u8> ✓          │           │  Vec<u8> ✓          │
│  str ✗（DST）       │           │  str ✓（允许了！）   │
│  [T] ✗（DST）       │           │  [T] ✓（允许了！）   │
│  dyn Trait ✗（DST） │           │  dyn Trait ✓（允许了）│
└─────────────────────┘           └─────────────────────┘
可以按值传 T                       不能按值传 T（必须用 &T）
```

---

## 详细原理

### 1. 默认约束导致的编译错误

```rust
// 普通泛型函数：T 隐式是 Sized
fn print_it<T>(value: T) {
    println!("{:?}", value);
}

// 这会失败，因为 str 不是 Sized：
// fn bad() {
//     let s: Box<str> = "hello".into();
//     print_it(*s); // ❌ E0277: str 没有固定大小
// }
```

### 2. 用 `?Sized` 接受 DST

```rust
use std::fmt::Debug;

// 必须用 &T（因为 T 可能是 DST，不知道大小）
fn print_it<T: Debug + ?Sized>(value: &T) {
    println!("{:?}", value);
}

fn main() {
    print_it(&42_i32);           // T = i32（Sized）
    print_it("hello");           // T = str（DST！）
    print_it(&[1, 2, 3] as &[_]);// T = [i32]（DST！）

    // 通过 Box<T> 也可以：
    let boxed: Box<str> = "world".into();
    print_it(&*boxed);           // T = str（DST）
}
```

### 3. `?Sized` 在 trait 方法里的隐式使用

```rust
// std::fmt::Debug 的（简化）定义：
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    //     ^^^^
    // 这里 self 的类型是 &Self，而 Self: ?Sized（trait 的 Self 默认是 ?Sized）
    // 这就是为什么你可以对 &str 调用 {:?}，而不需要 str: Sized
}
```

**关键点**：trait 的 `Self` 默认是 `?Sized`（不像普通泛型函数的 `T` 默认是 `Sized`）。这让 trait 方法可以在 DST 上被调用，只要是通过 `&self` 或 `&mut self`。

### 4. 泛型结构体里的 `?Sized`

```rust
// 默认：T 必须是 Sized，str 不行
struct Wrapper1<T> {
    value: T,   // 按值存储，T 必须 Sized
}

// 放开：T 可以是 DST，但字段必须是最后一个，且存放方式受限
struct Wrapper2<T: ?Sized> {
    header: u32,
    value: T,   // 必须是最后一个字段，且 T 是 DST 时只能用 Box<Wrapper2<str>> 等
}

// 常见做法：加一个 _marker 字段来"声明和 T 有关"，但 T 是 DST
use std::marker::PhantomData;
struct Visitor<T: ?Sized> {
    id: u64,
    _phantom: PhantomData<*const T>, // 告诉编译器"我关心T的类型信息"，但不真的存T
}
```

---

## 完整运行示例

```rust
use std::fmt::Debug;
use std::mem::size_of;

// 1. 默认 Sized 约束
fn requires_sized<T: Debug>(x: T) {
    println!("Sized: size={}, value={:?}", size_of::<T>(), x);
}

// 2. ?Sized 放开约束
fn accepts_any<T: Debug + ?Sized>(x: &T) {
    // 用 size_of_val 获取运行时大小
    println!("?Sized: size_of_val={}, value={:?}",
             std::mem::size_of_val(x), x);
}

// 3. 自定义 ?Sized trait
trait Printable {
    fn print_me(&self); // self: &Self，Self: ?Sized（trait 默认）
}

impl Printable for str {
    fn print_me(&self) { println!("str: \"{}\" ({} bytes)", self, self.len()); }
}
impl Printable for [i32] {
    fn print_me(&self) { println!("[i32]: {:?} ({} elements)", self, self.len()); }
}
impl Printable for i32 {
    fn print_me(&self) { println!("i32: {}", self); }
}

fn main() {
    // === Sized 路径（按值传递）===
    println!("--- requires_sized ---");
    requires_sized(42_i32);
    requires_sized(String::from("owned string"));
    // requires_sized("hello"); // 注意：字面量 "hello" 是 &str，不是 str
    // 下面这行会失败（str 不是 Sized）：
    // let s: Box<str> = "hello".into();
    // requires_sized(*s); // ❌ E0277

    println!();

    // === ?Sized 路径（通过引用传递）===
    println!("--- accepts_any ---");
    accepts_any(&42_i32);           // T = i32 (Sized)
    accepts_any("hello world");     // T = str (DST!)
    accepts_any(&[1, 2, 3][..]);    // T = [i32] (DST!)
    accepts_any(&vec![1, 2, 3]);    // T = Vec<i32> (Sized)

    println!();

    // === trait 在 DST 上的调用 ===
    println!("--- Printable trait on DST ---");
    let s: &str = "Rust";
    let v: &[i32] = &[10, 20, 30];
    let n: i32 = 99;
    s.print_me();
    v.print_me();
    n.print_me();

    // 通过 dyn：
    println!();
    let printables: Vec<Box<dyn Printable>> = vec![
        Box::new(String::from("hello")),   // ❌ 需要 impl Printable for String
        // 注意：Box<str> 和 Box<[i32]> 本身就是胖指针
    ];
    // 更准确的演示：
    let a: &dyn Printable = &42_i32;
    let b: &dyn Printable = "world";
    let c: &dyn Printable = &[1i32, 2, 3] as &[i32];
    a.print_me();
    b.print_me();
    c.print_me();
}
```

---

## 编译器错误分析

### ❌ E0277：DST 不满足 `Sized` bound

```rust
fn foo<T>(x: T) {} // 隐式 T: Sized

foo("hello" as str); // ❌
```

```text
error[E0277]: the size for values of type `str` cannot be known at
              compilation time
  |
  | foo("hello" as str);
  |     ^^^^^^^^^^
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: all local variables must have a statically known size
```

**修复**：改为接受 `&str` 或加 `?Sized`

```rust
fn foo<T: ?Sized>(x: &T) {} // 接受 &str、&[T]、&dyn Trait 等
foo("hello");  // ✓
```

### ❌ 在带 `?Sized` 的泛型里按值使用 T

```rust
fn bad<T: ?Sized>(x: T) { // ❌ 按值接收 T，但 T 可能是 DST
    //           ^^^
}
```

```text
error[E0277]: the size for values of type `T` cannot be known at
              compilation time
```

**修复**：始终用 `&T` 或 `Box<T>` 接收 `?Sized` 的 T

---

## 实际工程场景

### 标准库中 `?Sized` 的大量使用

```rust
// From std::borrow::Borrow
pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}
// 这允许 String 实现 Borrow<str>：
// impl Borrow<str> for String {
//     fn borrow(&self) -> &str { self.as_str() }
// }

// From std::ops::Deref
pub trait Deref {
    type Target: ?Sized; // Target 可以是 DST！
    fn deref(&self) -> &Self::Target;
}
// 这就是为什么 Box<str> 可以 deref 成 &str

// From std::cmp::PartialEq
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
}
// String == &str 就是这么工作的：
// impl PartialEq<str> for String { ... }
```

### 自定义可接受 DST 的函数

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// 可以对任何可哈希类型（包括 str、[u8] 这样的 DST）计算哈希
fn compute_hash<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

fn main() {
    println!("{}", compute_hash("hello"));              // T = str
    println!("{}", compute_hash(&42_i32));              // T = i32
    println!("{}", compute_hash(&b"hello world"[..])); // T = [u8]
}
```

---

## 注意点与陷阱

### 陷阱 1：`?Sized` 和 `Sized` 不能同时出现

```rust
fn foo<T: Sized + ?Sized>() {} // ❌ 逻辑矛盾，编译会警告/报错
```

### 陷阱 2：`?Sized` 只在需要的地方用，不要滥用

```rust
// 过度使用：大多数函数根本不需要接受 DST
fn add_one<T: Add<Output = T> + ?Sized>(x: &T, y: &T) -> T { ... }
// ❌ ?Sized 没有意义：Add::add 按值接收参数，T 不能是 DST

// 正确：只在你真的需要接受 str、[T]、dyn Trait 时才加 ?Sized
```

### 陷阱 3：trait 里的 `Self: Sized` 约束

```rust
trait Shape {
    fn area(&self) -> f64;

    // 这个方法只在具体类型上可用，不能通过 dyn Shape 调用
    fn clone_shape(&self) -> Self where Self: Sized {
        // 如果没有 where Self: Sized，这个 trait 就不是 object-safe 的
        // 因为 dyn Shape 的 Self 是 ?Sized
        unimplemented!()
    }
}
```

---

## 我的理解与记忆方法

**记忆口诀**：

```
泛型默认 Sized，想用 DST 加问号（?Sized）
加了问号必须引用，值语义不再可能
```

**判断是否需要 `?Sized` 的决策树**：

```
我的泛型函数需要接受 &str 或 &[T] 或 &dyn Trait 吗？
  ├── 是 → 加 ?Sized，参数改为 &T
  └── 否 → 保持默认，参数可以是 T（按值）
```

**对比记忆**：

| | 普通泛型 `<T>` | 可选大小 `<T: ?Sized>` |
|-|-------------|---------------------|
| T 的范围 | 仅 Sized 类型 | Sized + DST 类型 |
| 参数形式 | `x: T`（可按值）| `x: &T`（必须引用）|
| 使用场景 | 大多数函数 | 需要接受 str/[T]/dyn |

---

## 下一步

下一篇会继续深化 `?Sized`——当 struct 的字段本身需要是 DST 时，语法细节有哪些要注意。

- 继续阅读：[4. `?Sized` 与泛型参数的配合](./4-?Sized与泛型参数.md)
- 回到目录：[第 13 章：Understanding Size in Rust](./README.md)
- 官方参考：[The `Sized` Trait - Rust Reference](https://doc.rust-lang.org/reference/special-types-and-traits.html#sized)
- 官方参考：[Trait Objects - Rust Reference](https://doc.rust-lang.org/reference/types/trait-object.html)
