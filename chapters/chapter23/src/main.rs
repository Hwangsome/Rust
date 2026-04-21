// 第 21 章对应课程 Part 3 的章节：Macros。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_macros_basics;
mod topic_02_capturing_types;
mod topic_03_repeating_patterns;
mod topic_04_question_mark_operator;

fn main() {
    println!("Chapter 22: Macros");
    println!();

    topic_01_macros_basics::run();
    topic_02_capturing_types::run();
    topic_03_repeating_patterns::run();
    topic_04_question_mark_operator::run();
    lab::run();
}
