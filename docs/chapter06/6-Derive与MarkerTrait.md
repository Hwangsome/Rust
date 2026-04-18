# 6. Derive 与 Marker Trait：自动实现与能力标记

> - **所属章节**：第 6 章 · Flexibility and Abstraction with Generics and Traits
> - **Cargo package**：`chapter06`
> - **运行方式**：`cargo run -p chapter06`
> - **代码位置**：`chapters/chapter06/src/topic_06_derived_and_marker_traits.rs`
> - **Lab**：`chapters/chapter06/src/lab.rs`
> - **上一篇**：[5. Trait Object（`dyn Trait`）](./5-TraitObject.md)
> - **下一篇**：[7. Trait 中的关联类型](./7-Trait中的关联类型.md)
> - **关键词**：`#[derive]`、`Debug`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Hash`、`Default`、`Send`、`Sync`、marker trait、blanket impl

---

## 这一节解决什么问题

每次定义一个 struct，你都要手写 `Debug`（打印）、`Clone`（复制）、`PartialEq`（相等性）吗？

Rust 提供 `#[derive(...)]` 宏，让编译器自动为你的类型生成这些常用 trait 的实现，只要**成员类型都已经实现了对应 trait**。

另一个话题是 Marker Trait（标记 trait）：没有任何方法的 trait，只是在类型系统里"打标签"，表示某种能力或约束，比如 `Send`、`Sync`、`Sized`。

---

## 一分钟结论

- `#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, ...)]` 让编译器自动实现常用 trait
- **常用可派生 trait**：`Debug`（`{:?}`）、`Clone`、`Copy`、`PartialEq`、`Eq`、`Hash`、`Default`、`PartialOrd`、`Ord`
- 派生要求**所有字段**都已实现对应 trait
- **Marker trait**：没有方法，只是类型标签（`Send`、`Sync`、`Sized`、`Copy`）
- `Send`：类型可以跨线程安全转移所有权
- `Sync`：`&T` 可以跨线程安全共享（对 T 的不可变引用可以并发）
- **Blanket impl**：为所有满足条件的类型一次性实现 trait

---

## 与其他语言对比

| 语言 | 自动生成常用方法 |
|-----|-------------|
| Java | IDE 工具生成 `equals`、`hashCode`、`toString` 等，但非语言内置 |
| Python | `@dataclass` 装饰器（Python 3.7+）自动生成 `__eq__`、`__repr__` 等 |
| C# | `record` 类型（C# 9+）自动生成相等性 |
| Go | 无，需要手动实现 |
| Kotlin | `data class` 自动生成 `equals`、`hashCode`、`toString`、`copy` |
| Rust | `#[derive(...)]` 宏，编译期生成 |

---

## 详细原理

### 1. 常用可派生 trait 一览

```rust
#[derive(
    Debug,        // println!("{:?}")  和 println!("{:#?}")
    Clone,        // .clone() 深拷贝
    Copy,         // 赋值后原变量仍可用（要求所有字段都 Copy）
    PartialEq,    // == 和 !=（字段逐一比较）
    Eq,           // 完全等价关系（所有字段都 Eq 时可派生）
    Hash,         // 放进 HashSet / HashMap（要求 Eq）
    Default,      // T::default() 创建默认值
    PartialOrd,   // < > <= >= 的部分实现
    Ord,          // 全序比较（所有字段都 Ord 时可派生）
)]
struct Config {
    retries: u32,
    timeout_ms: u64,
    name: String,
}
```

### 2. 派生的前提：所有字段都满足

```rust
#[derive(Clone)]
struct Inner { data: Vec<i32> } // Vec<i32>: Clone ✅

#[derive(Clone)] // ← 可以派生，因为 Inner: Clone
struct Outer {
    inner: Inner,
    count: u32,  // u32: Clone ✅
}

// 如果某字段不能 Clone：
struct NotClone { raw: *mut i32 } // 裸指针没有 Clone

// #[derive(Clone)]  // ← 编译报错：NotClone 不能自动派生 Clone
// struct WithRaw { ptr: NotClone }
```

### 3. Marker Trait：`Send` 和 `Sync`

```rust
// 这是概念性伪代码，说明 Send/Sync 的定义
unsafe auto trait Send {}  // 自动 trait：编译器自动判断
unsafe auto trait Sync {}

// 规则：
// T: Send  → T 的所有权可以安全转移到另一个线程
// T: Sync  → &T 可以安全被多个线程同时持有

// 自动推导：如果所有字段都 Send，struct 也是 Send
// 需要手动 !Send 的情况：包含裸指针、Rc 等

use std::rc::Rc;
struct NotSend {
    data: Rc<String>, // Rc 不是 Send（引用计数不是原子操作）
}
// NotSend 自动推导为 !Send（因为 Rc 不是 Send）
// fn spawn_not_send() {
//     let ns = NotSend { data: Rc::new("hi".into()) };
//     std::thread::spawn(move || drop(ns)); // ❌ E0277
// }
```

### 4. Blanket Impl

```rust
use std::fmt::Display;

// 为所有实现了 Display 的类型自动实现 ToString
// 这是标准库里的 blanket impl：
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{self}")
    }
}

// 所以不需要手动为每种类型实现 to_string()：
println!("{}", 42.to_string());    // i32 实现了 Display → 自动有 to_string
println!("{}", 3.14.to_string());  // f64 同理
println!("{}", true.to_string());  // bool 同理
```

---

## 完整运行示例

