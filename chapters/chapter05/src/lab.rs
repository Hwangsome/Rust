//! 第 5 章练习：测试是"边写边验证"，要亲自跑 `cargo test`。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：给你的函数补上单元测试");
    println!("  - 选一个之前章节写过的纯函数（比如 add/max3）");
    println!("  - 在同一个文件加 `#[cfg(test)] mod tests`，写 3 个 `#[test]` 函数");
    println!("  - 分别用 `assert_eq!` / `assert_ne!` / `assert!(cond, \"msg\")`");

    println!();

    println!("▷ 练习 2：按名字筛选");
    println!("  - `cargo test -p chapter05 add`  只跑名字含 add 的");
    println!("  - `cargo test -p chapter05 divide_by_zero_returns_err -- --exact`");

    println!();

    println!("▷ 练习 3：should_panic");
    println!("  - 写一个 `fn must_be_even(n: i32)` ，n 为奇数时 panic");
    println!("  - 加 `#[should_panic(expected = \"...\")]` 测试");

    println!();

    println!("▷ 练习 4：#[ignore] 和选择性运行");
    println!("  - 给一个慢测试加 `#[ignore]`");
    println!("  - 用 `cargo test -- --ignored` 单独跑它");
    println!("  - 用 `cargo test -- --include-ignored` 一并跑");

    println!();

    println!("▷ 练习 5：Result<(), E> 作为测试返回值");
    println!("  - 把一个 `fn f() -> Result<(), String>` 写进 #[test]");
    println!("  - 用 `?` 传播错误，观察 Cargo 如何把 Err 呈现为测试失败");

    println!();

    println!("▷ 练习 6：集成测试");
    println!("  - 在 chapters/chapter05/tests/ 新建一个 public_api.rs");
    println!("  - 从外部导入本 crate 某个 public 函数并断言");
    println!("  - 运行 `cargo test -p chapter05 --test public_api`");

    println!();

    println!("▷ 练习 7：粗略计时 vs criterion");
    println!("  - 用 Instant 测两种实现的耗时差");
    println!("  - 注意用 --release，用 black_box 防优化");
    println!("  - 可选：给项目加 dev-dependency criterion 再写一个基准");

    println!();

    println!("完成标准：");
    println!("  - 能流畅用 `cargo test` 的常见命令行控制测试");
    println!("  - 能说出单元 / 集成 / 文档 三类测试的位置和边界");
    println!("  - 能解释为什么自测计时必须在 --release 下、并用 black_box");
    println!();
}
