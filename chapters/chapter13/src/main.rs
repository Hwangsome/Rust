// 第 12 章对应课程 Part 3 的章节：Understanding Size in Rust。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_size_in_rust;
mod topic_02_references_to_unsized_type;
mod topic_03_sized_and_optionally_sized_trait;
mod topic_04_optionally_sized_trait_and_generic_parameters;
mod topic_05_unsized_coercion;
mod topic_06_zero_sized_types_never_type;
mod topic_07_zero_sized_types_unit_type;
mod topic_08_zero_sized_types_unit_structs;
mod topic_09_zero_sized_types_phantom_data;

fn main() {
    println!("Chapter 13: Understanding Size in Rust");
    println!();

    topic_01_size_in_rust::run();
    topic_02_references_to_unsized_type::run();
    topic_03_sized_and_optionally_sized_trait::run();
    topic_04_optionally_sized_trait_and_generic_parameters::run();
    topic_05_unsized_coercion::run();
    topic_06_zero_sized_types_never_type::run();
    topic_07_zero_sized_types_unit_type::run();
    topic_08_zero_sized_types_unit_structs::run();
    topic_09_zero_sized_types_phantom_data::run();
    lab::run();
}
