//! `Option<T>`：用类型显式表达"可能没有值"。
//!
//! 在很多语言里，"空"是一个特殊的运行时值（`null` / `nil` / `None`）——
//! 经常导致空指针异常。Rust 把这件事**提到了类型层面**：
//!
//! ```ignore
//! enum Option<T> {
//!     None,
//!     Some(T),
//! }
//! ```
//!
//! 只要一个函数返回 `Option<T>`，调用方**必须在类型层面处理 None**。
//! 这从根源上消灭了"忘记判空"这类错误。
//!
//! 本节覆盖最常用的 Option 使用方式：
//! - 构造 `Some` / `None`
//! - `match` 穷尽处理
//! - `if let` 只关心 Some 分支
//! - `unwrap` / `expect`（教学阶段偶尔用，实战要克制）
//! - 链式方法 `map` / `and_then` / `unwrap_or` / `unwrap_or_else`
//! - `?` 操作符（早尝一口，细节后面章节再深入）

/// 查找：如果找到，返回下标；找不到，返回 `None`。
fn find_index(slice: &[i32], target: i32) -> Option<usize> {
    for (i, v) in slice.iter().enumerate() {
        if *v == target {
            return Some(i);
        }
    }
    None
}

/// 安全除法：除数为 0 时返回 `None`，不会 panic。
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// 演示 `?` 操作符：当函数本身返回 `Option`，可以用 `?` 快速传播 `None`。
fn first_half(slice: &[i32], target: i32) -> Option<i32> {
    let idx = find_index(slice, target)?; // 找不到就直接 `return None`
    let half = safe_div(idx as i32, 2)?;  // 安全除法失败也直接 `return None`
    Some(half)
}

pub fn run() {
    println!("== Option ==");

    println!("-- (1) match 穷尽处理 --");
    let maybe_name: Option<&str> = Some("Rust");
    match maybe_name {
        Some(name) => println!("有值: {name}"),
        None => println!("没值"),
    }
    println!();

    println!("-- (2) if let 只看 Some 分支 --");
    if let Some(value) = Some(42) {
        println!("if let 拿到: {value}");
    }
    println!();

    println!("-- (3) unwrap / expect --");
    // unwrap() 在 None 时 panic；expect() 同样 panic，但可以提供错误信息。
    let value = Some(100).unwrap();
    let described = Some(200).expect("这里不应该是 None");
    println!("unwrap()  => {value}");
    println!("expect()  => {described}");
    println!("实战建议: 只在‘不可能为 None’时用 unwrap，否则用 match / unwrap_or / ?");
    println!();

    println!("-- (4) 组合子: map / and_then / unwrap_or --");
    let n: Option<i32> = Some(10);
    let doubled = n.map(|x| x * 2); // Some(10) -> Some(20)
    let chained = n.and_then(|x| if x > 0 { Some(x - 1) } else { None }); // 连续 Option
    let fallback = None.unwrap_or(-1); // None 时回退到默认值
    println!("map: {doubled:?}, and_then: {chained:?}, unwrap_or(-1): {fallback}");
    println!();

    println!("-- (5) 查找函数：Option<usize> --");
    let v = [10, 20, 30, 40];
    println!("find 20 in {v:?} => {:?}", find_index(&v, 20));
    println!("find 99 in {v:?} => {:?}", find_index(&v, 99));
    println!();

    println!("-- (6) ? 操作符: 碰到 None 就早退 --");
    println!("first_half({v:?}, 30) = {:?}", first_half(&v, 30));
    println!("first_half({v:?}, 99) = {:?}", first_half(&v, 99));
    println!();
}
