# 8. 解构 Struct 参数

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_08_destructured_struct_parameters.rs`
> - **上一篇**：[7. 模式匹配上下文](./7-模式匹配上下文.md)
> - **下一篇**：[9. 引用的转换与赋值](./9-引用的转换与赋值.md)
> - **关键词**：struct 解构、函数参数解构、`..` 忽略字段、match struct

---

## 这一节解决什么问题

函数参数是 struct 时，有时候你只关心某几个字段，可以直接在参数位置解构，不需要在函数体里再 `.field`：

```rust
// 传统写法
fn area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

// 解构写法（直接在参数里拆出 width 和 height）
fn area(Rectangle { width, height, .. }: &Rectangle) -> f64 {
    width * height
}
```

---

## 详细原理

### 1. 函数参数解构

```rust
#[derive(Debug)]
struct Point { x: f64, y: f64 }
#[derive(Debug)]
struct Rectangle { width: f64, height: f64, color: String }

// 直接在参数里解构
fn distance(Point { x: x1, y: y1 }: Point, Point { x: x2, y: y2 }: Point) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// 使用 .. 忽略不关心的字段
fn area(Rectangle { width, height, .. }: &Rectangle) -> f64 {
    width * height
}
```

### 2. `match` 中解构 struct

```rust
let r = Rectangle { width: 10.0, height: 5.0, color: "red".into() };

match &r {
    Rectangle { width, height, .. } if width == height => {
        println!("正方形! 边长={width}")
    }
    Rectangle { width, height, color } => {
        println!("{color} 矩形: {width}×{height}")
    }
}
```

### 3. `let` 解构

```rust
let Point { x, y } = Point { x: 3.0, y: 4.0 };
println!("x={x}, y={y}");

// 重命名字段
let Point { x: px, y: py } = Point { x: 1.0, y: 2.0 };
println!("px={px}, py={py}");
```

---

## 完整运行示例

```rust
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_ms: u64,
    debug: bool,
}

impl Config {
    fn default() -> Self {
        Config {
            host: "localhost".into(),
            port: 8080,
            max_connections: 100,
            timeout_ms: 5000,
            debug: false,
        }
    }
}

// 只关心 host 和 port
fn format_address(Config { host, port, .. }: &Config) -> String {
    format!("{host}:{port}")
}

// 只关心连接相关配置
fn describe_limits(Config { max_connections, timeout_ms, .. }: &Config) -> String {
    format!("最大连接数: {max_connections}，超时: {timeout_ms}ms")
}

fn main() {
    let cfg = Config::default();

    println!("=== 函数参数解构 ===");
    println!("地址: {}", format_address(&cfg));
    println!("限制: {}", describe_limits(&cfg));
    println!();

    println!("=== match 解构 ===");
    let configs = vec![
        Config::default(),
        Config { port: 443, debug: true, ..Config::default() },
        Config { max_connections: 1000, ..Config::default() },
    ];

    for config in &configs {
        match config {
            Config { debug: true, host, port, .. } =>
                println!("  调试模式: {host}:{port}"),
            Config { port: 443, .. } =>
                println!("  HTTPS 配置"),
            Config { max_connections, .. } if *max_connections > 500 =>
                println!("  高负载: {max_connections} 连接"),
            Config { host, port, .. } =>
                println!("  标准: {host}:{port}"),
        }
    }
    println!();

    println!("=== let 解构 ===");
    let Config { host, port, debug, .. } = cfg;
    println!("解构出: host={host}, port={port}, debug={debug}");
}
```

---

## 注意点

### 必须用 `..` 忽略未列出的字段

```rust
let Point { x, y } = point;  // Point 只有 x 和 y，OK
let Config { host, port } = cfg; // ❌ Config 还有其他字段，必须用 ..
let Config { host, port, .. } = cfg; // ✅
```

---

## 下一步

- 继续阅读：[9. 引用的转换与赋值](./9-引用的转换与赋值.md)
- 回到目录：[第 3 章：自定义类型](./README.md)
