// 第 19 章对应课程 Part 3 的章节：Dropcheck。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_drop_check;
mod topic_02_drop_check_and_partial_move;
mod topic_03_drop_order_part_1;
mod topic_04_drop_order_part_2;
mod topic_05_drop_order_and_self_referencing;
mod topic_06_avoiding_drop;
mod topic_07_panic_safety;

fn main() {
    println!("Chapter 19: Dropcheck");
    println!();

    topic_01_drop_check::run();
    topic_02_drop_check_and_partial_move::run();
    topic_03_drop_order_part_1::run();
    topic_04_drop_order_part_2::run();
    topic_05_drop_order_and_self_referencing::run();
    topic_06_avoiding_drop::run();
    topic_07_panic_safety::run();
    lab::run();
}
