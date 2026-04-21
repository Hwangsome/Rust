# 5. Unsized Coercion：从 Sized 到 DST 的隐式转换

> - **所属章节**：第 13 章 · Understanding Size in Rust
> - **Cargo package**：`chapter14`
> - **运行方式**：`cargo run -p chapter14`
> - **代码位置**：`chapters/chapter14/src/topic_05_unsized_coercion.rs`
> - **Lab**：`chapters/chapter14/src/lab.rs`
> - **上一篇**：[4. `?Sized` 与泛型参数的配合](./4-?Sized与泛型参数.md)
> - **下一篇**：[6. Never 类型 `!`](./6-Never类型.md)
> - **关键词**：coercion、unsized coercion、`&[T;N]` → `&[T]`、deref coercion、`CoerceUnsized`

---

## 这一节解决什么问题

你写了这样的代码，它能工作，但你可能不知道为什么：

```rust
fn sum(nums: &[i32]) -> i32 { nums.iter().sum() }
let arr = [1, 2, 3, 4, 5];
sum(&arr); // ← &[i32; 5] 怎么变成了 &[i32]？
```

或者：

```rust
let b: Box<dyn std::fmt::Debug> = Box::new(42_i32); // ← Box<i32> 怎么变成 Box<dyn Debug>？
```

这背后的机制叫 **Unsized Coercion**——Rust 在编译期自动把 Sized 类型的指针/引用转换成 DST 的胖指针。

理解这个转换，能让你写出更通用的 API（`&[T]` 比 `&Vec<T>` 通用），也让你看懂很多初学时觉得"神奇"的代码。

---

## 一分钟结论

- Unsized Coercion 是编译器自动把"细指针"升级为"胖指针"的过程
- 常见的四种 coercion：
  1. `&[T; N]` → `&[T]`（数组引用 → 切片引用）
  2. `&String` → `&str`（通过 Deref trait）
  3. `Box<T>` → `Box<dyn Trait>`（具体类型 → trait object）
  4. `&T` → `&dyn Trait`（具体类型引用 → trait object 引用）
- 这些 coercion 只在**特定位置**（coercion site）自动发生
- 背后的 trait 是 `CoerceUnsized`，标准库已经为常见类型实现好了
- 不会自动发生在泛型参数位置（需要显式写）

---

## 与其他语言对比

| 语言 | 数组到"切片/视图"的转换 | 具体类型到"接口/基类"的转换 |
|-----|-------------------|----------------------|
| C++ | `std::span<int>` 需要显式构造 | `Base*` ← `Derived*` 隐式（多态） |
| Java | 数组本身就是对象，通过接口隐式 | `Interface i = new ConcreteClass()` 隐式 |
| Go | `[]int` 切片本身就支持，可以从数组 `arr[:]` 取 | 接口赋值隐式 |
| Rust | `&[T; N]` → `&[T]` 自动 coerce | `&ConcreteT` → `&dyn Trait` 自动 coerce |

Rust 的 coercion 比 Java / Go 更**明确**（必须有 `&`），但比 C++ 更**自动**（不需要你手动构造）。

---

## 核心概念与心智模型

### Coercion 发生的条件

Unsized Coercion 只在以下"coercion site"自动发生：

```
1. 函数/方法的参数位置
   fn f(x: &[i32]) { ... }
   f(&[1,2,3]);     ← 这里自动 coerce

2. let 绑定的类型标注
   let s: &str = &String::from("hello"); ← coerce

3. struct/enum 字段初始化
   Foo { field: &[1, 2, 3] } // 如果 field: &[i32] ← coerce

4. 返回值
   fn foo() -> &str { &String::from("hi") } // ← coerce（但有生命周期问题）

5. 强制类型转换点（as 或 as_ref 等）
```

### 四种主要 Coercion 的内存变化

```
1. &[T; N] → &[T]（数组引用 → 切片）
   之前：细指针（8字节）= 数组地址
   之后：胖指针（16字节）= 数组地址 + N（元素数量）

2. &String → &str（通过 Deref 链）
   &String → &String::deref() 调用 → &str
   实质是 Deref trait，不是纯粹的 Unsized coerce

3. &T → &dyn Trait（具体类型 → trait object）
   之前：细指针（8字节）= T 地址
   之后：胖指针（16字节）= T 地址 + vtable for (T: Trait) 地址

4. Box<T> → Box<dyn Trait>
   之前：细指针（8字节）= 堆上 T 的地址
   之后：胖指针（16字节）= 堆上 T 的地址 + vtable 地址
```

