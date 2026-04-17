// 这一章的练习要尽量自己跑 `cargo test`，
// 因为测试主题的学习重点之一就是熟悉命令行反馈。
pub fn run() {
    println!("== Lab ==");
    println!("1. 给一个普通函数补上 #[test] 单元测试");
    println!("2. 试一次 cargo test <pattern>，观察按名称筛选测试的效果");
    println!("3. 给某个测试加上 #[ignore]，再单独运行 ignored tests");
    println!("4. 在 tests/ 目录下新增一个 smoke test");
    println!("5. 用 Instant 先写一个最小计时例子，再比较不同实现");
    println!();
}
