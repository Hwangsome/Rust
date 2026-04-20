// 第 10 章讨论“如何把 struct 设计得更顺手”。
// 运行时重点观察：初始化方式、builder 模式和结构拆分，其实都在服务可维护性。
// 尤其是结构拆分，经常能直接缓解借用冲突。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_initializing_struct_instances;
mod topic_02_builder_pattern;
mod topic_03_simplifying_structures;

fn main() {
    println!("Chapter 11: Useful Patterns for Handling Structs");
    println!();

    topic_01_initializing_struct_instances::run();
    topic_02_builder_pattern::run();
    topic_03_simplifying_structures::run();
    lab::run();
}
