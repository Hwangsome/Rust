//! 第 6 章练习：以**泛型**为主（`Pair<T>`、trait bound 入门）。
//!
//! Trait 专项练习见 **第 7 章** `chapter07` 的 `lab.rs`。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：泛型 struct Pair<T>");
    println!("  - 写 `struct Pair<T> {{ first: T, second: T }}`");
    println!("  - 写 `impl<T: PartialOrd> Pair<T> {{ fn max(&self) -> &T }}`");
    println!("  - 用 `Pair::new(3, 7)` 和 `Pair::new(\"a\", \"c\")` 分别测试");
    println!();

    println!("完成标准：");
    println!("  - 能解释单态化：为什么 `Pair<i32>` 与 `Pair<&str>` 是两套代码");
    println!("  - 知道何时要给 `T` 加 `PartialOrd` 等 trait bound");
    println!("  - 准备继续：打开第 7 章学习 trait 定义、`dyn Trait`、关联类型");
    println!();
}
