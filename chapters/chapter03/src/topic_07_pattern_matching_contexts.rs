//! 模式匹配的使用场景：不仅仅是 `match`。
//!
//! 初学者容易以为"模式 = match"。实际上 Rust 的模式（pattern）出现在非常多地方：
//!
//! - `match` 表达式
//! - `if let` / `while let`
//! - `let` 绑定（`let (a, b) = pair;`）
//! - 函数参数（`fn draw((x, y): (i32, i32)) { ... }`）
//! - `for` 循环（`for (k, v) in map { ... }`）
//!
//! 本节把这些场景走一遍，再演示 `match` 的几个进阶写法：
//! - 字面量匹配 + 范围（`1..=5`）
//! - 或模式（`1 | 2 | 3`）
//! - 守卫条件（`x if x > 0 =>`）
//! - 绑定名字（`x @ 1..=5`）
//! - 忽略（`_` 单独一个；`..` 忽略其余）

/// 函数参数里的模式：直接把 tuple 解构成两个绑定。
fn print_coords((x, y): (i32, i32)) {
    println!("函数参数模式: ({x}, {y})");
}

pub fn run() {
    println!("== Pattern Matching Contexts ==");

    println!("-- (1) match 字面量 --");
    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
    println!();

    println!("-- (2) match 范围 + 或模式 + 守卫 + @ 绑定 --");
    let n = 7;
    match n {
        0 => println!("zero"),
        1 | 2 | 3 => println!("small"),       // 或模式
        digit @ 4..=9 => println!("single digit: {digit}"), // 范围 + @ 绑定
        x if x < 0 => println!("negative: {x}"),             // 守卫条件
        _ => println!("big"),
    }
    println!();

    println!("-- (3) if let 只看一个分支 --");
    let maybe = Some(5);
    if let Some(value) = maybe {
        println!("if let 命中: {value}");
    }
    // if let 也可以带 else
    if let Some(v) = None::<i32> {
        println!("不会到这里: {v}");
    } else {
        println!("if let None 时走 else 分支");
    }
    println!();

    println!("-- (4) while let 循环直到不再匹配 --");
    let mut stack = vec![1, 2, 3];
    while let Some(value) = stack.pop() {
        println!("  pop 出: {value}");
    }
    println!("stack 清空后: {stack:?}");
    println!();

    println!("-- (5) let 解构 + 函数参数解构 --");
    let (a, b) = (10, 20);
    println!("let 解构: a = {a}, b = {b}");
    print_coords((5, 8));
    println!();

    println!("-- (6) for 循环里的模式 --");
    let pairs = [(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in pairs.iter() {
        println!("  {num} => {word}");
    }
    println!();

    println!("-- (7) 忽略: _ 与 .. --");
    let triple = (1, 2, 3);
    let (first, _, _) = triple;         // `_` 忽略单个
    let (head, ..) = triple;             // `..` 忽略其余
    println!("first = {first}, head = {head}");
    println!();
}
