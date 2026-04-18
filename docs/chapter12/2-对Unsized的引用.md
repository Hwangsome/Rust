# 2. 对 Unsized 类型的引用：胖指针的内部结构

> - **所属章节**：第 12 章 · Understanding Size in Rust
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_02_references_to_unsized_type.rs`
> - **Lab**：`chapters/chapter12/src/lab.rs`
> - **上一篇**：[1. Rust 中的类型大小（Sized 与 Unsized）](./1-类型大小与Sized.md)
> - **下一篇**：[3. Sized 与 `?Sized` 可选大小绑定](./3-Sized与?Sized.md)
> - **关键词**：fat pointer、胖指针、vtable、`&str`、`&[T]`、`&dyn Trait`、`size_of_val`

---

## 这一节解决什么问题

上一篇我们知道了 DST（`str`、`[T]`、`dyn Trait`）不能按值存放，只能通过引用访问。那么这个"引用"究竟是什么？编译器如何用一个"引用"来表示大小不固定的数据？

如果你接触过 C 语言，你知道一个指针就是 8 字节的地址值，它只告诉你"数据在哪"，但不告诉你"有多少"。为了处理变长数据，C 程序员总是要手动维护"指针 + 长度"这对信息，极易出错。

Rust 把这个配对**自动化并内建进类型系统**。对 DST 的引用不是普通的 8 字节指针，而是带有**元数据**的 **16 字节胖指针（fat pointer）**。

元数据的内容取决于 DST 的类型：
- `&str` / `&[T]`：数据地址 + **长度**（元素/字节个数）
- `&dyn Trait`：数据地址 + **vtable 指针**（方法表地址）

理解胖指针的结构，能让你看懂很多否则"莫名其妙"的 Rust 行为。

---

## 一分钟结论

- 对 Sized 类型的引用 = **细指针（8 字节）**，只含数据地址
- 对 DST 的引用 = **胖指针（16 字节）**，含数据地址 + 元数据
- `&str` 的元数据是字节长度（`usize`）
- `&[T]` 的元数据是元素个数（`usize`）
- `&dyn Trait` 的元数据是 vtable 指针（方法表地址）
- vtable 还保存了 `size`（实际大小）和 `align`（对齐）用于 drop
- `size_of::<&T>()` 总是 8 或 16，取决于 T 是不是 DST
- `size_of_val(x)` 返回 x 实际指向数据的字节大小（运行时）

---

## 与其他语言对比

| 语言 | 变长数据的引用/指针如何表示 |
|-----|----------------------|
| C | `char* ptr` + `size_t len`：程序员手动维护，极易不匹配 |
| C++ | `std::string_view`（ptr+len）；`std::span<T>`（ptr+len）；但没有统一的语言层支持 |
| Go | slice header（ptr + len + cap）；interface（ptr to data + ptr to type info）|
| Java | 所有引用都是 8 字节；对象头里存了类型信息；大小通过 JVM 内部机制知道 |
| Rust | **胖指针**作为语言一等公民；类型系统保证 ptr+metadata 永远配对正确 |

Rust 的做法比 C/C++ 安全（语言层面保证配对），比 Java 低开销（无 GC、无对象头），比 Go 更精确（fat pointer 只有两字段，不含 cap）。

---

## 核心概念与心智模型

### 胖指针的内存布局

```
对 Sized 类型的引用（细指针 = 8 字节）
┌─────────────────────────┐
│  ptr  (8 字节)           │
└─────────────────────────┘
      │
      ▼
   实际数据


对 &str 的胖指针（16 字节）
┌─────────────────────────┬──────────────────────┐
│  ptr  (8 字节)           │  len  (8 字节)        │
└─────────────────────────┴──────────────────────┘
      │                          │
      ▼                          ▼
   字节数据                    字节长度
   "hello, world"               12


对 &[T] 的胖指针（16 字节）
┌─────────────────────────┬──────────────────────┐
│  ptr  (8 字节)           │  len  (8 字节)        │
└─────────────────────────┴──────────────────────┘
      │                          │
      ▼                          ▼
   元素数组                   元素个数
   [10, 20, 30, 40]              4


对 &dyn Trait 的胖指针（16 字节）
┌─────────────────────────┬──────────────────────┐
│  ptr  (8 字节)           │  vtable_ptr (8字节)  │
└─────────────────────────┴──────────────────────┘
      │                          │
      ▼                          ▼
   实际对象（Dog 或 Cat）      vtable（方法表）
                              ┌──────────────────┐
                              │ drop: fn(*mut T) │
                              │ size: usize      │  ← Dog 或 Cat 的实际大小
                              │ align: usize     │  ← 对齐要求
                              │ method1: fn(...)│
                              │ method2: fn(...)│
                              └──────────────────┘
