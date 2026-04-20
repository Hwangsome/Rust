//! 方差背景：**生命周期边界** `'a: 'b`（outlives bound）。
//!
//! 语法 `'a: 'b` 读作"`'a` 活得至少和 `'b` 一样久"。
//!
//! 常见写法：
//!
//! ```ignore
//! fn f<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str { ... }
//! // 这里 'b: 'a 要求 'b 比 'a 活得久（或一样长）
//! ```
//!
//! 这让函数能"**把长寿命引用安全降级到短寿命返回值**"。

/// 一个生命周期关系示例：`'b` 必须 outlive `'a`。
/// 返回 `&'a str` —— 最短的生命周期，调用方用起来最受限，但最容易满足。
fn choose<'a, 'b: 'a>(short: &'a str, long: &'b str, pick_long: bool) -> &'a str {
    if pick_long { long } else { short }
}

pub fn run() {
    println!("== Variance Background: Lifetime Bounds ==");

    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        // 调用方视角下：'a 是 inner 的生命周期（较短）；'b 是 outer 的（较长）
        let pick1 = choose(&inner, &outer, true);
        let pick2 = choose(&inner, &outer, false);
        println!("  pick1 = {pick1}, pick2 = {pick2}");
    }
    println!("  outer 仍然可用: {outer}");
    println!();
}
