// 第 6 章开始进入 Rust 的“抽象能力”阶段。
// 这里不再只看具体值怎么跑，而是开始看“同一套规则如何作用在多种类型上”。
// 运行这一章时，重点观察：泛型解决重复，trait 负责约束，trait object 负责运行时多态。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_generics;
mod topic_02_traits;
mod topic_03_trait_bounds;
mod topic_04_super_traits;
mod topic_05_trait_objects;
mod topic_06_derived_and_marker_traits;
mod topic_07_associated_types_in_traits;
mod topic_08_choosing_associated_vs_generic_type;

fn main() {
    println!("Chapter 06: Flexibility and Abstraction with Generics and Traits");
    println!();

    // 顺序上先讲“参数化类型”，再讲“共享行为”，最后补“约束”和“多态成本”。
    topic_01_generics::run();
    topic_02_traits::run();
    topic_03_trait_bounds::run();
    topic_04_super_traits::run();
    topic_05_trait_objects::run();
    topic_06_derived_and_marker_traits::run();
    topic_07_associated_types_in_traits::run();
    topic_08_choosing_associated_vs_generic_type::run();
    lab::run();
}
