//! 单元测试：和被测代码放在同一个文件里，近距离验证"最小行为单元"。
//!
//! Rust 的测试系统内置在 `cargo` 里，不需要任何外部框架。核心工具如下：
//!
//! - `#[cfg(test)]`：**只在 `cargo test` 下编译**的代码块——测试不会影响 `cargo build`
//! - `#[test]`：声明"这是一个测试函数"
//! - `assert!` / `assert_eq!` / `assert_ne!`：三种基本断言
//! - `#[should_panic]`：反向断言——测试 **应该** panic
//! - `Result<(), E>` 作为测试返回值：可以用 `?` 传播错误
//!
//! 本节把这些工具**同时呈现**在一个 `tests` 子模块里，让你一次看到标准结构。
//!
//! 运行方式：`cargo test -p chapter05 --lib`（本章不是 lib 但道理一致）或
//! `cargo test -p chapter05`。

/// 一个被测函数：两数相加。
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

/// 另一个被测函数：除法，0 做除数时返回 `Err`。
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("divide by zero"))
    } else {
        Ok(a / b)
    }
}

/// 明确会 panic 的函数——用于演示 `#[should_panic]`。
pub fn assert_positive(x: i32) {
    if x <= 0 {
        panic!("value must be positive, got {x}");
    }
}

pub fn run() {
    println!("== Unit Testing ==");
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 3) = {:?}", divide(10, 3));
    println!("divide(10, 0) = {:?}", divide(10, 0));
    println!();
    println!("单元测试写在同文件 `#[cfg(test)] mod tests` 里；运行 `cargo test -p chapter05`。");
    println!();
}

// ---------- 测试代码 ----------
//
// `#[cfg(test)]` 的作用是：**只在运行 `cargo test` 时**把这段代码编译进去；
// 平时 `cargo build` / `cargo run` 完全看不到它，也不会被算进二进制体积。
#[cfg(test)]
mod tests {
    // 把父模块的 item 引进来，避免每处都写 `super::`。
    use super::*;

    /// 最常见的断言：`assert_eq!(actual, expected)`。
    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    /// 不等断言：`assert_ne!`。
    #[test]
    fn add_not_equal_edge_case() {
        assert_ne!(add(1, 1), 3);
    }

    /// 任意布尔断言：`assert!(condition)`。
    #[test]
    fn add_is_commutative_sample() {
        let a = 7;
        let b = 11;
        assert!(add(a, b) == add(b, a));
    }

    /// 返回 `Result<(), E>`：可以在测试里用 `?` 传播错误。
    /// 测试失败时直接把错误信息作为 Err 返回，比 panic 更可读。
    #[test]
    fn divide_ok() -> Result<(), String> {
        let result = divide(10, 2)?;
        assert_eq!(result, 5);
        Ok(())
    }

    /// 断言一定返回 `Err`。
    #[test]
    fn divide_by_zero_returns_err() {
        let result = divide(1, 0);
        assert!(result.is_err(), "1/0 应该返回 Err，实际 = {:?}", result);
    }

    /// `#[should_panic]`：期望函数 panic，如果没 panic 测试就算失败。
    /// `expected = "..."` 会检查 panic 消息里是否含有指定子串。
    #[test]
    #[should_panic(expected = "value must be positive")]
    fn assert_positive_panics_on_zero() {
        assert_positive(0);
    }
}