```

### vtable 的组织结构

每个 `impl Trait for ConcreteType` 都会生成一张 vtable，里面的内容是**函数指针的固定排列**：

```
vtable for Dog: impl Animal for Dog
┌──────────────────────────────────────────────────────┐
│ [0] drop_in_place: 指向 Dog 的 drop 实现               │
│ [1] size: 8（Dog struct 的字节大小）                   │
│ [2] align: 8（对齐要求）                               │
│ [3] speak: 指向 Dog::speak 的函数指针                  │
│ [4] eat:   指向 Dog::eat 的函数指针                    │
└──────────────────────────────────────────────────────┘

vtable for Cat: impl Animal for Cat
┌──────────────────────────────────────────────────────┐
│ [0] drop_in_place: 指向 Cat 的 drop 实现               │
│ [1] size: 16（Cat struct 的字节大小）                  │
│ [2] align: 8（对齐要求）                               │
│ [3] speak: 指向 Cat::speak 的函数指针                  │
│ [4] eat:   指向 Cat::eat 的函数指针                    │
└──────────────────────────────────────────────────────┘
```

当你调用 `animal.speak()` 时（`animal: &dyn Animal`），Rust：
1. 从胖指针里取出 vtable 地址
2. 在 vtable 里查找 `speak` 方法的函数指针（固定偏移量，编译器决定）
3. 调用那个函数指针，把 `ptr` 作为第一个参数传进去

这就是"动态分派"的完整过程，每次调用比静态分派多大约 1 次内存访问。

---

## 详细原理

### 1. `size_of` vs `size_of_val`

这两个函数是理解胖指针的关键工具：

```rust
use std::mem::{size_of, size_of_val};

// size_of::<T>() 返回"存储一个 T 所需的字节数"（编译期）
println!("size_of::<&str>()  = {}", size_of::<&str>());   // 16（胖指针本身）
println!("size_of::<&[i32]>()= {}", size_of::<&[i32]>()); // 16（胖指针本身）

// size_of_val(x) 返回"x 实际指向/持有的数据有多少字节"（运行时）
let s: &str = "hello";
println!("size_of_val(s) = {}", size_of_val(s)); // 5（字节数，不是字符数！）

let arr = [1i32, 2, 3];
let slice: &[i32] = &arr;
println!("size_of_val(slice) = {}", size_of_val(slice)); // 12（3个i32 × 4字节）

let s_utf8: &str = "中文";
println!("size_of_val(s_utf8) = {}", size_of_val(s_utf8)); // 6（UTF-8编码：每汉字3字节）
println!("s_utf8.chars().count() = {}", s_utf8.chars().count()); // 2（字符个数）
```

> 注意 `str` 的长度是**字节数**，不是字符数。这是初学者最常踩的坑之一。

### 2. `&str` vs `&[u8]` 的区别（同样是胖指针）

```rust
let s: &str = "hello";         // 胖指针：ptr + len（字节数）
let b: &[u8] = s.as_bytes();   // 胖指针：ptr + len（字节数）
// 二者的内存布局完全相同！
// 唯一区别是类型系统层面：&str 保证内容是合法 UTF-8

println!("&str  大小: {}", size_of::<&str>());   // 16
println!("&[u8] 大小: {}", size_of::<&[u8]>());  // 16
```

### 3. `&T` 和 `Box<T>` 的胖/细取决于 T

```rust
// T = i32（Sized）→ 细指针
size_of::<&i32>()      // 8
size_of::<Box<i32>>()  // 8

// T = str（DST）→ 胖指针
size_of::<&str>()      // 16
size_of::<Box<str>>()  // 16

// T = [i32]（DST）→ 胖指针
size_of::<&[i32]>()    // 16
size_of::<Box<[i32]>>()// 16

// T = dyn Trait（DST）→ 胖指针
size_of::<&dyn Trait>()    // 16
size_of::<Box<dyn Trait>>()// 16
```

规律：**包含 DST 的任何指针/智能指针都是胖指针**，大小 = 16 字节。

### 4. 通过 `unsafe` 直接观察胖指针的两个字段

```rust
use std::mem;

let s: &str = "hello, world";

// 将 &str 胖指针转为两个 usize（仅用于演示，生产代码禁止）
let fat: [usize; 2] = unsafe { mem::transmute(s) };
println!("ptr  = {:#x}", fat[0]); // 数据地址（十六进制）
println!("len  = {}",    fat[1]); // 12（字节数）

