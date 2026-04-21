//! Cargo feature flags：库端的写法。
//!
//! 示范一个 `math` 库，按 feature 暴露不同模块：
//!
//! - `basic_math`：基础四则运算（default 开）
//! - `stat`：统计，依赖 basic_math
//! - `advance_math`：依赖 rust_math crate
//! - `matrix`：依赖 nalgebra crate
//!
//! 关键语法：
//!
//! ```toml
//! [features]
//! default     = ["basic_math"]
//! basic_math  = []
//! stat        = ["basic_math"]             # feature 依赖其他 feature
//! advance_math = ["dep:rust_math"]         # 启用可选依赖
//! matrix      = ["dep:nalgebra"]
//!
//! [dependencies]
//! rust_math = { version = "0.3", optional = true }
//! nalgebra  = { version = "0.33", optional = true }
//! ```
//!
//! 本 chapter 不真的引入这些依赖——把模板展示清楚即可。

/// 本地演示：仿造一个 basic_math 的 `add`。
#[inline]
fn add(a: f64, b: f64) -> f64 { a + b }

/// 演示统计模块里的 mean。
fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

pub fn run() {
    println!("== Cargo Features: Library ==");

    println!("模拟 basic_math::add(1.0, 2.0) = {}", add(1.0, 2.0));
    println!("模拟 statistics::mean(&[4.0, 5.0, 6.0]) = {}", mean(&[4.0, 5.0, 6.0]));
    println!();
    println!("-- [features] 段的写法（见文件顶部注释）--");
    println!();
}
