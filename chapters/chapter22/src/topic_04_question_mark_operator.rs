//! 宏里的 `?` 操作符（宏层面的，不是运行期的 `?`）。
//!
//! 在 `macro_rules!` 的**匹配器**里，`$(...)?` 表示"这个片段可选"——0 或 1 次。
//! 这不是 Rust 里的 `?` 错误传播，只是宏语法。
//!
//! 常见用法：可选的尾部分隔符、可选的参数组。

/// 支持可选 debug 前缀的 println 宏。
macro_rules! log_msg {
    ( $prefix:ident : $($arg:tt)* ) => {
        println!("  [{}] {}", stringify!($prefix), format!($($arg)*));
    };
    // 没有前缀的默认版本（可选前缀通过另一条规则表达）
    ( $($arg:tt)* ) => {
        println!("  {}", format!($($arg)*));
    };
}

pub fn run() {
    println!("== Macro ? Operator (optional) ==");

    log_msg!("no prefix here: {}", 42);
    log_msg!(INFO: "with prefix: x = {}", 42);
    log_msg!(WARN: "low battery: {}%", 12);
    println!();
    println!("  宏里的 `?` 是可选片段符——和运行期的 ? 是同一符号，不同语义");
    println!();
}
