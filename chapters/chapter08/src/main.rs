// 第 8 章把“内存管理特性”集中到一起看。
// 运行时重点观察：生命周期负责约束引用能活多久，智能指针负责表达拥有关系和可变性。
// 如果这一章理解清楚，链表、缓存、共享状态这些例子就不再只是“背写法”。
mod box_pointer_usecases;
mod box_smart_pointer;
mod concrete_lifetimes;
mod generic_lifetimes;
mod lab;
mod lifetime_elision;
mod lifetimes_in_structs;
mod rc_smart_pointer;
mod refcell;
mod refcell_example;

fn main() {
    println!("Chapter 08: Memory Management Features");
    println!();

    // 先讲引用和生命周期，再讲所有权工具箱 Box / Rc / RefCell。
    concrete_lifetimes::run();
    generic_lifetimes::run();
    lifetime_elision::run();
    lifetimes_in_structs::run();
    box_smart_pointer::run();
    box_pointer_usecases::run();
    rc_smart_pointer::run();
    refcell::run();
    refcell_example::run();
    lab::run();
}
