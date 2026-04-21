# 3. Super Trait：Trait 的前置依赖

> - **所属章节**：第 7 章 · Traits
> - **Cargo package**：`chapter07`
> - **运行方式**：`cargo run -p chapter07`
> - **代码位置**：`chapters/chapter07/src/topic_05_super_traits.rs`
> - **Lab**：`chapters/chapter07/src/lab.rs`
> - **上一篇**：[2. Trait 约束](./2-Trait约束.md)
> - **下一篇**：[4. Trait Object（`dyn Trait`）](./4-TraitObject.md)
> - **关键词**：super trait、supertrait、前置 trait、`trait A: B`、组合 trait

---

## 这一节解决什么问题

你设计了一个高层 trait `Animal`，它的 `describe` 方法需要调用 `name()`——而 `name()` 来自另一个 trait `Named`。如何保证"实现 Animal 的类型也必须实现 Named"？

答案是 Super Trait：

```rust
trait Animal: Named { // ← Animal 的"前置条件"是实现了 Named
    fn describe(&self) -> String {
        format!("I am {}", self.name()) // 可以直接调用 Named 的方法
    }
}
```

Super Trait 不是继承——Rust 没有继承。它是"**要实现 A，你必须先实现 B**"的约束，是 trait 的前置依赖。

---

## 一分钟结论

- 语法：`trait A: B + C { ... }` 表示"实现 A 必须先实现 B 和 C"
- Super trait 的方法体可以调用前置 trait 的方法（因为保证了类型实现了它）
- `impl A for T {}` 之前必须已经有 `impl B for T {}` 和 `impl C for T {}`
- 不是继承：super trait 不传递字段，不传递实现
- 常用场景：把多个基础能力组合成高层接口

---

## 与其他语言对比

| 概念 | Java | C++ | Rust |
|-----|------|-----|------|
| "A 依赖 B" | `interface A extends B` | 纯虚继承 | `trait A: B` |
| 传递字段 | 是（通过继承）| 是 | **否** |
| 传递实现 | 是（通过继承）| 是 | **否**（只约束，不共享）|
| 多重前置 | `extends B, C`（受限）| 多继承 | `trait A: B + C` |

**最大区别**：Java 的 `extends` 是继承（字段 + 实现都传递）；Rust 的 `: B` 只是**约束**（"我需要 B 的功能，但 B 的实现你自己管"）。

---

## 详细原理

### 1. 基础用法

```rust
use std::fmt::Display;

// Named trait：提供名字
trait Named {
    fn name(&self) -> &str;
}

// Describable trait：需要 Named + Display 的能力才能实现
trait Describable: Named + Display {
    fn describe(&self) -> String {
        format!("[{}] {}", self.name(), self)
        // self.name() 来自 Named
        // {} 格式化来自 Display
        // 都可以在这里直接使用，因为 super trait 保证了它们存在
    }
}

struct Dog { name: String }

impl Named for Dog {
    fn name(&self) -> &str { &self.name }
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

// 实现 Describable 时，Named 和 Display 已经实现了，所以默认方法可以直接用
impl Describable for Dog {}
```

### 2. Super Trait 的默认方法可以调用前置 trait 方法

```rust
trait Summary {
    fn summarize_author(&self) -> String;
}

trait Article: Summary {
    fn content(&self) -> String;

    // 默认实现可以调用 Summary 的方法
    fn preview(&self) -> String {
        format!(
            "作者: {}, 内容摘要: {}",
            self.summarize_author(),  // 来自 Summary
            &self.content()[..50.min(self.content().len())]
        )
    }
}
```

### 3. 多个前置 trait

```rust
use std::fmt::{Debug, Display};
use std::hash::Hash;

// 要成为 Key，必须同时满足三个条件
trait Key: Eq + Hash + Display {
    fn key_string(&self) -> String {
        self.to_string() // 利用了 Display
    }
}

// 为满足所有条件的类型自动实现 Key（blanket impl）
impl<T: Eq + Hash + Display> Key for T {}

// 现在 String、i32、u64 等都自动是 Key
use std::collections::HashMap;
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("hello".to_string(), 42);
println!("{}", "hello".to_string().key_string());
```

---

## 完整运行示例

