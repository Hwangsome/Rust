//! Cargo feature flags：让一个 crate **可选**启用某些模块/依赖。
//!
//! 基本概念：
//!
//! - `[features]` 段定义 feature 名字
//! - `optional = true` 的依赖只在对应 feature 打开时才引入
//! - `#[cfg(feature = "xxx")]` 让某段代码只在 feature 打开时编译
//! - 调用方用 `features = ["stat"]` 选择要的能力
//!
//! 这一对 `math` / `consumer_of_lib` 文件最终给你一个"library + 使用者"的完整骨架示范。
//! 本节的 "consumer" 模块说明调用方长什么样——它只关心自己要打开哪些 feature。

pub fn run() {
    println!("== Cargo Features: Consumer ==");

    println!("典型 Cargo.toml 片段（调用方）:");
    println!("  [dependencies]");
    println!("  math = {{ path = \"../math\", default-features = false, features = [\"stat\"] }}");
    println!();
    println!("然后代码里直接用：");
    println!("  use math::statistics;");
    println!("  math::statistics::mean(&[1.0, 2.0, 3.0]);");
    println!();
    println!("-- 选型建议 --");
    println!("  default-features = false + 明确 features = [...]  → 只引你需要的");
    println!("  避免隐式依赖一堆 default feature，编译更快，体积更小");
    println!();
}