---

## 详细原理

### 1. 数组 → 切片（最常用）

```rust
fn print_slice(s: &[i32]) {
    println!("{:?}, len={}", s, s.len());
}

let arr: [i32; 5] = [1, 2, 3, 4, 5];

// 下面三种写法等价，都发生了 &[i32; 5] → &[i32] 的 coerce
print_slice(&arr);          // 直接传，自动 coerce
print_slice(&arr[..]);      // 显式切片语法
print_slice(arr.as_slice()); // 显式方法调用

// 同样适用于 Vec
let v = vec![10, 20, 30];
print_slice(&v);        // &Vec<i32> → deref 成 &[i32]
print_slice(v.as_slice());

// 嵌套切片
let matrix = [[1, 2], [3, 4], [5, 6]];
// &[[i32; 2]; 3] → &[[i32; 2]]（切片的元素是数组，不是另一层切片）
```

### 2. Deref coercion 链（可以多层）

```rust
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn accept_str(s: &str) { println!("{}", s); }

let mb = MyBox(String::from("hello"));
// MyBox<String> → String（通过 MyBox::Deref）
//              → str（通过 String::Deref）
// 两层 deref，全部自动！
accept_str(&mb);
```

**Deref 链的查找规则**：编译器会反复调用 `deref()`，直到找到目标类型，最多 N 层（不会无限循环，因为每层都会让类型"更接近"目标）。

### 3. `Box<T>` → `Box<dyn Trait>`

```rust
use std::fmt::Debug;

// 下面是合法的：
let b1: Box<i32>        = Box::new(42);
let b2: Box<dyn Debug>  = Box::new(42_i32);  // Box<i32> coerce 成 Box<dyn Debug>

// 也可以显式：
let b3: Box<dyn Debug> = b1 as Box<dyn Debug>; // ← 不行！必须在 coercion site
// 正确写法：
let b4: Box<dyn Debug> = Box::new(42_i32);      // 在初始化位置 coerce

// 注意：coerce 只能从具体类型到 trait object，不能从一个 dyn 到另一个 dyn
let b5: Box<dyn Debug> = b4; // ← 可以（dyn Debug 到 dyn Debug，同类型）
// let b6: Box<dyn Display> = b4; // ← 不行！Debug ≠ Display
```

### 4. `&T` → `&dyn Trait`

```rust
trait Speak { fn speak(&self) -> &str; }
struct Dog;
impl Speak for Dog { fn speak(&self) -> &str { "Woof" } }

fn make_sound(animal: &dyn Speak) {
    println!("{}", animal.speak());
}

let dog = Dog;
make_sound(&dog);   // &Dog → &dyn Speak，自动 coerce
```

---

## 完整运行示例

```rust
use std::fmt::Debug;
use std::mem::size_of;

// 演示 1：数组 → 切片
fn sum_slice(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// 演示 2：各种类型 → &str
fn print_str(s: &str) {
    println!("  str: \"{}\" ({} bytes)", s, s.len());
}

// 演示 3：具体类型 → dyn trait
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
}
struct Dog;
struct Cat;
impl Animal for Dog { fn name(&self) -> &str { "Dog" } fn sound(&self) -> &str { "Woof" } }
impl Animal for Cat { fn name(&self) -> &str { "Cat" } fn sound(&self) -> &str { "Meow" } }

fn describe(a: &dyn Animal) {
    println!("  {} says {}", a.name(), a.sound());
}

fn main() {
    println!("=== 1. &[T; N] → &[T] (数组 → 切片) ===");
    let arr: [i32; 4] = [10, 20, 30, 40];
    let v = vec![1, 2, 3];
    println!("  sum([10,20,30,40]) = {}", sum_slice(&arr));
    println!("  sum([1,2,3])       = {}", sum_slice(&v));
    println!("  sum(1..=5)         = {}", sum_slice(&[1,2,3,4,5]));
    println!();

    println!("=== 2. Deref Coercion 链 ===");
    let owned = String::from("hello");
    let boxed = Box::new(String::from("world"));
    print_str("literal");   // &'static str → &str（trivial）
    print_str(&owned);      // &String → &str（一层 Deref）
    print_str(&boxed);      // &Box<String> → &String → &str（两层 Deref）
    println!();

    println!("=== 3. 具体类型 → &dyn Trait ===");
    let dog = Dog;
    let cat = Cat;
    describe(&dog);  // &Dog → &dyn Animal
    describe(&cat);  // &Cat → &dyn Animal

    // Vec 存放不同类型
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog), // Box<Dog> → Box<dyn Animal>
        Box::new(Cat), // Box<Cat> → Box<dyn Animal>
    ];
    for a in &animals {
        describe(a.as_ref()); // 解引用后再 coerce
    }
    println!();

    println!("=== 4. 指针大小变化 ===");
    println!("  &[i32; 4] size = {}", size_of::<&[i32; 4]>());   // 8（细）
    println!("  &[i32]    size = {}", size_of::<&[i32]>());       // 16（胖）
    println!("  &Dog      size = {}", size_of::<&Dog>());         // 8（细）
    println!("  &dyn Animal  = {}", size_of::<&dyn Animal>());    // 16（胖）
    println!("  Box<i32>  size = {}", size_of::<Box<i32>>());     // 8（细）
    println!("  Box<[i32]>   = {}", size_of::<Box<[i32]>>());    // 16（胖）
}
```

