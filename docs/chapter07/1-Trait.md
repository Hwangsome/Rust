# 1. Trait：行为抽象与接口定义

> - **所属章节**：第 7 章 · Traits
> - **Cargo package**：`chapter07`
> - **运行方式**：`cargo run -p chapter07`
> - **代码位置**：`chapters/chapter07/src/topic_01_traits.rs`
> - **Lab**：`chapters/chapter07/src/lab.rs`
> - **上一篇**：[1. 泛型：类型参数化与单态化](../chapter06/1-泛型.md)
> - **下一篇**：[2. Trait 约束](./2-Trait约束.md)
> - **关键词**：trait、`impl Trait for Type`、默认实现、`&impl Trait`、孤儿规则、行为抽象

---

## 这一节解决什么问题

你有 `Circle`、`Rectangle`、`Triangle` 三种形状，都需要实现"计算面积"和"打印描述"，但各自的计算方式不同。

没有 trait 时，你只能靠约定（方法都叫 `area()`），没有编译器保证。有 trait 时，你用 `trait Shape { fn area(&self) -> f64; }` 定义接口，编译器强制每种形状都正确实现。

**Trait 是 Rust 的核心抽象机制**，它做的事相当于：

- Java / C# 的 `interface`
- Go 的隐式 `interface`（但 Rust 是显式实现）
- C++ 的纯虚函数（但没有继承）
- Python 的 ABC（Abstract Base Class）

---

## 一分钟结论

- Trait 定义"一组方法签名"，描述类型承诺提供的能力
- `impl MyTrait for MyType { ... }` 让类型实现 trait
- Trait 方法可以有**默认实现**——实现者可以选择覆盖或继承
- 一个类型可以实现多个 trait；一个 trait 可以被多个类型实现
- `&impl Trait` / `impl Trait` 语法糖：接受"任何实现了 Trait 的类型"
- **孤儿规则**：`impl ExternalTrait for ExternalType` 是非法的（防止冲突）
- Rust **没有继承**，"共享行为"全靠 trait 默认实现 + 组合

---

## 与其他语言对比


| 概念      | Java                       | C++        | Go              | Rust                      |
| ------- | -------------------------- | ---------- | --------------- | ------------------------- |
| "接口/契约" | `interface`                | 纯虚函数 / 抽象类 | `interface`（隐式） | `trait`（显式）               |
| 实现方式    | `implements Interface`（显式） | 虚函数重写（隐式）  | 自动满足（隐式）        | `impl Trait for Type`（显式） |
| 默认实现    | Java 8+ `default method`   | 可以         | 不支持             | 支持                        |
| 继承      | 有（单继承）                     | 有（多继承）     | 无               | **无**                     |
| "多态"    | 通过继承                       | 通过虚函数      | 通过接口            | 通过 trait（静态或动态）           |
| 孤儿规则    | 无（可以在任何地方实现任何接口）           | 无          | 无               | **有**（防止冲突）               |


---

## 核心概念与心智模型

### Trait 是"行为合同"

```
Trait Shape 定义了合同：
┌─────────────────────────────────────────────────────┐
│  trait Shape {                                       │
│      fn area(&self) -> f64;       // 必须实现         │
│      fn perimeter(&self) -> f64;  // 必须实现         │
│      fn describe(&self) -> String {                 │  
│          // 默认实现：可以不实现，直接继承             │
│          format!("面积={:.2}", self.area())          │
│      }                                              │
│  }                                                  │
└─────────────────────────────────────────────────────┘

签了合同的类型（实现了 trait）：
  Circle → impl Shape for Circle { fn area / perimeter }
  Rectangle → impl Shape for Rectangle { fn area / perimeter }

可以统一对待：
  fn print_info(shape: &impl Shape) {
      println!("{}", shape.describe()); // 无论是哪种 shape，都能调用
  }
```

### Rust 没有继承，但有 Trait 组合

