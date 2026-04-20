// 第 9 章把前一章的内存管理工具真正落到数据结构上。
// 运行时重点观察：单向链表为什么只需要 Box，双向链表为什么马上引出 Rc 和 RefCell，
// 以及为什么一旦双向引用出现，就必须认真面对引用环问题。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_singly_link_list_part1;
mod topic_02_singly_link_list_part2;
mod topic_03_doubly_link_list_part1;
mod topic_04_doubly_link_list_part2;
mod topic_05_reference_cycles;

fn main() {
    println!("Chapter 10: Implementing Typical Data Structures");
    println!();

    topic_01_singly_link_list_part1::run();
    topic_02_singly_link_list_part2::run();
    topic_03_doubly_link_list_part1::run();
    topic_04_doubly_link_list_part2::run();
    topic_05_reference_cycles::run();
    lab::run();
}