let arr = [10i32, 20, 30, 40];
let slice: &[i32] = &arr;
let fat: [usize; 2] = unsafe { mem::transmute(slice) };
println!("ptr  = {:#x}", fat[0]); // 数组首地址
println!("len  = {}",    fat[1]); // 4（元素个数）
```

---

## 完整运行示例

```rust
use std::mem::{size_of, size_of_val};

trait Animal {
    fn speak(&self) -> &'static str;
    fn name(&self) -> &'static str;
}

struct Dog { weight: f32 }
struct Cat { indoor: bool }

impl Animal for Dog {
    fn speak(&self) -> &'static str { "Woof!" }
    fn name(&self) -> &'static str { "Dog" }
}
impl Animal for Cat {
    fn speak(&self) -> &'static str { "Meow!" }
    fn name(&self) -> &'static str { "Cat" }
}

fn demonstrate_fat_pointers() {
    println!("=== 胖指针 vs 细指针 ===");
    println!("&i32          = {} bytes (细)", size_of::<&i32>());
    println!("&str          = {} bytes (胖：地址+长度)", size_of::<&str>());
    println!("&[i32]        = {} bytes (胖：地址+元素数)", size_of::<&[i32]>());
    println!("&dyn Animal   = {} bytes (胖：地址+vtable)", size_of::<&dyn Animal>());
    println!();
}

fn demonstrate_size_of_val() {
    println!("=== size_of_val: 数据本身有多大 ===");

    let ascii: &str = "hello";
    let chinese: &str = "你好";
    let emoji: &str = "🦀🦀";

    println!("\"hello\" bytes    = {}", size_of_val(ascii));   // 5
    println!("\"你好\" bytes      = {}", size_of_val(chinese)); // 6（每汉字3字节UTF-8）
    println!("\"🦀🦀\" bytes      = {}", size_of_val(emoji));   // 8（每emoji4字节UTF-8）

    println!("\"hello\" chars    = {}", ascii.chars().count());  // 5
    println!("\"你好\" chars      = {}", chinese.chars().count());// 2
    println!("\"🦀🦀\" chars      = {}", emoji.chars().count());  // 2
    println!();
}

fn demonstrate_dynamic_dispatch() {
    println!("=== 动态分派：通过 vtable 调用方法 ===");

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { weight: 25.0 }),
        Box::new(Cat { indoor: true }),
    ];

    for animal in &animals {
        // 每次调用都需要通过 vtable 间接跳转
        println!("[dyn] {} says: {}", animal.name(), animal.speak());
    }
    println!();
}

fn demonstrate_slice_metadata() {
    println!("=== 切片胖指针的元数据 ===");

    let v = vec![1i32, 2, 3, 4, 5];
    let full: &[i32] = &v;
    let head: &[i32] = &v[..3];
    let tail: &[i32] = &v[2..];

    println!("full  ptr={:p}, len={}", full.as_ptr(), full.len());
    println!("head  ptr={:p}, len={}", head.as_ptr(), head.len());
    println!("tail  ptr={:p}, len={}", tail.as_ptr(), tail.len());
    // tail 的 ptr 会比 full 大 8（2个i32 × 4字节），验证它们共享同一内存
    println!();
}

fn main() {
    demonstrate_fat_pointers();
    demonstrate_size_of_val();
    demonstrate_dynamic_dispatch();
    demonstrate_slice_metadata();
}
```

**预期输出**：

```text
=== 胖指针 vs 细指针 ===
&i32          = 8 bytes (细)
&str          = 16 bytes (胖：地址+长度)
&[i32]        = 16 bytes (胖：地址+元素数)
&dyn Animal   = 16 bytes (胖：地址+vtable)

=== size_of_val: 数据本身有多大 ===
"hello" bytes    = 5
"你好" bytes      = 6
"🦀🦀" bytes      = 8
"hello" chars    = 5
"你好" chars      = 2
"🦀🦀" chars      = 2

=== 动态分派：通过 vtable 调用方法 ===
[dyn] Dog says: Woof!
[dyn] Cat says: Meow!

=== 切片胖指针的元数据 ===
full  ptr=0x..., len=5
head  ptr=0x..., len=3
tail  ptr=0x...(full+8), len=3
```

---

## 编译器错误分析

### ❌ 对 DST 使用 `size_of`

```rust
// 错误：不能对 DST 调用 size_of
let _ = std::mem::size_of::<str>();
```

```text
error[E0277]: the size for values of type `str` cannot be known
              at compilation time
