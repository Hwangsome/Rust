// 第 3 章开始进入“自己定义类型”和“直接使用标准库类型”的组合阶段。
// 这里的顺序是：先 struct，再给 struct 加行为，再进入 enum / Option / Result / HashMap，
// 最后补模式匹配和引用细节。
mod topic_02_adding_functionality_to_structs;
mod topic_09_casting_and_assignment_of_references;
mod topic_08_destructured_struct_parameters;
mod topic_03_enums;
mod topic_06_hashmaps;
mod lab;
mod topic_10_method_chaining_constraints;
mod topic_04_option_type;
mod topic_07_pattern_matching_contexts;
mod topic_05_result_type;
mod topic_01_structs_basics;

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
    lab::run();
}
