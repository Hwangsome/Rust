//! 集成测试：放在 `tests/` 目录，从**外部**消费 crate。
//!
//! Rust 的测试有 3 种形态：
//!
//! | 类型       | 放在哪                      | 以什么身份编译                  |
//! |----------|--------------------------|-------------------------|
//! | 单元测试   | 与被测代码同文件，`#[cfg(test)] mod tests` | 与被测代码同 crate（可见私有 item）|
//! | 集成测试   | `tests/<name>.rs`        | 每个文件是一个**独立的 crate**        |
//! | 文档测试   | `///` 注释里的代码块        | 独立 mini crate 编译 + 运行          |
//!
//! 集成测试的特点：
//! - **只能**访问 crate 的**公开 API**（跟真实使用方视角一致）
//! - 每个 `tests/*.rs` 会编译成独立的测试二进制
//! - 所以共享辅助代码要放在 `tests/common/mod.rs`（用 `mod common;` 引入）
//!
//! 本章的 `tests/smoke.rs` 就是一个最小集成测试——运行 `cargo test -p chapter05`
//! 会同时跑单元测试与集成测试。

pub fn run() {
    println!("== Integration Testing ==");
    println!("集成测试位置：chapters/chapter05/tests/smoke.rs");
    println!("每个 tests/*.rs 是独立 crate；只能消费 public API。");
    println!();
    println!("-- 三类测试对比 --");
    println!("  单元测试：同文件 #[cfg(test)] mod tests，可访问私有 item");
    println!("  集成测试：tests/*.rs，独立 crate，仅可访问 public API");
    println!("  文档测试：/// 注释里的代码块（```rust ... ```），每块都会编译并运行");
    println!();
    println!("-- 共享辅助代码 --");
    println!("  约定：tests/common/mod.rs  （用 `mod common;` 从测试文件里引入）");
    println!("  不要使用 tests/common.rs 作为模块，否则会被当作独立测试二进制");
    println!();
}
