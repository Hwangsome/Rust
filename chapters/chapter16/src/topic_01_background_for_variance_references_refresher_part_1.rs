//! 方差（variance）背景 part 1：引用的回顾。
//!
//! 在讲方差前，先把"生命周期之间的子类型关系"复习清楚：
//!
//! **生命周期子类型**：如果 `'long` **活得不短于** `'short`，就有：
//!
//! ```text
//! 'long: 'short      （读作 "'long outlives 'short"）
//! ```
//!
//! 直观理解：`'static` 是最长的生命周期，它可以被当作任何更短生命周期使用。
//!
//! **关键**：生命周期子类型 + 引用的不变性/协变性/逆变性 决定了一段代码能不能编译。

pub fn run() {
    println!("== Variance Background: References Refresher (part 1) ==");

    println!("-- (1) 'static 活得最久 --");
    let literal: &'static str = "literal";
    let needs_short: &str = literal; // 'static → '_ 是合法的子类型转换
    println!("  literal = {literal}");
    println!("  needs_short (借自 'static) = {needs_short}");
    println!();

    println!("-- (2) 短生命周期不能冒充长的 --");
    // 下面如果取消注释会 E0597：
    // let out: &'static str;
    // {
    //     let s = String::from("short");
    //     out = &s;
    // }
    // 因为 s 在内层块结束时就被 drop，&s 不可能是 &'static。
    println!("  （见代码注释）反方向不合法：不能把短寿命当长寿命用");
    println!();
}
