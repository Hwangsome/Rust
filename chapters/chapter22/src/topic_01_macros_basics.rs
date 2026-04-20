//! 声明式宏（macro_rules!）基础。
//!
//! Rust 有两大类宏：
//!
//! 1. **声明式宏**（`macro_rules!`）：基于模式匹配的文本替换，本章讲这种
//! 2. **过程宏**（proc macro）：一个独立 crate 做真正的 AST 处理（`#[derive(...)]`、属性宏、函数宏）
//!
//! 基本语法：
//!
//! ```ignore
//! macro_rules! my_macro {
//!     (matcher) => { expansion };
//! }
//! ```
//!
//! 匹配器里用 `$name:type` 捕获片段——`type` 常见有 `expr` / `ident` / `ty` / `tt` / `pat`。

/// 最小宏：无参数。
macro_rules! greet {
    () => {
        println!("  hello from macro!");
    };
}

/// 带参宏：捕获一个表达式。
macro_rules! print_expr {
    ($e:expr) => {
        println!("  expr: {} = {}", stringify!($e), $e);
    };
}

pub fn run() {
    println!("== Macro Basics ==");
    greet!();
    print_expr!(2 + 3);
    print_expr!(10 * 20);
    println!();
}