```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Product {
    id: u32,
    name: String,
    price_cents: u32,
    in_stock: bool,
}

impl Product {
    fn new(id: u32, name: &str, price_cents: u32) -> Self {
        Product { id, name: name.into(), price_cents, in_stock: true }
    }
}

// Blanket impl：为所有 Product 提供额外方法
trait PriceTrait {
    fn price_display(&self) -> String;
    fn apply_discount(&mut self, percent: u32);
}

impl PriceTrait for Product {
    fn price_display(&self) -> String {
        format!("${:.2}", self.price_cents as f64 / 100.0)
    }
    fn apply_discount(&mut self, percent: u32) {
        self.price_cents = self.price_cents * (100 - percent) / 100;
    }
}

fn main() {
    // ===== Debug =====
    println!("=== Debug 打印 ===");
    let p1 = Product::new(1, "Rust Book", 3999);
    println!("{p1:?}");    // 一行
    println!("{p1:#?}");   // 多行（pretty print）
    println!();

    // ===== Clone =====
    println!("=== Clone ===");
    let p2 = p1.clone();
    println!("p1 == p2: {}", p1 == p2); // true（PartialEq）
    println!();

    // ===== PartialEq + Hash → HashSet =====
    println!("=== HashSet（需要 PartialEq + Hash）===");
    let mut catalog: HashSet<Product> = HashSet::new();
    catalog.insert(p1.clone());
    catalog.insert(p2.clone()); // 和 p1 相同，不会重复插入
    catalog.insert(Product::new(2, "Cargo Guide", 4999));
    println!("商品种类数: {}", catalog.len()); // 2（p1 和 p2 相同，只存一份）
    println!();

    // ===== Default =====
    println!("=== Default ===");
    let default_product: Product = Product::default();
    println!("默认商品: {default_product:?}");
    println!();

    // ===== HashMap（需要 Eq + Hash）=====
    println!("=== HashMap Key ===");
    let mut inventory: HashMap<Product, u32> = HashMap::new();
    inventory.insert(Product::new(1, "Rust Book", 3999), 100);
    inventory.insert(Product::new(2, "Cargo Guide", 4999), 50);

    for (product, qty) in &inventory {
        println!("  {} - {}: {} 件", product.name, product.price_display(), qty);
    }
    println!();

    // ===== Blanket impl（PriceTrait）=====
    println!("=== 应用折扣 ===");
    let mut sale = Product::new(3, "Sale Item", 10000); // $100
    println!("原价: {}", sale.price_display());
    sale.apply_discount(20); // 打八折
    println!("折后: {}", sale.price_display());
}
```

---

## 编译器错误分析

### ❌ 试图为包含不可 Clone 字段的 struct 派生 Clone

```rust
struct NonClone(*mut i32); // 裸指针不实现 Clone

#[derive(Clone)]
struct Wrapper {
    data: NonClone, // ❌
}
```

```text
error[E0277]: the trait bound `*mut i32: Clone` is not satisfied
  |
  | #[derive(Clone)]
  | ^^^^^^^^ the trait `Clone` is not implemented for `*mut i32`
```

### ❌ 派生 Copy 但字段不是 Copy

```rust
#[derive(Copy, Clone)]
struct OwnedData {
    s: String, // String 不是 Copy！
}
```

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
  |
  | struct OwnedData {
  |        --------- because this field does not implement `Copy`
  |     s: String,
```

---

## 实际工程场景

### 1. 领域对象标准推导

```rust
// 几乎所有业务 struct 都值得考虑这些派生
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: UserId,
    email: String,
    role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Role { Admin, User, Guest }
```

### 2. 配置类使用 Default

```rust
#[derive(Debug, Clone, Default)]
struct ServerConfig {
    host: String,        // Default: ""
    port: u16,           // Default: 0
    max_connections: u32, // Default: 0
    debug_mode: bool,    // Default: false
}

// 使用结构体更新语法
let config = ServerConfig {
    host: "localhost".into(),
    port: 8080,
    ..ServerConfig::default()
};
```

---

## 注意点与陷阱

### 陷阱 1：`#[derive(Eq)]` 需要先派生 `PartialEq`

```rust
#[derive(Eq)] // ❌ 必须同时有 PartialEq
// 正确：
#[derive(PartialEq, Eq)]
struct Id(u64);
```

### 陷阱 2：`Copy` 自动要求 `Clone`

```rust
#[derive(Copy)] // ❌ Copy 需要 Clone
// 正确：
#[derive(Copy, Clone)]
struct Point { x: f64, y: f64 }
```

### 陷阱 3：`Debug` 的输出仅用于调试，不应该被解析

```rust
let s = format!("{:?}", some_struct);
// ❌ 不要试图解析这个字符串！格式在版本间可能变化
// ✅ 如果需要序列化，用 serde 等专门工具
```

---

## 我的理解与记忆方法

**Derive 的使用原则**：

```
定义新 struct/enum 后，第一步问：
  需要打印（调试）？ → 加 Debug
  需要比较？        → 加 PartialEq（值相等），Eq（完全等价），PartialOrd/Ord（大小）
  需要放进 Set/Map？ → 加 Eq + Hash
  需要复制？        → 如果所有字段 Copy → 加 Copy + Clone；否则只加 Clone
  需要默认值？      → 加 Default
```

**Send/Sync 的记忆**：

```
Send：我可以被送去另一个线程（转移所有权）
Sync：我的引用可以在多个线程里同时存在（安全并发读）
  T: Sync  ⟺  &T: Send（等价关系）
不安全的：Rc<T>（引用计数非原子）、Cell<T>（非同步内部可变）
```

---

## 下一步

下一篇讲关联类型（Associated Types）——让 trait 更精确地描述"输入输出类型的关系"。

- 继续阅读：[7. Trait 中的关联类型](./7-Trait中的关联类型.md)
- 回到目录：[第 6 章：Generics and Traits](./README.md)
- 官方参考：[The Rust Book - Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
