// 第 5 章把测试拆成四块：
// 单元测试、测试执行控制、集成测试、性能测量。
mod benchmark_basics;
mod controlling_how_tests_are_run;
mod integration_testing;
mod lab;
mod unit_testing;

fn main() {
    println!("Chapter 05: Testing");
    println!();

    // 先从最常见的单元测试开始，再进入运行控制和更外层的测试形式。
    unit_testing::run();
    controlling_how_tests_are_run::run();
    integration_testing::run();
    benchmark_basics::run();
    lab::run();
}
