//! 控制测试如何运行：按名字筛选、`#[ignore]`、线程数、输出。
//!
//! `cargo test` 背后其实是**两个**命令行合在一起：
//!
//! ```text
//! cargo test [CARGO_FLAGS] -- [TEST_BINARY_FLAGS]
//! ```
//!
//! `--` **之前**是给 cargo 看的参数（选 package、选 profile、过滤测试名）；
//! `--` **之后**是给测试二进制看的参数（控制输出、线程数等）。
//!
//! ## 速查表
//!
//! | 目标                          | 命令                                                |
//! |-----------------------------|---------------------------------------------------|
//! | 跑当前 package 全部测试       | `cargo test -p chapter05`                         |
//! | 按名字子串筛选              | `cargo test -p chapter05 add`                     |
//! | 精确匹配某个测试             | `cargo test -p chapter05 add_returns_sum -- --exact` |
//! | 只跑标 `#[ignore]` 的测试     | `cargo test -p chapter05 -- --ignored`            |
//! | 同时跑普通 + ignored          | `cargo test -p chapter05 -- --include-ignored`    |
//! | 即便测试通过也打印其 stdout   | `cargo test -p chapter05 -- --show-output`        |
//! | 单线程串行跑（便于调试）      | `cargo test -p chapter05 -- --test-threads=1`     |
//! | 列出所有测试名但不执行         | `cargo test -p chapter05 -- --list`               |
//! | 发现第一个失败就停           | `cargo test -p chapter05 --no-fail-fast` 的反面   |

pub fn run() {
    println!("== Controlling How Tests Are Run ==");
    println!("本节里 `#[cfg(test)] mod tests` 下有 4 个测试，分别示范不同的控制方式。");
    println!();
    println!("-- 典型命令 --");
    println!("  cargo test -p chapter05");
    println!("  cargo test -p chapter05 named_filter         # 按名字子串筛选");
    println!("  cargo test -p chapter05 -- --ignored         # 只跑 #[ignore] 的");
    println!("  cargo test -p chapter05 -- --include-ignored # 全部跑");
    println!("  cargo test -p chapter05 -- --show-output     # 看到测试里的 println!");
    println!("  cargo test -p chapter05 -- --test-threads=1  # 串行便于调试");
    println!();
}

#[cfg(test)]
mod tests {
    /// 按名字筛选的典型目标：`cargo test named_filter`。
    #[test]
    fn named_filter_demo() {
        assert_eq!(2 + 2, 4);
    }

    /// 期望 panic，且 panic 信息里应包含指定子串。
    #[test]
    #[should_panic(expected = "radius should be positive")]
    fn should_panic_demo() {
        panic!("radius should be positive");
    }

    /// `#[ignore]`：默认不跑，适合慢测试、依赖网络的测试、依赖硬件的测试。
    /// 想跑这个测试，要么 `cargo test -- --ignored`，要么 `cargo test -- --include-ignored`。
    #[test]
    #[ignore]
    fn ignored_demo_slow() {
        // 假想：这个测试会花很久
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert!(true);
    }

    /// 默认跑的测试：会打印，但 `cargo test` 默认**吞掉 stdout**。
    /// 要想在"测试通过时"也看到这里的 println!，需要加 `-- --show-output`。
    #[test]
    fn prints_something_to_stdout() {
        println!("⚙️ 测试内部的输出，默认被吞掉；加 --show-output 才能看到");
        assert_eq!(1 + 1, 2);
    }
}