```rust
use std::fmt::{Display, Formatter, Result};

// ===== 基础 trait =====
trait HasName {
    fn name(&self) -> &str;
}

trait HasAge {
    fn age(&self) -> u32;
}

// ===== Super trait：依赖 HasName + HasAge =====
trait Person: HasName + HasAge {
    fn introduce(&self) -> String {
        format!("我叫{}，今年{}岁", self.name(), self.age())
    }

    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

// ===== Printable：依赖 HasName + Display =====
trait Printable: HasName + Display {
    fn print_card(&self) {
        println!("┌─────────────────────────┐");
        println!("│  姓名: {:<17}│", self.name());
        println!("│  详情: {:<17}│", format!("{self}"));
        println!("└─────────────────────────┘");
    }
}

// ===== 具体类型 =====
struct Employee {
    name: String,
    age: u32,
    title: String,
}

// 必须实现所有前置 trait
impl HasName for Employee {
    fn name(&self) -> &str { &self.name }
}

impl HasAge for Employee {
    fn age(&self) -> u32 { self.age }
}

impl Display for Employee {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.title, self.age)
    }
}

// 现在可以实现高层 trait（前置条件已满足）
impl Person for Employee {}
impl Printable for Employee {}

// ===== 使用泛型约束高层 trait =====
fn onboard<P: Person + Printable>(person: &P) {
    println!("欢迎新员工!");
    person.print_card();
    println!("{}", person.introduce());
    println!("是否成年: {}", person.is_adult());
}

fn main() {
    let alice = Employee {
        name: "Alice Zhang".into(),
        age: 28,
        title: "Senior Engineer".into(),
    };

    let bob = Employee {
        name: "Bob Li".into(),
        age: 22,
        title: "Junior Developer".into(),
    };

    onboard(&alice);
    println!();
    onboard(&bob);
}
```

**预期输出**：

```text
欢迎新员工!
┌─────────────────────────┐
│  姓名: Alice Zhang       │
│  详情: Senior Engineer (28)│
└─────────────────────────┘
我叫Alice Zhang，今年28岁
是否成年: true

欢迎新员工!
┌─────────────────────────┐
│  姓名: Bob Li            │
│  详情: Junior Developer (22)│
└─────────────────────────┘
我叫Bob Li，今年22岁
是否成年: true
```

---

## 编译器错误分析

### ❌ 实现高层 trait 但缺少前置 trait 的实现

```rust
trait Named { fn name(&self) -> &str; }
trait Animal: Named { fn speak(&self) -> &str; }

struct Cat;

// 忘记实现 Named！
// impl Named for Cat { fn name(&self) -> &str { "Cat" } }

impl Animal for Cat {
    fn speak(&self) -> &str { "Meow" }
}
```

```text
error[E0277]: the trait bound `Cat: Named` is not satisfied
  |
  | impl Animal for Cat {
  | ^^^^^^^^^^^^^^^^^^^ the trait `Named` is not implemented for `Cat`
  |
  = note: `Animal` requires that `Cat: Named` is satisfied
```

---

## 实际工程场景

### 1. 数据库 ORM 模式

```rust
trait Entity {
    fn table_name() -> &'static str;
    fn id(&self) -> i64;
}

trait Serializable {
    fn to_row(&self) -> Vec<String>;
}

// 能持久化的实体：必须同时是 Entity 和 Serializable
trait Persistable: Entity + Serializable {
    fn save(&self) -> String {
        format!("INSERT INTO {} VALUES ({})",
            Self::table_name(),
            self.to_row().join(", ")
        )
    }
}
```

### 2. 标准库中的 super trait

```rust
// Eq 要求实现 PartialEq（Eq 是 PartialEq 的子集）
pub trait Eq: PartialEq { ... }

// Ord 要求实现 PartialOrd + Eq
pub trait Ord: PartialOrd + Eq { ... }

// Iterator 的辅助 trait（IntoIterator）
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 60+ 个基于 next() 的默认方法
}
```

---

## 我的理解与记忆方法

**类比**：

> Super trait 就像职位晋升要求："要当高级工程师（`trait Senior: Engineer + Mentor`），必须先是工程师（`Engineer`）且具备导师能力（`Mentor`）。"公司（编译器）不会让你绕过这个要求。

**和继承的本质区别**：

```
继承（Java）：父类给子类"留下"字段和实现
Super Trait（Rust）：只是说"你必须也懂这个"，
  实现自己管，我不帮你
```

---

## 下一步

下一篇讲 Trait Object（`dyn Trait`）——把 trait 从"编译期约束"变成"运行期动态分派"。

- 继续阅读：[5. Trait Object（`dyn Trait`）](./5-TraitObject.md)
- 回到目录：[第 7 章：Traits](./README.md)
- 官方参考：[The Rust Book - Supertrait](https://doc.rust-lang.org/reference/items/traits.html#supertraits)
