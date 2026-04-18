# 1. Rust 中的类型大小（Sized 与 Unsized）

> - **所属章节**：第 12 章 · Understanding Size in Rust
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_01_size_in_rust.rs`
> - **Lab**：`chapters/chapter12/src/lab.rs`
> - **上一篇**：本章第一篇
> - **下一篇**：[2. 对 Unsized 类型的引用（胖指针）](./2-对Unsized的引用.md)
> - **关键词**：`Sized`、`DST`、`str`、`[T]`、`dyn Trait`、`size_of`、胖指针、fat pointer

---

## 这一节解决什么问题

在 C / C++ 里，`sizeof(T)` 是每个程序员从入门就会用的东西，但没人会刻意讨论"类型大小"，因为几乎所有类型的大小在编译期都是已知的。

在 Java / Python 里，你几乎不需要考虑这件事——GC 和虚拟机替你管理了一切。

Rust 不同。Rust 要在**没有 GC 的情况下保证内存安全**，而且要做到零运行期开销。这要求编译器在编译期知道每个变量占多少栈空间——**这就需要每个变量的类型大小在编译期是确定的**。

但有些类型的大小天然不固定：字符串 `str` 的长度在运行时才知道；切片 `[T]` 的元素个数在运行时才知道；`dyn Trait` 的具体类型在编译时不知道。这类类型在 Rust 里叫做 **DST（Dynamically Sized Type，动态大小类型）** 或 **Unsized 类型**。

理解 Sized 与 Unsized 的边界，是理解：

- 为什么 `Box<dyn Trait>` 和 `&dyn Trait` 必须通过指针访问
- 为什么泛型参数默认带隐式 `Sized` 约束
- 为什么 `&[T]`、`&str` 是 16 字节而不是 8 字节
- 为什么 trait object 需要"胖指针"

的根本原因。

---

## 一分钟结论

- Rust 类型分两大阵营：**Sized**（编译期大小固定）和 **Unsized / DST**（运行期大小才确定）
- 几乎所有"普通"类型都是 Sized：`i32`、`String`、`Vec<T>`、`(A, B)`、`[T; N]`
- 只有三类 DST：`str`（字节序列）、`[T]`（元素序列）、`dyn Trait`（trait 对象）
- DST 不能按值存储或传递，**只能通过引用或智能指针访问**
- 对 Sized 类型的引用是**"细指针"（thin pointer）**，8 字节
- 对 DST 的引用是**"胖指针"（fat pointer）**，16 字节（地址 + 元数据）

---

## 与其他语言对比


| 概念         | C / C++                 | Java             | Python             | Rust                                   |
| ---------- | ----------------------- | ---------------- | ------------------ | -------------------------------------- |
| 获取类型大小     | `sizeof(T)`             | 不直接暴露            | `sys.getsizeof(x)` | `std::mem::size_of::<T>()`             |
| 字符串如何存储    | `char`* 或 `std::string` | `String`（堆，长度可变） | 隐藏细节               | `&str`（胖指针）或 `String`（堆上拥有型）           |
| 动态大小类型     | 无一等公民                   | 所有对象引用（多态）       | 所有对象               | `str`、`[T]`、`dyn Trait`                |
| "不定长"的解决方案 | 指针 + 长度手动维护             | 对象引用（自动）         | 对象引用（自动）           | 胖指针（编译器自动生成）                           |
| 多态的成本      | vtable 或函数指针            | vtable（JVM 优化）   | 字典查找               | `dyn Trait`（运行期 vtable）或泛型（编译期单态化，零成本） |


**关键差异**：

- Java 里所有非 primitive 类型都是引用语义，你永远不会"直接拥有"一个对象的内存。Rust 的默认是**值语义**，你直接拥有数据，只有明确写 `&` 才是借用。
- Python 里一切都是引用，你从来不考虑大小。Rust 要求你理解这个，否则写不了 DST 相关代码。
- C 里你手动维护"指针 + 长度"，极易出 bug。Rust 的胖指针在语言层面保证了这个配对是安全的。

---

## 核心概念与心智模型

### Sized 类型：编译期已知大小

```
┌─────────────────────────────────────────────────────────┐
│                     栈（Stack）                          │
│                                                          │
│  let x: i32 = 42;                                       │
│  ┌──────────┐                                           │
│  │    42    │  ← 4 字节，编译期已知                      │
│  └──────────┘                                           │
│                                                          │
│  let p: Point { x: i32, y: i32 } = Point { x:1, y:2 }  │
│  ┌────────────────────┐                                  │
│  │  x: 1  │  y: 2    │  ← 8 字节，编译期已知             │
│  └────────────────────┘                                  │
└─────────────────────────────────────────────────────────┘
```

编译器可以直接在栈上为这些变量分配精确的空间，因为大小在编译期完全已知。

### DST（Unsized）类型：运行期才知道大小

```
┌─────────────────────────────────────────────────────────┐
│              str 和 [T]：大小在运行期才知道               │
│                                                          │
│  "hello"    ← 5 字节                                    │
│  "hello, world"  ← 12 字节                              │
│                                                          │
│  所以 str 本身不能放在栈上，因为编译器不知道该分配多少空间  │
│                                                          │
│  dyn Trait  ← 运行期才知道是 Dog 还是 Cat，大小不同       │
└─────────────────────────────────────────────────────────┘
```

### 胖指针（Fat Pointer）：对 DST 的引用

```
┌─────────────────────────────────────────────────────────┐
│  细指针（Thin Pointer）对 Sized 类型：8 字节              │
│                                                          │
│  let r: &i32 = &42;                                     │
│  ┌───────────────┐                                      │
│  │    地址       │  8 字节（指针）                        │
│  └───────────────┘                                      │
│           │                                             │
│           ▼ 指向栈/堆上的 i32                            │
│       ┌────────┐                                        │
│       │   42   │                                        │
│       └────────┘                                        │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  胖指针（Fat Pointer）对 &str：16 字节                    │
│                                                          │
│  let s: &str = "hello";                                 │
│  ┌───────────────┬────────────────┐                     │
│  │  数据地址      │  长度 (5)       │  16 字节            │
│  └───────────────┴────────────────┘                     │
│           │                                             │
│           ▼ 指向只读内存段里的字节                         │
│       ┌───────────────┐                                 │
│       │ h e l l o    │                                  │
│       └───────────────┘                                 │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  胖指针（Fat Pointer）对 &dyn Trait：16 字节              │
│                                                          │
│  let d: &dyn Animal = &Dog;                             │
│  ┌───────────────┬────────────────┐                     │
│  │  数据地址      │  vtable 指针   │  16 字节            │
│  └───────────────┴────────────────┘                     │
│           │              │                              │
│           │              ▼ vtable（方法表）               │
│           │          ┌──────────────────┐               │
│           │          │ drop: fn(*mut T) │               │
│           │          │ size: usize      │               │
│           │          │ speak: fn(*T)    │               │
│           │          └──────────────────┘               │
│           ▼ 指向 Dog 实例                                │
│       ┌──────────┐                                      │
│       │  Dog {}  │                                      │
│       └──────────┘                                      │
└─────────────────────────────────────────────────────────┘
```

**关键区别总结**：

- `&[i32; 3]`：细指针，8 字节，因为数组长度是类型的一部分
- `&[i32]`：胖指针，16 字节，因为切片长度只有运行时才知道
- `&dyn Trait`：胖指针，16 字节（数据地址 + vtable 地址）

---

## 详细原理

### 1. 用 `size_of` 亲自验证

```rust
use std::mem::size_of;

