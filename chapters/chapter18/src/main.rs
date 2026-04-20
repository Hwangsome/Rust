// 第 17 章对应课程 Part 3 的章节：Trait Objects Limitations。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_orphan_rule;
mod topic_02_multiple_traits;
mod topic_03_associated_types;
mod topic_04_methods_with_generics;
mod topic_05_function_with_no_self_parameter;
mod topic_06_size_and_trait_objects;
mod topic_07_partial_object_safety;
mod topic_08_operator_overloading;
mod topic_09_sealed_traits;

fn main() {
    println!("Chapter 18: Trait Objects Limitations");
    println!();

    topic_01_orphan_rule::run();
    topic_02_multiple_traits::run();
    topic_03_associated_types::run();
    topic_04_methods_with_generics::run();
    topic_05_function_with_no_self_parameter::run();
    topic_06_size_and_trait_objects::run();
    topic_07_partial_object_safety::run();
    topic_08_operator_overloading::run();
    topic_09_sealed_traits::run();
    lab::run();
}
