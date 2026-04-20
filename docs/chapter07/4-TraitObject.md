# 4. Trait Object：`dyn Trait` 与动态分派

> - **所属章节**：第 7 章 · Traits
> - **Cargo package**：`chapter07`
> - **运行方式**：`cargo run -p chapter07`
> - **代码位置**：`chapters/chapter07/src/topic_04_trait_objects.rs`
> - **Lab**：`chapters/chapter07/src/lab.rs`
> - **上一篇**：[3. Super Trait](./3-SuperTrait.md)
> - **下一篇**：[5. Derive 与 Marker Trait](./5-Derive与MarkerTrait.md)
> - **关键词**：`dyn Trait`、trait object、动态分派、vtable、object safety、`Box<dyn Trait>`、`&dyn Trait`

---

## 这一节解决什么问题

泛型（`fn f<T: Trait>(x: T)`）实现"编译期多态"——编译器为每种具体类型生成专属代码。但有些情况下你**不想/不能在编译期确定具体类型**：

```rust
// 需要存放"任意实现了 Shape 的类型"的集合
// Vec<Shape>？      ← 错：Shape 是 trait，不是类型，大小不定
// Vec<impl Shape>？ ← 错：编译期语法，不能用于集合元素
// Vec<Box<dyn Shape>> ← ✅ 正确！
```

当你的代码需要在**运行时**才能决定调用哪个具体类型的方法时，就需要 **Trait Object（`dyn Trait`）**。

---

## 一分钟结论

- `dyn Trait` 是"实现了 Trait 的某种类型"，具体类型在运行时决定
- Trait object 必须通过指针访问：`&dyn Trait`、`Box<dyn Trait>`、`Rc<dyn Trait>`
- 动态分派通过 **vtable（虚函数表）** 实现，每次方法调用多一次间接寻址（约 2-4ns 额外开销）
- 对比泛型：泛型是编译期单态化（零开销，可内联）；dyn 是运行期动态分派
- **Object safety（对象安全）**：并非所有 trait 都能做成 `dyn Trait`
- 不 object safe 的 trait 特征：方法返回 `Self`；方法有泛型参数；方法没有接收者

---

## 与其他语言对比

| 语言 | 运行时多态的实现方式 |
|-----|----------------|
| Java | 所有对象方法默认都是虚方法（接口引用）|
| C++ | `virtual` 关键字 + 虚函数表 |
| Go | 接口（隐式，所有方法通过接口表）|
| Python | 所有方法默认动态分派 |
| Rust | 显式写 `dyn Trait`；泛型则是编译期分派 |

**Rust 的独特之处**：泛型（编译期）和 `dyn`（运行期）是两种不同的工具，各有优缺点，你必须显式选择。其他语言往往只有一种（Java/Python 全是运行时，C++ 默认是静态的）。

---

## 核心概念与心智模型

### 静态分派（泛型）vs 动态分派（dyn）

```
静态分派（编译时决定）：
  fn draw<T: Shape>(s: &T) → 编译器生成 draw_circle、draw_rect 等
  调用时直接 CALL draw_circle 指令
  ✅ 零开销，可内联  ❌ 二进制有多份代码

动态分派（运行时决定）：
  fn draw(s: &dyn Shape) → 一份代码
  调用时：取出 vtable 指针 → 查 draw 函数地址 → CALL 那个地址
  ✅ 一份代码，可运行时多态  ❌ 每次调用多一次间接寻址
```

### `dyn Trait` 的内存结构（胖指针）

```
&dyn Shape 在内存里：
┌──────────────────────────────────────────────────┐
│  ptr (8字节)  │  vtable_ptr (8字节)               │
└──────────────────────────────────────────────────┘
       │                 │
       ▼                 ▼ vtable for (Circle, Shape)
  ┌──────────┐      ┌──────────────────────────────┐
  │  Circle  │      │ drop_fn:  fn(*mut Circle)     │
  │  data    │      │ size:     24（Circle 的大小）  │
  └──────────┘      │ align:    8                   │
                    │ area:     fn(&Circle) -> f64  │
                    │ perim:    fn(&Circle) -> f64  │
                    └──────────────────────────────┘

调用 shape.area() 的过程：
1. 从胖指针取出 vtable_ptr
2. 从 vtable 找到 area 函数指针（固定偏移量）
3. 以 ptr（数据指针）为参数调用 area 函数
```

---

## 详细原理