// Sized 类型
println!("i8              = {} 字节", size_of::<i8>());         // 1
println!("i32             = {} 字节", size_of::<i32>());        // 4
println!("i64             = {} 字节", size_of::<i64>());        // 8
println!("(i32, i32)      = {} 字节", size_of::<(i32, i32)>());// 8
println!("[i32; 3]        = {} 字节", size_of::<[i32; 3]>());  // 12
println!("bool            = {} 字节", size_of::<bool>());       // 1
println!("char            = {} 字节", size_of::<char>());       // 4（Unicode 标量值）

// 指针和智能指针
println!("&i32            = {} 字节", size_of::<&i32>());       // 8（细指针）
println!("Box<i32>        = {} 字节", size_of::<Box<i32>>());   // 8（细指针）
println!("fn(i32) -> i32  = {} 字节", size_of::<fn(i32)->i32>());// 8

// DST 的引用 = 胖指针
println!("&[i32]          = {} 字节", size_of::<&[i32]>());     // 16（地址+长度）
println!("&str            = {} 字节", size_of::<&str>());       // 16（地址+长度）
println!("&dyn SomeTrait  = {} 字节", size_of::<&dyn SomeTrait>()); // 16（地址+vtable）
```

> 注意：`size_of::<str>()` 会编译失败，因为 `str` 是 DST，编译期大小不定。
> 但 `size_of::<&str>()` 合法，因为引用本身（胖指针）大小是确定的（16 字节）。

### 2. struct 的内存布局与字段对齐

```rust
// 字段顺序影响 struct 大小（内存对齐）
struct Bad {
    a: u8,   // 1 字节
    b: u64,  // 8 字节，需要 8 字节对齐
    c: u8,   // 1 字节
}
// 实际大小：1 + 7（填充）+ 8 + 1 + 7（填充）= 24 字节！