---

## 编译器错误分析

### ❌ 在泛型参数位置 coercion 不自动发生

```rust
fn foo<T: std::fmt::Debug>(x: T) { println!("{:?}", x); }

let arr: [i32; 3] = [1, 2, 3];
// foo(&arr);  这里传的是 &[i32; 3]，T 推断为 &[i32; 3]，不会 coerce 成 &[i32]
// 但也没问题，因为 &[i32; 3] 实现了 Debug

// 问题出在期望 T = &[i32] 但传了 &[i32; 3] 的情况：
fn bar<T: std::fmt::Debug>(x: &[T]) { println!("{:?}", x); }
// bar(&arr); // ❌ 类型不匹配：&[i32; 3] 不是 &[T]
// 必须显式 coerce：
bar(&arr[..]);  // ✓ 显式切片
bar(arr.as_slice()); // ✓ 显式方法
```

---

## 实际工程场景

### 1. 写更通用的函数签名

```rust
// ❌ 只能接受 Vec
fn process_numbers(nums: &Vec<i32>) -> i32 { nums.iter().sum() }

// ✅ 可以接受数组、Vec、切片
fn process_numbers(nums: &[i32]) -> i32 { nums.iter().sum() }

// 调用方不受限制：
process_numbers(&vec![1, 2, 3]);
process_numbers(&[1, 2, 3]);
process_numbers(&some_vec[1..4]);

// 同理：
fn log(msg: &str) { println!("{}", msg); }
// 可以接受字符串字面量、&String、Box<str> 等
```

### 2. 插件/扩展系统

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>, // Box<ConcretePlugin> coerce 到这里
}

impl PluginRegistry {
    fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin)); // Box<P> → Box<dyn Plugin>
    }
}
```

---

## 注意点与陷阱

### 陷阱 1：Coercion 不能"跨越"类型壳

```rust
// &Box<i32> → &Box<dyn Debug> ❌（不会自动发生）
// 因为 coerce 只作用在最外层的"壳"（Box），不会穿透两层
let b: Box<i32> = Box::new(42);
// let r: &Box<dyn Debug> = &b; // ❌ 类型不匹配

// 必须先 coerce Box 再借用，或者反过来：
let d: Box<dyn Debug> = b; // ✓ Box<i32> → Box<dyn Debug>
let r: &Box<dyn Debug> = &d; // ✓ 借用已经是 dyn 的 Box
```

### 陷阱 2：同一个 trait 的多次 impl 导致 coerce 失败

```rust
trait Convert<T: ?Sized> {
    fn convert(&self) -> &T;
}
// 如果一个类型实现了 Convert<str> 和 Convert<[u8]>，
// let x: &str = obj.convert(); // 需要明确是哪个 impl
```

---

## 我的理解与记忆方法

**四条 Coercion 记忆口诀**：

```
数组变切片：加个 [..]
String变str：加个 &
具体变dyn：BoxNew或直接引用
Box升级：初始化时直接写目标类型
```

**判断 coercion 会不会发生**：

```
是不是从"更具体"变成"更抽象/更通用"？
  是 → 在 coercion site？
    是 → 会自动发生
    否 → 需要显式 .as_slice() / as / 等方法
```

---

## 下一步

下一篇讲 Never 类型 `!`——Rust 里永远不产生值的类型，以及它在类型系统中的特殊地位。

- 继续阅读：[6. Never 类型 `!`](./6-Never类型.md)
- 回到目录：[第 13 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Reference - Coercion sites](https://doc.rust-lang.org/reference/type-coercions.html)
