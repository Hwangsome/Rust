# 3. Enum：枚举与代数数据类型

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_03_enums.rs`
> - **上一篇**：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
> - **下一篇**：[4. Option](./4-Option.md)
> - **关键词**：`enum`、variant、携带数据的 enum、`match`、穷尽性、ADT

---

## 这一节解决什么问题

Java/Python 的枚举是"常量的集合"：`enum Direction { NORTH, SOUTH, EAST, WEST }`，每个变体只是一个名字。

Rust 的 enum 强大得多——**每个变体可以携带不同类型和数量的数据**：

```rust
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 具名字段（像 struct）
    Write(String),              // 一个 String
    ChangeColor(i32, i32, i32), // 三个 i32
}
```

这叫做**代数数据类型（ADT）**，是 Rust 类型系统最强大的特性之一。

---

## 一分钟结论

- Rust enum 的每个 variant 可以携带任意数据（无数据 / 元组形式 / struct 形式）
- `match` 是处理 enum 的主要工具，并且会**穷尽检查**（漏写分支会编译错误）
- `Option<T>` 和 `Result<T, E>` 本质上都是 enum
- 可以给 enum 实现方法（`impl MyEnum { ... }`）
- 不同 variant 之间类型不同，不能直接比较（除非 `#[derive(PartialEq)]`）

---

## 与其他语言对比


| 特性          | Java enum       | C enum     | Rust enum     |
| ----------- | --------------- | ---------- | ------------- |
| variant 有数据 | ❌（只有常量）         | ❌（只是整数）    | **✅**（任意类型）   |
| 穷尽检查        | ❌（switch 可以不完整） | ❌          | **✅**（编译强制）   |
| 模式匹配        | 有限（switch）      | 有限         | **完整**（match） |
| 等价概念        | 联合类型需要多个类       | 多个 #define | 一个 enum       |


---

## 详细原理

### 1. 基础 enum

```rust
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;

match dir {
    Direction::North => println!("向北"),
    Direction::South => println!("向南"),
    Direction::East  => println!("向东"),
    Direction::West  => println!("向西"),
    // 如果漏掉任何一个分支，编译会报错！
}
```

### 2. 携带数据的 enum

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                    // 半径
    Rectangle(f64, f64),            // 宽、高
    Triangle { base: f64, height: f64 }, // 底、高（具名字段）
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => base * height / 2.0,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "圆形",
            Shape::Rectangle(_, _) => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
}
```

### 3. `_` 通配符和 `..`

```rust
enum Status {
    Active,
    Inactive,
    Banned(String),
    Suspended { reason: String, duration_days: u32 },
}

fn handle(s: Status) {
    match s {
        Status::Active => println!("活跃"),
        Status::Banned(reason) => println!("封禁原因: {reason}"),
        // `..` 忽略 struct 变体的部分字段
        Status::Suspended { reason, .. } => println!("暂停: {reason}"),
        // `_` 匹配剩余所有变体
        _ => println!("其他状态"),
    }
}
```

---

## 完整运行示例

```rust
#[derive(Debug)]
enum Command {
    // 简单变体
    Quit,
    Help,
    // 携带数据
    Move { x: i32, y: i32 },
    Write(String),
    SetColor(u8, u8, u8),
}

impl Command {
    fn execute(&self) {
        match self {
            Command::Quit => println!("退出程序"),
            Command::Help => println!("帮助信息：..."),
            Command::Move { x, y } => println!("移动到 ({x}, {y})"),
            Command::Write(msg) => println!("写入: {msg}"),
            Command::SetColor(r, g, b) => println!("设置颜色: RGB({r}, {g}, {b})"),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Command::Quit) // matches! 宏的简洁写法
    }
}

fn main() {
    let commands = vec![
        Command::Move { x: 10, y: 20 },
        Command::Write("Hello, Rust!".into()),
        Command::SetColor(255, 128, 0),
        Command::Help,
        Command::Quit,
    ];

    for cmd in &commands {
        cmd.execute();
    }

    println!();
    println!("最后一条是 Quit: {}", commands.last().unwrap().is_quit());
    println!();

    // enum 的大小
    println!("Command 大小: {} bytes", std::mem::size_of::<Command>());
    // enum 的大小 = 最大 variant 的大小 + 判别字节
}
```

---

## 编译器错误分析

### ❌ E0004：match 不穷尽

```rust
enum Color { Red, Green, Blue }
let c = Color::Red;

match c {
    Color::Red => println!("红"),
    Color::Green => println!("绿"),
    // 忘写 Blue！
}
```

```text
error[E0004]: non-exhaustive patterns: `Color::Blue` not covered
  |
  | match c {
  |       ^ pattern `Color::Blue` not covered
  |
  = help: ensure that all possible cases are being handled
```

**修复**：补上遗漏的分支，或用 `_` 通配

---

## 实际工程场景

### 1. HTTP 响应状态

```rust
#[derive(Debug)]
enum HttpStatus {
    Ok(String),            // 200 + 响应体
    NotFound,              // 404
    Error { code: u16, message: String }, // 5xx
    Redirect(String),      // 301/302 + 新 URL
}

fn handle_response(status: HttpStatus) -> String {
    match status {
        HttpStatus::Ok(body) => format!("成功: {}", body.len()),
        HttpStatus::NotFound => "404 Not Found".into(),
        HttpStatus::Error { code, message } => format!("{code}: {message}"),
        HttpStatus::Redirect(url) => format!("跳转到: {url}"),
    }
}
```

### 2. AST（抽象语法树）

```rust
#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),   // 加法节点
    Mul(Box<Expr>, Box<Expr>),   // 乘法节点
    Neg(Box<Expr>),              // 取负
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e) => -eval(e),
    }
}
```

---

## 我的理解与记忆方法

**Rust enum 的精髓**：

> 每个变体不只是一个名字，而是一种"状态 + 这个状态特有的数据"的组合。用 match 强制你处理每种状态——不会遗漏任何情况。

**和 Java switch 的关键区别**：

```
Java: switch 不强制处理所有 case → 遗漏导致运行时 bug
Rust: match 强制穷尽 → 遗漏直接编译失败
```

---

## 下一步

- 继续阅读：[4. Option](./4-Option.md)
- 回到目录：[第 3 章：自定义类型](./README.md)

