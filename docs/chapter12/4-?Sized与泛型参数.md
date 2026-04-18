# 4. `?Sized` 与泛型参数的配合

> - **所属章节**：第 12 章 · Understanding Size in Rust
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_04_optionally_sized_trait_and_generic_parameters.rs`
> - **Lab**：`chapters/chapter12/src/lab.rs`
> - **上一篇**：[3. Sized 与 `?Sized` 可选大小绑定](./3-Sized与?Sized.md)
> - **下一篇**：[5. Unsized Coercion（非固定大小强转）](./5-unsized强转.md)
> - **关键词**：`?Sized`、尾部字段、unsized struct、`Box<T>`、`Rc<T>`、标准库惯用法

---

## 这一节解决什么问题

上一篇讲了函数签名里的 `?Sized`——接受 `&str` / `&[T]` 等 DST 参数。但 `?Sized` 还有更深的一层应用场景：**struct 的字段本身就是 DST**。

标准库里的 `Box<T>`、`Rc<T>`、`Arc<T>`、`Cow<'a, B>` 等都有 `T: ?Sized` 约束——正是这个设计让你能写 `Box<str>`、`Rc<[u8]>`、`Box<dyn Error>` 而不只是 `Box<String>`、`Rc<Vec<u8>>`。

这一篇重点讲：
- 泛型 struct 里 `T: ?Sized` 的规则（尾部字段限制）
- 标准库是怎么用这个特性的
- 什么时候你自己的 struct 需要 `T: ?Sized`

---

## 一分钟结论

- 泛型 struct 如果字段包含 DST 类型 T，必须写 `<T: ?Sized>`
- DST 字段必须是 struct 的**最后一个字段**（"尾部字段"）
- 通过 `Box<MyStruct<str>>` 这样的胖指针才能在堆上构造含 DST 字段的 struct
- 标准库的 `Box<T>`、`Rc<T>`、`Arc<T>` 都用了 `T: ?Sized`，这是它们支持 `Box<str>` 的原因
- `PhantomData<T>` 常用于"假装持有 T 的类型信息"但不真的存 T，也支持 `T: ?Sized`
- 绝大多数业务代码**不需要**写 `T: ?Sized` 的 struct，这是库设计者才常用的技巧

---

## 与其他语言对比

| 语言 | 如何处理"字段大小不固定"的容器 |
|-----|--------------------------|
| C | `struct { ... }` 结尾加灵活数组成员 `int data[];`（C99 FAM），手动管理 |
| C++ | `std::vector` / `std::string` 用指针间接，不是"结尾嵌入" |
| Java | 一切对象都是引用，没有"嵌入大小不固定数据"的概念 |
| Rust | 通过 `T: ?Sized` + 尾部字段规则，让编译器自动处理胖指针 |

---

## 核心概念与心智模型

### `Box<T: ?Sized>` 的内存布局差异

```
Box<i32>（T = i32，Sized）
┌──────────────────┐    栈上 Box 结构体（8字节，细指针）
│  heap_ptr (8字节)│────────────────────▶ ┌────────┐ 堆
└──────────────────┘                      │  42    │（4字节）
                                          └────────┘

Box<str>（T = str，DST）
┌──────────────────────────────────────┐  栈上 Box 结构体（16字节，胖指针）
│  heap_ptr (8字节) │  len (8字节)      │
└──────────────────────────────────────┘
         │                 │
         ▼                 ▼
    ┌─────────────┐       = 5
    │ h e l l o  │（堆，5字节UTF-8）
    └─────────────┘

Box<dyn Error>（T = dyn Error，DST）
┌──────────────────────────────────────┐  栈上 Box 结构体（16字节，胖指针）
│  heap_ptr (8字节) │ vtable_ptr(8字节) │
└──────────────────────────────────────┘
         │                 │
         ▼                 ▼
    ┌─────────────┐   ┌──────────────────────┐
    │ error data  │   │ drop / size / align  │ vtable
    └─────────────┘   │ source / description │
                      └──────────────────────┘
```

**关键结论**：`Box<T>` 本身始终是 Sized（它就是一个指针），但当 `T` 是 DST 时，这个"指针"升级为胖指针，占 16 字节。

### struct 的尾部字段（Trailing Field）规则

```rust
// 正确：DST 字段放最后
struct StrWrapper {
    header: u64,   // Sized 字段
    data: str,     // DST 字段，必须是最后一个
}
// 这个 struct 本身是 DST，只能通过 Box<StrWrapper> 使用

// 错误：DST 字段不在最后
struct Bad {
    data: str,     // ❌ DST 字段
    footer: u64,   // 还有 Sized 字段跟在后面
}
// error: the type `str` is not `Sized` and must be the last field in a struct
```