### 1. 最基础的 trait object 用法

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.side.powi(2) }
    fn name(&self) -> &str { "Square" }
}

// 接受任何 Shape（运行时决定）
fn describe(shape: &dyn Shape) {
    println!("{}: area = {:.2}", shape.name(), shape.area());
}

// Vec 里存放不同类型的 Shape
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 3.0 }),
    Box::new(Square { side: 4.0 }),
    Box::new(Circle { radius: 1.5 }),
];

for shape in &shapes {
    describe(shape.as_ref());
}
```

### 2. `Box<dyn Trait>` vs `&dyn Trait`

```rust
// &dyn Trait：借用，有生命周期约束
fn process_borrowed(s: &dyn Shape) {
    println!("{}", s.area());
}

// Box<dyn Trait>：拥有所有权，可以存入集合、跨函数传递
fn process_owned(s: Box<dyn Shape>) {
    println!("{}", s.area());
    // s 在函数结束时 drop
}

// Rc<dyn Trait>：共享所有权
use std::rc::Rc;
let shared: Rc<dyn Shape> = Rc::new(Circle { radius: 5.0 });
let clone = Rc::clone(&shared);
```

### 3. Object Safety（对象安全）

并非所有 trait 都能做成 `dyn Trait`。要成为 object safe，trait 需要满足：

```rust
// ❌ 不 object safe：方法返回 Self
trait NotSafe {
    fn clone_self(&self) -> Self; // ← 返回 Self，dyn 下不知道 Self 的大小
}

// ❌ 不 object safe：方法有泛型参数
trait NotSafe2 {
    fn parse<T: FromStr>(&self) -> T; // ← 泛型参数，需要单态化
}

// ✅ object safe：所有方法都有 self 接收者，无泛型参数，不返回 Self
trait Safe {
    fn name(&self) -> &str;
    fn area(&self) -> f64;
}

// ✅ 用 where Self: Sized 把不安全方法排除出 object safe 范围
trait Hybrid {
    fn name(&self) -> &str;  // ← 可以通过 dyn 调用
    fn clone_self(&self) -> Self where Self: Sized; // ← 通过 dyn 调用时被排除
}
```

---

## 完整运行示例

```rust
use std::fmt;

trait Drawable: fmt::Debug {
    fn draw(&self) -> String;
    fn area(&self) -> f64;

    fn info(&self) -> String {
        format!("{}: area={:.2}", self.draw(), self.area())
    }
}

#[derive(Debug)]
struct Circle { radius: f64 }

#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

#[derive(Debug)]
struct Text { content: String }

impl Drawable for Circle {
    fn draw(&self) -> String { format!("○ (r={})", self.radius) }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String { format!("□ ({}×{})", self.width, self.height) }
    fn area(&self) -> f64 { self.width * self.height }
}

impl Drawable for Text {
    fn draw(&self) -> String { format!("\"{}\"", self.content) }
    fn area(&self) -> f64 { 0.0 } // 文字没有面积
}

fn render_all(canvas: &[Box<dyn Drawable>]) {
    println!("=== 渲染画布 ({} 个元素) ===", canvas.len());
    for item in canvas {
        println!("  {}", item.info());
    }
}

fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let mut canvas: Vec<Box<dyn Drawable>> = Vec::new();

    // 不同类型放进同一个 Vec
    canvas.push(Box::new(Circle { radius: 5.0 }));
    canvas.push(Box::new(Rectangle { width: 10.0, height: 3.0 }));
    canvas.push(Box::new(Text { content: "Hello Rust".into() }));
    canvas.push(Box::new(Circle { radius: 2.0 }));

    render_all(&canvas);
    println!("总面积: {:.2}", total_area(&canvas));
    println!();

    // 运行时多态：根据用户输入决定创建哪种形状
    let shapes: Vec<Box<dyn Drawable>> = vec![
        create_shape("circle"),
        create_shape("rect"),
        create_shape("unknown"),
    ];

    println!("=== 动态创建 ===");
    for s in &shapes {
        println!("  {}", s.draw());
    }
}

fn create_shape(kind: &str) -> Box<dyn Drawable> {
    match kind {
        "circle" => Box::new(Circle { radius: 1.0 }),
        "rect"   => Box::new(Rectangle { width: 2.0, height: 3.0 }),
        _        => Box::new(Text { content: format!("unknown: {kind}") }),
    }
}
```

---

## 编译器错误分析

### ❌ 使用不 object safe 的 trait 创建 dyn

```rust
trait Clone {
    fn clone(&self) -> Self; // 返回 Self → 不 object safe
}

