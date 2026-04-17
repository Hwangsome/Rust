// 第 3 章开始进入“自己定义类型”和“直接使用标准库类型”的组合阶段。
// 这里的顺序是：先 struct，再给 struct 加行为，再进入 enum / Option / Result / HashMap，
// 最后补模式匹配和引用细节。
mod adding_functionality_to_structs;
mod casting_and_assignment_of_references;
mod destructured_struct_parameters;
mod enums;
mod hashmaps;
mod lab;
mod method_chaining_constraints;
mod option_type;
mod pattern_matching_contexts;
mod result_type;
mod structs_basics;

fn main() {
    println!("Chapter 03: Custom and Library Provided");
    println!();

    // 这里刻意先把“类型长什么样”讲清楚，再去讲“这些类型怎样被匹配和调用”。
    structs_basics::run();
    adding_functionality_to_structs::run();
    enums::run();
    option_type::run();
    result_type::run();
    hashmaps::run();
    pattern_matching_contexts::run();
    destructured_struct_parameters::run();
    casting_and_assignment_of_references::run();
    method_chaining_constraints::run();
    lab::run();
}
