// 第 8 章把“内存管理特性”集中到一起看。
// 运行时重点观察：生命周期负责约束引用能活多久，智能指针负责表达拥有关系和可变性。
// 如果这一章理解清楚，链表、缓存、共享状态这些例子就不再只是“背写法”。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_concrete_lifetimes;
mod topic_02_generic_lifetimes;
mod topic_03_lifetime_elision;
mod topic_04_lifetimes_in_structs;
mod topic_05_box_smart_pointer;
mod topic_06_box_pointer_usecases;
mod topic_07_rc_smart_pointer;
mod topic_08_refcell;
mod topic_09_refcell_example;

fn main() {
    println!("Chapter 09: Memory Management Features");
    println!();

    // 先讲引用和生命周期，再讲所有权工具箱 Box / Rc / RefCell。
    topic_01_concrete_lifetimes::run();
    topic_02_generic_lifetimes::run();
    topic_03_lifetime_elision::run();
    topic_04_lifetimes_in_structs::run();
    topic_05_box_smart_pointer::run();
    topic_06_box_pointer_usecases::run();
    topic_07_rc_smart_pointer::run();
    topic_08_refcell::run();
    topic_09_refcell_example::run();
    lab::run();
}