struct Good {
    b: u64,  // 8 字节
    a: u8,   // 1 字节
    c: u8,   // 1 字节
}
// 实际大小：8 + 1 + 1 + 6（填充）= 16 字节

println!("Bad  = {}", size_of::<Bad>());   // 24
println!("Good = {}", size_of::<Good>());  // 16
```

Rust 默认按字段的最大对齐要求进行对齐，顺序不同会影响最终大小。如果想强制紧凑排列，可以用 `#[repr(packed)]`，但会带来未对齐访问性能损失。

### 3. 为什么不能直接用 `str` 或 `[T]`

```rust
// 这些都会编译失败：
let s: str = "hello";           // 错：str 是 DST，大小不固定
let v: [i32] = [1, 2, 3];       // 错：[i32] 是 DST，大小不固定

// 必须通过引用或指针访问：
let s: &str = "hello";           // 正确：&str 是胖指针，大小 = 16 字节
let v: &[i32] = &[1, 2, 3];     // 正确：&[i32] 是胖指针，大小 = 16 字节
let b: Box<str> = "hello".into();// 正确：Box<str> 是胖指针，大小 = 16 字节
```

直观理解：就像你不能直接搬一个"不知道多重的货物"，但可以通过"知道在哪里，知道多重"的标签（胖指针）来操作它。

### 4. String 和 &str 的区别（以此为例深入理解）

很多人刚学 Rust 时会被 `String` 和 `&str` 搞混，从 Sized 角度看就很清楚了：

```
String（拥有堆数据的 Sized 类型）
┌────────────────────────────────────────────┐
│  栈上的 String 结构体 = 24 字节               │
│  ┌────────┬────────┬────────┐               │
│  │ ptr    │ len    │ cap    │               │
│  │(8字节) │(8字节) │(8字节) │               │
│  └────────┴────────┴────────┘               │
│      │                                      │
│      ▼ 指向堆                                │
│  ┌─────────────────────────────┐            │
│  │  h  e  l  l  o             │  (堆)       │
│  └─────────────────────────────┘            │
└────────────────────────────────────────────┘

&str（借用视图的胖指针 = 16 字节）
┌────────────────────────────────────────────┐
│  ┌──────────────┬─────────────┐             │
│  │  ptr (8字节) │ len (8字节) │             │
│  └──────────────┴─────────────┘             │
│       │                                     │
│       ▼ 可以指向堆、栈、或只读数据段            │
│  ┌─────────────────────────────┐            │
│  │  h  e  l  l  o             │            │
│  └─────────────────────────────┘            │
└────────────────────────────────────────────┘
```

- `String` 是 **Sized**，24 字节（ptr + len + cap），拥有堆上的字节缓冲区
- `&str` 是**胖指针**，16 字节（ptr + len），是对字节序列的借用视图
- `str` 本身是 **DST**，没有固定大小

---

## 完整运行示例

```rust
use std::mem::size_of;

trait Speak {
    fn speak(&self) -> &str;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> &str { "Woof" }
}

fn main() {
    // ===== Sized 类型 =====
    println!("=== Sized 类型（细指针 = 8 字节）===");
    println!("i32           = {} 字节", size_of::<i32>());
    println!("bool          = {} 字节", size_of::<bool>());
    println!("char          = {} 字节", size_of::<char>());
    println!("(i32, i32)    = {} 字节", size_of::<(i32, i32)>());
    println!("[i32; 3]      = {} 字节", size_of::<[i32; 3]>());
    println!("&i32          = {} 字节 (细指针)", size_of::<&i32>());
    println!("Box<i32>      = {} 字节 (细指针)", size_of::<Box<i32>>());
    println!();

    // ===== DST 的胖指针 =====
    println!("=== DST（胖指针 = 16 字节）===");
    println!("&str          = {} 字节 (地址+长度)", size_of::<&str>());
    println!("&[i32]        = {} 字节 (地址+长度)", size_of::<&[i32]>());
    println!("&dyn Speak    = {} 字节 (地址+vtable)", size_of::<&dyn Speak>());
    println!();

    // ===== 胖指针实际验证 =====
    println!("=== 胖指针的元数据 ===");
    let s: &str = "hello, rust";
    let arr = [1i32, 2, 3, 4, 5];
    let slice: &[i32] = &arr;
    let d: &dyn Speak = &Dog;

    println!("&str 长度元数据:  {} 字节", s.len());
    println!("&[i32] 长度元数据: {} 个元素", slice.len());
    println!("dyn Speak 调用: {}", d.speak());
    println!();

    // ===== String vs &str =====
    println!("=== String(Sized) vs &str(胖指针) ===");
    println!("String        = {} 字节 (ptr+len+cap)", size_of::<String>());
    println!("&String       = {} 字节 (细指针，指向 String 本身)", size_of::<&String>());
    println!("&str          = {} 字节 (胖指针，指向字节数据)", size_of::<&str>());
}
```