---

## 详细原理

### 1. 标准库是如何定义 `Box<T>` 的

```rust
// 标准库 Box<T> 的（简化）定义
pub struct Box<T: ?Sized>(Unique<T>);
//                ^^^^^^
//  没有这个 ?Sized，就不能有 Box<str>、Box<dyn Error>

// 正因为 T: ?Sized，所以：
let b1: Box<i32> = Box::new(42);        // T = i32（Sized），Box 是细指针
let b2: Box<str> = "hello".into();      // T = str（DST），Box 是胖指针
let b3: Box<[u8]> = vec![1,2,3].into();// T = [u8]（DST），Box 是胖指针
let b4: Box<dyn std::error::Error> = ..;// T = dyn Error（DST），Box 是胖指针
```

### 2. `Rc<T: ?Sized>` 和 `Arc<T: ?Sized>`

```rust
use std::rc::Rc;
use std::sync::Arc;

// Rc 支持 DST：
let rc1: Rc<i32> = Rc::new(42);
let rc2: Rc<str> = Rc::from("hello"); // ← 这能成立，因为 Rc<T: ?Sized>
let rc3: Rc<[i32]> = Rc::from(vec![1, 2, 3].as_slice());
let rc4: Rc<dyn std::fmt::Debug> = Rc::new(42_i32) as Rc<dyn std::fmt::Debug>;

// Arc 同理：
let arc: Arc<dyn Send + Sync> = Arc::new(42_i32);
```

### 3. 自定义带 DST 字段的 struct（高级）

这是库级别的技巧，普通业务代码极少需要，但理解它有助于看懂标准库源码：

```rust
use std::alloc::{alloc, Layout};
use std::mem;

// 一个自定义的"带头部的字节缓冲区"
struct Buffer {
    capacity: usize,
    data: [u8], // DST 尾部字段
}

// 不能直接用 Buffer { capacity: 4, data: [1,2,3,4] }
// 必须通过 Box<Buffer> 或 unsafe 内存操作构造

// 通常这种场景用 Vec<u8> 就够了，上面只是说明机制
```

### 4. `Cow<'a, B>` 的 `B: ?Sized + ToOwned`

```rust
use std::borrow::Cow;

// Cow 的定义中 B: ?Sized，所以可以有 Cow<'_, str>（最常见用法）
let borrowed: Cow<'_, str> = Cow::Borrowed("hello");
let owned:    Cow<'_, str> = Cow::Owned(String::from("world"));

// 对比：如果 B 不是 ?Sized，就不能有 Cow<'_, str>
// 因为 str 是 DST
println!("{}", borrowed);
println!("{}", owned);
```

### 5. `PhantomData<T>` 和 `?Sized`

```rust
use std::marker::PhantomData;

// PhantomData<T: ?Sized> 让你声明"和 T 相关"但不存储 T
struct Typed<T: ?Sized> {
    id: u64,
    _phantom: PhantomData<*const T>, // 不存储 T，只是声明类型关联
}

// 这让 Typed<str>、Typed<[u8]>、Typed<dyn Trait> 都合法
// 常用于 newtype / ID 系统
struct UserId(u64);
struct OrderId(u64);
// 更进一步，用 PhantomData 绑定类型参数，防止混用
struct Id<T: ?Sized>(u64, PhantomData<*const T>);
type UserIdTyped = Id<UserTag>;
type OrderIdTyped = Id<OrderTag>;
```

---

## 完整运行示例

