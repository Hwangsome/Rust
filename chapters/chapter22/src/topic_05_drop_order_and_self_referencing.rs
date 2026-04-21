//! Drop 顺序与**自引用**结构：为什么自引用在 Rust 里很危险。
//!
//! 自引用（一个 struct 的字段持有指向同 struct 另一字段的引用）在 Rust 里不好写——
//! 因为 drop 顺序固定了，编译器必须保证先 drop 的字段不能被后 drop 的字段引用。

struct Part(&'static str);

impl Drop for Part {
    fn drop(&mut self) {
        println!("  drop part {}", self.0);
    }
}

pub fn run() {
    println!("== Drop Order & Self-Referencing ==");

    // 简单示意：两个独立字段，按声明顺序 drop
    struct TwoParts {
        a: Part,
        b: Part,
    }

    let _tp = TwoParts { a: Part("a"), b: Part("b") };
    println!("  leaving scope; expect a → b");

    println!();
    println!("关于自引用：");
    println!("  struct T<'a> {{ data: Vec<i32>, view: &'a Vec<i32> }} ← 这种'两个字段互相关联'");
    println!("  在 Rust 里几乎不可能安全构造（会 E0597 / E0515）；");
    println!("  真正要做自引用要么用 Pin + unsafe，要么用 `ouroboros` 等 crate");
    println!();
}