**预期输出**（64 位系统）：

```text
=== Sized 类型（细指针 = 8 字节）===
i32           = 4 字节
bool          = 1 字节
char          = 4 字节
(i32, i32)    = 8 字节
[i32; 3]      = 12 字节
&i32          = 8 字节 (细指针)
Box<i32>      = 8 字节 (细指针)

=== DST（胖指针 = 16 字节）===
&str          = 16 字节 (地址+长度)
&[i32]        = 16 字节 (地址+长度)
&dyn Speak    = 16 字节 (地址+vtable)

=== 胖指针的元数据 ===
&str 长度元数据:  11 字节
&[i32] 长度元数据: 5 个元素
dyn Speak 调用: Woof

=== String(Sized) vs &str(胖指针) ===
String        = 24 字节 (ptr+len+cap)
&String       = 8 字节 (细指针，指向 String 本身)
&str          = 16 字节 (胖指针，指向字节数据)
```

---

## 编译器错误分析

### ❌ E0277：DST 不满足 `Sized` bound

**场景**：在泛型函数或 struct 里直接用 `str` / `[T]` / `dyn Trait`，没有通过指针。

```rust
// 错误：尝试按值存储 DST
fn print_str(s: str) { // 错！
    println!("{}", s);
}
```

```text
error[E0277]: the size for values of type `str` cannot be known at
              compilation time
  --> src/main.rs:1:15
   |
1  | fn print_str(s: str) {
   |               ^^^
   | doesn't have a size known at compile-time
   |
help: function arguments must have a statically known size, borrowed types
      always have a known size
```

**修复方案**：

```rust
// 方案 1：用引用（最常见）
fn print_str(s: &str) { println!("{}", s); }

// 方案 2：用 Box（拥有型，堆分配）
fn print_str(s: Box<str>) { println!("{}", s); }

// 方案 3：用 &dyn 或 impl（对 trait 的情况）
fn speak(animal: &dyn Animal) { animal.speak(); }
fn speak(animal: &impl Animal) { animal.speak(); }
```

### ❌ 对 `size_of::<str>()` 调用失败

```rust
// 错误：str 是 DST，没有固定大小
let _ = size_of::<str>();
// error[E0277]: the size for values of type `str` cannot be known
//               at compilation time
```

**修复**：

```rust
// 用 size_of_val 来获取运行时实际大小
let s: &str = "hello";
println!("{}", size_of_val(s)); // 5（字节数）
println!("{}", size_of::<&str>()); // 16（胖指针大小）
```

---

## 实际工程场景

### 1. 函数签名优化

```rust
// 不好的写法：绑定了 String 类型
fn greet(name: &String) {
    println!("Hello, {name}");
}

// 好的写法：接受任何能变成 &str 的类型（String, &str, Arc<str>...）
fn greet(name: &str) {
    println!("Hello, {name}");
}

// 调用时都可以用：
greet("Alice");               // &str 字面量
greet(&String::from("Bob")); // &String deref 成 &str
```

这背后的原因就是 `&str` 是一个胖指针，它只关心"给我一段 UTF-8 字节序列的视图"，不关心它来自哪里。

### 2. API 设计：`&[T]` 比 `&Vec<T>` 更通用

```rust
// 不好：只能接受 Vec
fn sum(nums: &Vec<i32>) -> i32 {
    nums.iter().sum()
}

// 好：可以接受 Vec、数组、切片
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// 调用时都可以用：
sum(&vec![1, 2, 3]);    // Vec 自动 deref
sum(&[1, 2, 3]);        // 数组引用自动 coerce
sum(&arr[1..]);         // 切片
```

### 3. 存储异构类型（dyn Trait 的场景）

