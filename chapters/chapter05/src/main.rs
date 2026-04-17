// 第 5 章把测试拆成四块：
// 单元测试、测试执行控制、集成测试、性能测量。
mod topic_04_benchmark_basics;
mod topic_02_controlling_how_tests_are_run;
mod topic_03_integration_testing;
mod lab;
mod topic_01_unit_testing;

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
