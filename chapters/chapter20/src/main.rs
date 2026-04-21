// 第 18 章对应课程 Part 3 的章节：Downcasting。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_downcasting_trait_objects;
mod topic_02_downcasting_example;
mod topic_03_downcasting_for_conversion_between_trait_objects;
mod topic_04_checking_type_without_downcasting_using_typeid;

fn main() {
    println!("Chapter 19: Downcasting");
    println!();

    topic_01_downcasting_trait_objects::run();
    topic_02_downcasting_example::run();
    topic_03_downcasting_for_conversion_between_trait_objects::run();
    topic_04_checking_type_without_downcasting_using_typeid::run();
    lab::run();
}