let v: Box<dyn Clone>; // ❌
```

```text
error[E0038]: the trait `Clone` cannot be made into an object
  = note: the trait cannot be made into an object because it requires the `Sized` trait
  = note: calling `dyn Clone::clone(...)` would return `Self`, but cannot be predicted
```

**修复方案**：

```rust
// 方案 A：换用泛型（编译期单态化）
fn clone_value<T: Clone>(x: &T) -> T { x.clone() }

// 方案 B：创建专门 object-safe 的 wrapper trait
trait CloneBox {
    fn clone_box(&self) -> Box<dyn CloneBox>;
}
```

### ❌ dyn Trait + dyn Trait（多 trait 组合）

```rust
let v: Box<dyn Display + Debug>; // ❌ 多个 non-auto trait
```

```text
error[E0225]: only auto traits can be used as additional traits in a trait object
```

**修复**：定义组合 trait：

```rust
trait DisplayDebug: Display + Debug {}
impl<T: Display + Debug> DisplayDebug for T {}
let v: Box<dyn DisplayDebug>; // ✅
```

---

## 实际工程场景

### 1. 事件系统

```rust
trait EventHandler {
    fn handle(&self, event: &str);
}

struct Logger;
struct Metrics;
struct Notifier { email: String }

impl EventHandler for Logger {
    fn handle(&self, event: &str) { println!("[LOG] {event}"); }
}
// ...

struct EventBus {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventBus {
    fn register(&mut self, h: impl EventHandler + 'static) {
        self.handlers.push(Box::new(h));
    }

    fn emit(&self, event: &str) {
        for h in &self.handlers {
            h.handle(event);
        }
    }
}
```

### 2. 错误处理中的 `Box<dyn Error>`

```rust
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("config.toml")?; // io::Error
    let n: i32 = file.trim().parse()?;                  // ParseIntError
    // 两种不同类型的错误都被装进 Box<dyn Error>
    Ok(())
}
```

---

## 性能影响

| 方面 | 泛型（impl Trait / `<T: Trait>`）| `dyn Trait` |
|-----|------------------------|----------|
| 分派时间 | 零（编译期内联）| ~2-4 纳秒（vtable 查找）|
| 内联 | 可以 | 不能 |
| 二进制大小 | 多份代码（每种 T）| 一份代码 |
| 集合存储 | 不能（类型不同）| 可以（Box<dyn T>）|
| 运行时多态 | 不支持 | 支持 |

**选择原则**：
- 性能热路径、不需要集合存储 → 泛型
- 需要运行时多态（插件系统、事件总线）→ `dyn Trait`
- 大多数业务代码 → 两者都合适，取决于是否需要异构集合

---

## 注意点与陷阱

### 陷阱 1：`dyn Trait` 本身是 Unsized（必须在指针后面）

```rust
// ❌ dyn Shape 不能直接按值存放
fn draw(s: dyn Shape) { ... }

// ✅ 必须通过引用或 Box
fn draw(s: &dyn Shape) { ... }
fn draw(s: Box<dyn Shape>) { ... }
```

### 陷阱 2：Object safety 检查在 trait object 使用时才触发

```rust
trait T {
    fn method(&self) -> Self; // 不 object safe
}

// 这个 impl 本身没问题
impl T for i32 { fn method(&self) -> Self { *self } }
let n: &dyn T = &42; // ← 这里才报错：T 不是 object safe
```

---

## 我的理解与记忆方法

**核心对比**：

```
泛型 + impl Trait  = 编译器的助手（帮你写多份代码）
dyn Trait          = 运行时的多态（一份代码，运行时查方法表）
```

**什么时候用 dyn**（3 个场景）：

1. 需要在**集合**里存放不同类型（`Vec<Box<dyn Trait>>`）
2. 需要从**函数返回**不同类型（根据条件决定）
3. 需要在**运行时**才知道具体类型（插件/策略模式）

---

## 下一步

下一篇讲 `#[derive]` 和 Marker Trait——自动实现常用 trait 的方法，以及如何用零大小 trait 来标记类型特征。

- 继续阅读：[6. Derive 与 Marker Trait](./6-Derive与MarkerTrait.md)
- 回到目录：[第 7 章：Traits](./README.md)
- 官方参考：[The Rust Book - Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- 延伸阅读：[Object Safety - David Tolnay](https://dtolnay.github.io/rust-quiz/)
