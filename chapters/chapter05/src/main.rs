// 第 5 章把测试拆成四块：
// 单元测试、测试执行控制、集成测试、性能测量。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_unit_testing;
mod topic_02_controlling_how_tests_are_run;
mod topic_03_integration_testing;
mod topic_04_benchmark_basics;

fn main() {
    println!("Chapter 05: Testing");
    println!();

    // 先从最常见的单元测试开始，再进入运行控制和更外层的测试形式。
    topic_01_unit_testing::run();
    topic_02_controlling_how_tests_are_run::run();
    topic_03_integration_testing::run();
    topic_04_benchmark_basics::run();
    lab::run();
}