```
Java 的做法（继承）：
  Animal → Dog
         → Cat
  Animal 的方法被 Dog/Cat 继承，可以有同样的字段

Rust 的做法（组合）：
  trait Speak { fn speak(&self) -> &str; }
  trait Move  { fn move_to(&self, x: f64, y: f64); }
  
  struct Dog { position: (f64, f64) }
  impl Speak for Dog { fn speak(&self) -> &str { "Woof" } }
  impl Move for Dog {
      fn move_to(&self, x: f64, y: f64) { /* ... */ }
  }
  // Dog "是" Speak + Move，不是"继承"某个类
```

---

## 详细原理

### 1. 定义和实现 Trait

```rust
trait Shape {
    // 必须实现的方法（没有默认实现）
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // 有默认实现的方法（可以选择覆盖）
    fn describe(&self) -> String {
        format!("图形: 面积={:.2}, 周长={:.2}", self.area(), self.perimeter())
    }

    // 默认实现可以调用其他方法（包括未实现的方法）
    fn is_larger_than(&self, other: &impl Shape) -> bool {
        self.area() > other.area()
    }
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    // describe() 不重写，使用默认实现
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    // 覆盖默认实现，提供更具体的描述
    fn describe(&self) -> String {
        format!("矩形 {}×{}", self.width, self.height)
    }
}
```

### 2. `&impl Trait` 参数（静态分派）

```rust
fn print_shape_info(shape: &impl Shape) {
    println!("{}", shape.describe());
    println!("  面积: {:.2}", shape.area());
}
// 等价于泛型写法（编译器展开后是同一回事）：
// fn print_shape_info<T: Shape>(shape: &T) { ... }
```

### 3. 为已有类型实现自己的 Trait

```rust
trait Greet {
    fn greeting(&self) -> String;
}

// 可以为标准库里的类型实现自己定义的 trait
impl Greet for i32 {
    fn greeting(&self) -> String {
        format!("Hi, I'm the number {}!", self)
    }
}

impl Greet for String {
    fn greeting(&self) -> String {
        format!("Hello, my name is '{}'!", self)
    }
}

println!("{}", 42.greeting());
println!("{}", String::from("Rust").greeting());
```

### 4. 孤儿规则（Orphan Rule）

```rust
// 这三种都合法：
// 1. 你的 trait + 你的类型
trait MyTrait {}
struct MyStruct;
impl MyTrait for MyStruct {}

// 2. 你的 trait + 外部类型
trait Printable { fn print(&self); }
impl Printable for i32 { fn print(&self) { println!("{self}"); } }

// 3. 外部 trait + 你的类型
impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct")
    }
}

// 这种不合法：外部 trait + 外部类型
// impl std::fmt::Display for i32 { ... } // ❌ E0117
// 原因：如果两个 crate 都这样做，编译器不知道用哪个实现
```

---

## 完整运行示例

```rust
use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;

    // 默认实现：基于其他方法
    fn describe(&self) -> String {
        format!("{}: 面积={:.2}, 周长={:.2}", self.name(), self.area(), self.perimeter())
    }

    fn compare_area(&self, other: &impl Shape) -> std::cmp::Ordering {
        self.area().partial_cmp(&other.area()).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug)]
struct Circle { radius: f64 }

#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

#[derive(Debug)]
struct Triangle { a: f64, b: f64, c: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius.powi(2) }
    fn perimeter(&self) -> f64 { 2.0 * PI * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn name(&self) -> &str { "Rectangle" }
    // 覆盖默认 describe
    fn describe(&self) -> String {
        format!("矩形 {:.1}×{:.1}: 面积={:.2}", self.width, self.height, self.area())
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // 海伦公式
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 { self.a + self.b + self.c }
    fn name(&self) -> &str { "Triangle" }
}

// 接受任何实现了 Shape 的类型
fn print_info(shape: &impl Shape) {
    println!("  {}", shape.describe());
}

fn find_largest(shapes: &[&dyn Shape]) -> usize {
    let mut max_idx = 0;
    for (i, shape) in shapes.iter().enumerate() {
        if shape.area() > shapes[max_idx].area() {
            max_idx = i;
        }
    }
    max_idx
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };
    let t = Triangle { a: 3.0, b: 4.0, c: 5.0 };

    println!("=== 各形状信息 ===");
    print_info(&c);
    print_info(&r); // 使用了覆盖后的 describe
    print_info(&t);
    println!();

    println!("=== 比较面积 ===");
    use std::cmp::Ordering;
    match c.compare_area(&r) {
        Ordering::Greater => println!("  圆形面积 > 矩形面积"),
        Ordering::Less    => println!("  圆形面积 < 矩形面积"),
        Ordering::Equal   => println!("  面积相等"),
    }
    println!();

    println!("=== 找最大图形 ===");
    let shapes: Vec<&dyn Shape> = vec![&c, &r, &t];
    let largest = find_largest(&shapes);
    println!("  最大的是: {} (面积={:.2})", shapes[largest].name(), shapes[largest].area());
}
```