```rust
use std::rc::Rc;
use std::borrow::Cow;
use std::mem::size_of;

fn demonstrate_box_sized_vs_dst() {
    println!("=== Box<T>: Sized vs DST ===");
    println!("Box<i32>  = {} bytes", size_of::<Box<i32>>());  // 8（细指针）
    println!("Box<str>  = {} bytes", size_of::<Box<str>>());  // 16（胖指针）
    println!("Box<[i32]>= {} bytes", size_of::<Box<[i32]>>()); // 16（胖指针）

    let b_int:   Box<i32>  = Box::new(42);
    let b_str:   Box<str>  = "hello".into();
    let b_slice: Box<[i32]>= Box::new([1, 2, 3]);

    println!("Box<i32>:   {b_int}");
    println!("Box<str>:   {b_str}");
    println!("Box<[i32]>: {b_slice:?}");
}

fn demonstrate_rc_dst() {
    println!("\n=== Rc<T> with DST ===");
    let rc_str: Rc<str> = Rc::from("shared string");
    let rc_clone = Rc::clone(&rc_str);
    println!("rc_str: {rc_str}, strong_count: {}", Rc::strong_count(&rc_str));
    println!("rc_clone: {rc_clone}");
    println!("Rc<str> size: {} bytes", size_of::<Rc<str>>()); // 16（胖指针）
}

fn demonstrate_cow() {
    println!("\n=== Cow<'_, str> ===");
    fn process(input: &str) -> Cow<'_, str> {
        if input.contains(' ') {
            // 需要修改：产生 Owned
            Cow::Owned(input.replace(' ', "_"))
        } else {
            // 不需要修改：返回 Borrowed（零拷贝！）
            Cow::Borrowed(input)
        }
    }

    let r1 = process("hello");
    let r2 = process("hello world");
    println!("process(\"hello\"):       {:?} (borrowed: {})", r1, matches!(r1, Cow::Borrowed(_)));
    println!("process(\"hello world\"): {:?} (owned: {})",    r2, matches!(r2, Cow::Owned(_)));
}

fn main() {
    demonstrate_box_sized_vs_dst();
    demonstrate_rc_dst();
    demonstrate_cow();
}
```

---

## 实际工程场景

### 1. 错误处理中的 `Box<dyn Error>`

```rust
use std::error::Error;

// 函数可以返回任何类型的错误——不需要在签名里列举所有可能
fn do_something() -> Result<(), Box<dyn Error>> {
    let n: i32 = "42a".parse()?;       // ParseIntError
    let file = std::fs::read("x.txt")?;// io::Error
    Ok(())
}
// Box<dyn Error> 能装下任何实现了 std::error::Error 的类型
// 因为 Error: ?Sized，所以 Box<dyn Error> 合法
```

### 2. 字符串处理中的 `Box<str>` vs `String`

```rust
// String：堆分配 + 可变，有容量（capacity）开销
let s: String = "hello".to_string();  // 24字节栈结构体

// Box<str>：堆分配 + 不可变，无容量浪费，比 String 省 8 字节
let b: Box<str> = "hello".into();     // 16字节胖指针
// 适合：确定内容不会变、需要和 &str 频繁交互的只读字符串
```

### 3. `Rc<[T]>` 共享不可变数组

```rust
use std::rc::Rc;

// 不可变共享数组，相比 Rc<Vec<T>> 省了 capacity 字段
let shared: Rc<[i32]> = Rc::from(vec![1, 2, 3]);
let clone1 = Rc::clone(&shared);
let clone2 = Rc::clone(&shared);
println!("shared[0] = {}", shared[0]);
// 多个所有者共享同一份数组数据，零拷贝
```

---

## 注意点与陷阱

### 陷阱 1：不能直接构造含 DST 尾部字段的 struct

```rust
struct Msg {
    len: u32,
    data: str,
}
// let m = Msg { len: 5, data: "hello" }; // ❌ 编译失败！
// 只能通过 unsafe + 裸指针，或借助标准库 API 间接构造
```

大多数情况下，用 `String` / `Vec<T>` / `Box<str>` 就可以了，不需要自定义含 DST 尾部字段的 struct。

### 陷阱 2：`Box<dyn Trait>` 和 `Box<ConcreteType>` 大小不同

```rust
println!("{}", size_of::<Box<i32>>());        // 8（细指针）
println!("{}", size_of::<Box<dyn Debug>>());  // 16（胖指针）
// 如果你的代码依赖 Box<T> 的大小，要注意这个差异
```

---

## 我的理解与记忆方法

**场景分类记忆**：

```
自己写的业务函数：几乎不需要 T: ?Sized
   ↓
工具函数（想接受 &str / &[T]）：参数加 &T，约束加 ?Sized
   ↓
库设计者（想让 Box<T> 支持 DST）：struct<T: ?Sized>，Box 自动变胖指针
```

**标准库中 `?Sized` 的规律**：凡是"容器"类型（`Box`、`Rc`、`Arc`、`Cow`）都有 `T: ?Sized`，这是让它们"通用"的根本原因。

---

## 下一步

下一篇讲 Unsized Coercion：Rust 如何隐式把 `&[i32; 3]` 变成 `&[i32]`，把 `Box<i32>` 变成 `Box<dyn Trait>`——这是日常代码里最常"默默发生"的类型转换。

- 继续阅读：[5. Unsized Coercion（非固定大小强转）](./5-unsized强转.md)
- 回到目录：[第 12 章：Understanding Size in Rust](./README.md)
- 延伸阅读：[Rustonomicon - Slice DST and Custom DST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts)