```rust
trait Widget {
    fn render(&self) -> String;
}

struct Button { label: String }
struct TextInput { placeholder: String }

impl Widget for Button { fn render(&self) -> String { format!("<button>{}</button>", self.label) } }
impl Widget for TextInput { fn render(&self) -> String { format!("<input placeholder='{}'>", self.placeholder) } }

// 必须通过 Box<dyn Widget>，因为不同 Widget 实现大小不同
// 如果用 Vec<Widget>（不是指针），编译器不知道每个元素占多少空间
let ui: Vec<Box<dyn Widget>> = vec![
    Box::new(Button { label: "Submit".into() }),
    Box::new(TextInput { placeholder: "Enter name...".into() }),
];

for widget in &ui {
    println!("{}", widget.render());
}
```

---

## 性能影响

### 细指针 vs 胖指针的性能差异


| 操作        | 细指针（&i32） | 胖指针（&str、&[T]、&dyn）      |
| --------- | --------- | ------------------------ |
| 指针本身大小    | 8 字节      | 16 字节（多占一个缓存行字节）         |
| 解引用       | 一次内存访问    | 同样一次内存访问（数据本身）           |
| 方法调用（dyn） | N/A       | 多一次 vtable 间接跳转（约 2~4ns） |
| 编译器优化     | 可以内联      | `dyn Trait` 无法内联         |


**何时用哪种**：

- **函数只接受一种具体类型** → 泛型 + trait bound，零成本（单态化）
- **集合需要存放多种类型** → `Box<dyn Trait>`，有 vtable 开销但很小
- **热路径多态** → 考虑枚举（enum dispatch）替代 `dyn Trait`

---

## 注意点与陷阱

### 陷阱 1：`char` 是 4 字节，不是 1 字节

Rust 的 `char` 是一个 Unicode 标量值（U+0000 到 U+10FFFF），占 4 字节。如果你从 C 背景来，这会让你惊讶。

```rust
println!("{}", size_of::<char>());  // 4，不是 1！
println!("{}", '中' as u32);         // 20013
println!("{}", '🦀' as u32);         // 129408
```

### 陷阱 2：struct 大小受字段顺序影响

```rust
struct A { x: u8, y: u64, z: u8 }  // 24 字节（填充！）
struct B { y: u64, x: u8, z: u8 }  // 16 字节（最优布局）
```

数据密集型场景（大量 struct 实例）中，字段顺序对内存使用量影响显著。可以使用 `#[repr(C)]` 固定布局，或 `cargo add cargo-sort-fields` 工具自动优化。

### 陷阱 3：`Box<T>` 本身是 Sized，但其内容可能是 DST

```rust
size_of::<Box<i32>>()     // 8 字节（细指针，因为 i32 是 Sized）
size_of::<Box<str>>()     // 16 字节（胖指针，因为 str 是 DST）
size_of::<Box<dyn Trait>>()// 16 字节（胖指针，因为 dyn Trait 是 DST）
```

`Box<T>` 本身总是 Sized（因为它就是一个指针），但当 T 是 DST 时，Box 会变成胖指针。

### 陷阱 4：`String`、`Vec<T>` 是 Sized，不是 DST

```rust
// String 是 Sized（它就是一个 24 字节的 struct）
size_of::<String>() // 24 字节

// 但 String 里存的数据（str）是 DST
// 你看不到 str 的大小，只能通过 s.len() 在运行时获取
let s = String::from("hello");
println!("{}", size_of_val(s.as_str())); // 5
```

---

## 我的理解与记忆方法

**记忆公式**：

```
大小固定 → Sized → 细指针（8字节）→ 可以按值传递
大小不定 → DST  → 胖指针（16字节）→ 必须通过引用/Box
```

**类比**：

- Sized 类型就像"快递箱"，重量标在箱子上，可以直接放进货架，随时知道占多少空间
- DST 就像"散装货物"，必须配一个"标签"（胖指针）才知道它在哪里、有多大

**判断一个类型是不是 DST 的快速判断**：

1. 它的大小在编译期能确定吗？→ 能：Sized；不能：DST
2. `&T` 是 8 字节还是 16 字节？→ 8：Sized；16：DST

**只有三类 DST，记住它们**：

```
DST 三件套：str   [T]   dyn Trait
             字符串 切片  trait 对象
```

---

## 下一步

下一篇深入讲"胖指针的元数据"——`&str` 里的长度和 `&dyn Trait` 里的 vtable 各自是什么结构，以及 Rust 是如何保证类型安全的。

- 继续阅读：[2. 对 Unsized 类型的引用（胖指针）](./2-对Unsized的引用.md)
- 回到目录：[第 12 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Reference - Dynamically Sized Types](https://doc.rust-lang.org/reference/dynamically-sized-types.html)
- 延伸阅读：[The Rustonomicon - Exotic Sizes](https://doc.rust-lang.org/nomicon/exotic-sizes.html)