---

## 编译器错误分析

### ❌ E0117：孤儿规则违反

```rust
// 试图为外部类型实现外部 trait
impl std::fmt::Display for std::vec::Vec<i32> { ... }
```

```text
error[E0117]: only traits defined in the current crate can be implemented
              for types defined outside of the crate
  = note: impl doesn't use only types from inside the current crate
```

**修复方案**：用 newtype 包装：

```rust
struct MyVec(Vec<i32>);
impl std::fmt::Display for MyVec { ... } // ✅ MyVec 是我的类型
```

### ❌ E0046：没有实现 trait 的所有必要方法

```rust
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
}

struct Dog;
impl Animal for Dog {
    fn name(&self) -> &str { "Dog" }
    // 忘记实现 sound()！
}
```

```text
error[E0046]: not all trait items implemented, missing: `sound`
  |
  | fn sound(&self) -> &str;
  |    ----- `sound` from trait
```

---

## 实际工程场景

### 1. 序列化接口（类似 serde 的思路）

```rust
trait Serialize {
    fn to_json(&self) -> String;
}

trait Deserialize: Sized {
    fn from_json(json: &str) -> Option<Self>;
}

struct User { id: u64, name: String }

impl Serialize for User {
    fn to_json(&self) -> String {
        format!(r#"{{"id":{},"name":"{}"}}"#, self.id, self.name)
    }
}
```

### 2. 插件系统

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct UpperPlugin;
impl Plugin for UpperPlugin {
    fn name(&self) -> &str { "upper" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> String { input.to_uppercase() }
}

// 插件注册
fn run_plugin(plugin: &dyn Plugin, input: &str) -> String {
    println!("[{}@{}]", plugin.name(), plugin.version());
    plugin.execute(input)
}
```

---

## 注意点与陷阱

### 陷阱 1：Trait 不能有类型字段（字段是 struct 的事）

```rust
trait Animal {
    let name: String; // ❌ trait 里不能有字段
}
// 解决：用方法
trait Animal {
    fn name(&self) -> &str; // ✅ 通过方法访问
}
```

### 陷阱 2：默认实现可能导致调用循环

```rust
trait T {
    fn a(&self) { self.b(); }
    fn b(&self) { self.a(); } // ← 默认互相调用！
}
// 如果实现者不覆盖其中一个，会运行时栈溢出
```

---

## 我的理解与记忆方法

**Trait 就是行为合同**：

> 你（类型）想加入某个俱乐部（使用某套 API），就必须签一份合同（实现 trait），承诺提供合同里要求的所有方法。签了合同，编译器就知道你能被当作这个俱乐部的成员来用。

**和 Java interface 的关键区别**：

- Java：要求继承/实现关系在类定义时声明
- Rust：任何时候任何地方都可以为类型实现 trait（只要满足孤儿规则）
- 效果：可以为 `i32` 加新行为，而不需要修改 `i32` 的定义

---

## 下一步

下一篇讲 Trait 约束（bound）——如何在泛型函数里表达"T 必须实现某些 trait"，以及 `where` 子句的使用。

- 继续阅读：[3. Trait 约束](./3-Trait约束.md)
- 回到目录：[第 7 章：Traits](./README.md)
- 官方参考：[The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

