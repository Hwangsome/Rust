// 第 13 章对应课程 Part 3 的章节：Coercion in Rust。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_coercion_basics_deref_coercion;
mod topic_02_coercion_sites;
mod topic_03_reference_coercion;
mod topic_04_function_item_coercion;
mod topic_05_trait_objects_coercion;
mod topic_06_unsized_coercion;
mod topic_07_coercion_in_generics;
mod topic_08_transitivity_in_coercion;
mod topic_09_least_upper_bound_coercion;

fn main() {
    println!("Chapter 13: Coercion in Rust");
    println!();

    topic_01_coercion_basics_deref_coercion::run();
    topic_02_coercion_sites::run();
    topic_03_reference_coercion::run();
    topic_04_function_item_coercion::run();
    topic_05_trait_objects_coercion::run();
    topic_06_unsized_coercion::run();
    topic_07_coercion_in_generics::run();
    topic_08_transitivity_in_coercion::run();
    topic_09_least_upper_bound_coercion::run();
    lab::run();
}
