// 第 9 章把前一章的内存管理工具真正落到数据结构上。
// 运行时重点观察：单向链表为什么只需要 Box，双向链表为什么马上引出 Rc 和 RefCell，
// 以及为什么一旦双向引用出现，就必须认真面对引用环问题。
mod doubly_link_list_part1;
mod doubly_link_list_part2;
mod lab;
mod reference_cycles;
mod singly_link_list_part1;
mod singly_link_list_part2;

fn main() {
    println!("Chapter 09: Implementing Typical Data Structures");
    println!();

    singly_link_list_part1::run();
    singly_link_list_part2::run();
    doubly_link_list_part1::run();
    doubly_link_list_part2::run();
    reference_cycles::run();
    lab::run();
}
