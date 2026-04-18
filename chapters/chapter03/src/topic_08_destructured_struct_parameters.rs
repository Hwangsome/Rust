//! 解构 struct：在 `match`、`let`、函数参数里直接拆字段。
//!
//! struct 的字段不仅能用 `.name` 访问，还能用**模式**一次性解构成多个绑定。
//! 这在函数签名里尤其常见——让接口一眼看清楚"我要用哪些字段"。
//!
//! 本节覆盖四种典型写法：
//! 1. `let Point { x, y } = p;`——整体解构
//! 2. `let Point { x, .. } = p;`——忽略其余字段
//! 3. `fn draw(Point { x, y }: Point)`——函数参数直接解构
//! 4. `match p { Point { x: 0, y } => ..., ... }`——在分支里用具体值匹配

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(dead_code)] // email / active 只作为“未解构字段”出现，这里专门演示 `..` 的用法
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

/// 函数参数直接解构：调用方传入一个 `Point`，函数内部直接拿到 `x` 和 `y`。
fn print_coord(Point { x, y }: Point) {
    println!("函数参数解构 Point: ({x}, {y})");
}

/// 部分字段解构：只拿 `name`，其他字段用 `..` 忽略。
/// 注意 `User` 是按值传入，所以调用后调用方的变量会被 move。
fn greet(User { name, .. }: &User) {
    println!("greet: 你好，{name}");
}

pub fn run() {
    println!("== Destructured Struct Parameters ==");

    println!("-- (1) match 里把字段绑定到变量 --");
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("在 y 轴上，y = {y}"),
        Point { x, y: 0 } => println!("在 x 轴上，x = {x}"),
        Point { x, y } => println!("在 ({x}, {y})"),
    }
    println!();

    println!("-- (2) let 解构 struct --");
    let origin = Point { x: 3, y: 4 };
    let Point { x, y } = origin;
    println!("let 解构: x = {x}, y = {y}");
    println!();

    println!("-- (3) 部分解构 + .. 忽略其余 --");
    let alice = User {
        name: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    let User { name, age, .. } = &alice;
    println!("只取 name = {name}, age = {age} (其余字段用 .. 忽略)");
    println!();

    println!("-- (4) 函数参数直接解构 --");
    print_coord(Point { x: 5, y: 6 });
    greet(&alice);
    println!();

    println!("-- (5) match 结合守卫 + 字段绑定 --");
    let samples = [
        Point { x: 1, y: 1 },
        Point { x: -3, y: 2 },
        Point { x: 0, y: -1 },
    ];
    for p in &samples {
        match p {
            Point { x: 0, y: 0 } => println!("原点"),
            Point { x, y } if *x > 0 && *y > 0 => println!("第一象限: ({x}, {y})"),
            Point { x, y } => println!("其他位置: ({x}, {y})"),
        }
    }
    println!();
}
