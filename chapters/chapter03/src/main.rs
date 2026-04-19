// 第 3 章开始进入“自己定义类型”和“直接使用标准库类型”的组合阶段。
// 这里的顺序是：先 struct，再给 struct 加行为，再进入 enum / Option / Result / HashMap，
// 最后补模式匹配和引用细节。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_structs_basics;
mod topic_02_adding_functionality_to_structs;
mod topic_03_enums;
mod topic_04_option_type;
mod topic_05_result_type;
mod topic_06_hashmaps;
mod topic_07_pattern_matching_contexts;
mod topic_08_destructured_struct_parameters;
mod topic_09_casting_and_assignment_of_references;
mod topic_10_method_chaining_constraints;
mod topic_11_self_explained;

fn main() {
    println!("Chapter 03: Custom and Library Provided");
    println!();

    // 这里刻意先把“类型长什么样”讲清楚，再去讲“这些类型怎样被匹配和调用”。
    topic_01_structs_basics::run();
    topic_02_adding_functionality_to_structs::run();
    topic_03_enums::run();
    topic_04_option_type::run();
    topic_05_result_type::run();
    topic_06_hashmaps::run();
    topic_07_pattern_matching_contexts::run();
    topic_08_destructured_struct_parameters::run();
    topic_09_casting_and_assignment_of_references::run();
    topic_10_method_chaining_constraints::run();
    topic_11_self_explained::run();
    lab::run();
}