```

**修复**：

```rust
// 用 size_of_val 获取运行时大小
let s: &str = "hello";
println!("{}", std::mem::size_of_val(s)); // 正确：5
```

### ❌ 混淆字节长度和字符个数

```rust
let s = "你好世界";
println!("len    = {}", s.len());           // 12（字节！）
println!("chars  = {}", s.chars().count()); // 4（字符）
// 如果按字节索引会 panic：
// let c = &s[0..1]; // ❌ panic：'不是 UTF-8 字符边界
let c = &s[0..3]; // 正确：取第一个汉字（3字节）
```

---

## 实际工程场景

### 1. 解析二进制协议（理解切片胖指针的价值）

```rust
// 解析 TLV（Type-Length-Value）格式的字节流
fn parse_tlv(data: &[u8]) -> (&[u8], &[u8]) {
    let length = data[1] as usize;
    let value = &data[2..2 + length];
    let rest  = &data[2 + length..];
    (value, rest)
}
// 这里 value 和 rest 都是对同一 Vec<u8> 的零拷贝视图
// 胖指针保证了 ptr + len 的安全配对
```

### 2. 回调接口（dyn Trait 的工程价值）

```rust
// 不用 dyn：每种 Handler 需要独立函数，代码爆炸
fn on_click_button(handler: &ButtonHandler) { ... }
fn on_click_link(handler: &LinkHandler) { ... }

// 用 dyn：统一接口
trait ClickHandler {
    fn handle(&self, target: &str);
}

struct EventBus {
    handlers: Vec<Box<dyn ClickHandler>>,
}
impl EventBus {
    fn dispatch(&self, event: &str) {
        for h in &self.handlers {
            h.handle(event); // vtable 动态分派
        }
    }
}
```

---

## 性能影响

### vtable 分派的实际开销

- 一次 vtable 方法调用 = 1 次额外内存访问（读 vtable 指针）+ 间接跳转
- 典型开销：**2~4 纳秒**（相比直接函数调用约 0.1~0.5 纳秒）
- vtable 数据通常在 L1 缓存中（因为同类型调用热度高），实测热路径影响往往 < 1%

**何时 vtable 开销值得考虑**：

```rust
// ❌ 热路径上的 dyn（可能影响性能）
for _ in 0..10_000_000 {
    hot_handler.process(); // 如果 hot_handler: &dyn Processor，有 vtable 开销
}

// ✅ 改用泛型（编译期单态化，可以内联）
fn process_many<P: Processor>(handler: &P) {
    for _ in 0..10_000_000 {
        handler.process(); // 可以内联，无 vtable
    }
}

// ✅ 或者 enum dispatch（无堆分配，无 vtable）
enum Shape { Circle(Circle), Square(Square) }
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.area(),
            Shape::Square(s) => s.area(),
        }
    }
}
```

---

## 注意点与陷阱

### 陷阱 1：`str` 的 `len()` 返回字节数，不是字符数

```rust
let s = "naïve";
println!("{}", s.len());           // 6（n-a-ï(2字节)-v-e）
println!("{}", s.chars().count()); // 5（字符个数）
```

### 陷阱 2：切片索引必须对齐到字符边界

```rust
let s = "hello中文";
// let bad = &s[5..6]; // ❌ 运行时 panic！'中' 是3字节，不能从第6字节切
let good = &s[5..8]; // ✅ '中' 字 = s[5..8]
```

### 陷阱 3：同一个 vtable 被所有实例共享（不是每个对象都有一张）

```rust
let d1 = Dog { name: "Rex".into() };
let d2 = Dog { name: "Max".into() };
let a1: &dyn Animal = &d1;
let a2: &dyn Animal = &d2;
// a1 和 a2 的 vtable 指向同一张 vtable！只有 data ptr 不同
// vtable 是静态生成的，存在只读内存段，不随实例分配
```

---

## 我的理解与记忆方法

**记忆公式**：

```
胖指针 = 地址 + 元数据
         ↓        ↓
    在哪里      有多少/能做什么
```

**快速判断**：

- 看到 `&str`、`&[T]`、`&dyn Trait` → 胖指针 → 16 字节
- 看到 `&i32`、`&String`、`&Vec<T>` → 细指针 → 8 字节
- `len()` on `&str` → 字节数（不是字符数！）

**类比**：

- `&str` 就像图书馆的"书签 + 页数"，知道从哪开始读，读多少页
- `&dyn Trait` 就像"演员合同 + 剧本目录"，合同指向演员本人，目录告诉你他能演哪些角色

---

## 下一步

下一篇讲 `Sized` trait 和 `?Sized`——即 Rust 用什么机制把"类型必须是 Sized"这个约束编进类型系统里。

- 继续阅读：[3. Sized 与 `?Sized` 可选大小绑定](./3-Sized与?Sized.md)
- 回到目录：[第 12 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Reference - Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
- 延伸阅读：[The Rustonomicon - Representing Types](https://doc.rust-lang.org/nomicon/repr-rust.html)
