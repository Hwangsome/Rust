// 第 11 章把 Part 2 的最后一块收口到“错误处理”。
// 运行时重点观察：Rust 并不是只有一种错误写法，而是根据边界和需求选择不同表达。
// 从 Result / Option 到 anyhow / thiserror，本质上是在选择“我要保留多少类型信息”。
mod anyhow_example;
mod chaining_question_marks;
mod lab;
mod layered_outcomes_result_option_part1;
mod layered_outcomes_result_option_part2;
mod method_chaining_constraints;
mod multiple_error_types;
mod propagating_errors;
mod rust_error_handling_approach;
mod thiserror_example;

fn main() {
    println!("Chapter 11: Error Handling");
    println!();

    rust_error_handling_approach::run();
    propagating_errors::run();
    multiple_error_types::run();
    chaining_question_marks::run();
    layered_outcomes_result_option_part1::run();
    layered_outcomes_result_option_part2::run();
    anyhow_example::run();
    thiserror_example::run();
    method_chaining_constraints::run();
    lab::run();
}
