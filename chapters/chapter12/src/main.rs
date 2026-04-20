// 第 11 章把 Part 2 的最后一块收口到“错误处理”。
// 运行时重点观察：Rust 并不是只有一种错误写法，而是根据边界和需求选择不同表达。
// 从 Result / Option 到 anyhow / thiserror，本质上是在选择“我要保留多少类型信息”。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_rust_error_handling_approach;
mod topic_02_propagating_errors;
mod topic_03_multiple_error_types;
mod topic_04_chaining_question_marks;
mod topic_05_layered_outcomes_result_option_part1;
mod topic_06_layered_outcomes_result_option_part2;
mod topic_07_anyhow_example;
mod topic_08_thiserror_example;
mod topic_09_method_chaining_constraints;

fn main() {
    println!("Chapter 12: Error Handling");
    println!();

    topic_01_rust_error_handling_approach::run();
    topic_02_propagating_errors::run();
    topic_03_multiple_error_types::run();
    topic_04_chaining_question_marks::run();
    topic_05_layered_outcomes_result_option_part1::run();
    topic_06_layered_outcomes_result_option_part2::run();
    topic_07_anyhow_example::run();
    topic_08_thiserror_example::run();
    topic_09_method_chaining_constraints::run();
    lab::run();
}
