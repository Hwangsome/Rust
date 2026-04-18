//! Enum：表达"这个值同时只能处于若干状态之一"。
//!
//! Rust 的 enum 比 C/Java 的 enum 强大得多——它的每个分支可以**携带自己的数据**，
//! 这让它能当"代数数据类型"（algebraic data type）用。`Option<T>`、`Result<T, E>`
//! 其实就是标准库里的 enum。
//!
//! 本节演示：
//! 1. 没有字段的简单 enum（最像其他语言的 enum）
//! 2. 每个分支携带不同数据的 enum（真正的强项）
//! 3. 在 `impl` 里给 enum 加方法
//! 4. `match` 对 enum 做**穷尽性检查**（exhaustiveness check）——漏写分支时编译失败

// 1) 简单 enum：只列分支，没有数据。
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 2) 带数据的 enum：每个分支的"形状"可以不同。
///
/// - `Quit`：没有数据
/// - `Move { x, y }`：带命名字段（就像内嵌的具名 struct）
/// - `Write(String)`：带单个 String（像元组结构体）
/// - `ChangeColor(i32, i32, i32)`：带多个字段
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 3) 给 enum 加方法：和 struct 一样写 impl。
impl TrafficLight {
    /// 把每种信号灯映射成"该做什么"的文字。
    fn describe(&self) -> &'static str {
        match self {
            TrafficLight::Red => "停止",
            TrafficLight::Yellow => "准备",
            TrafficLight::Green => "通行",
        }
    }

    /// 返回下一个信号灯——这是"状态机"风格建模的典型场景。
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

impl Message {
    /// 用一个方法统一处理"任意一种 Message"——enum 的典型价值点。
    fn handle(&self) {
        match self {
            Message::Quit => println!("[Quit] 退出"),
            Message::Move { x, y } => println!("[Move] 移动到 ({x}, {y})"),
            Message::Write(text) => println!("[Write] 写入文本: {text}"),
            Message::ChangeColor(r, g, b) => println!("[ChangeColor] RGB({r}, {g}, {b})"),
        }
    }
}

pub fn run() {
    println!("== Enums ==");

    println!("-- (1) 简单 enum + match --");
    let light = TrafficLight::Green;
    println!("当前是 {light:?}: {}", light.describe());
    let next = light.next();
    println!("下一个状态: {next:?}: {}", next.describe());
    println!();

    println!("-- (2) match 的穷尽性检查 --");
    // 如果在下面的 match 里漏掉一个分支，编译器会报：
    // error[E0004]: non-exhaustive patterns: `TrafficLight::Yellow` not covered
    let example = TrafficLight::Red;
    match example {
        TrafficLight::Red => println!("红灯"),
        TrafficLight::Yellow => println!("黄灯"),
        TrafficLight::Green => println!("绿灯"),
    }
    println!();

    println!("-- (3) 每个分支携带不同数据 --");
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages.iter() {
        msg.handle();
    }
    println!();
}
