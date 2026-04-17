// 第 6 章开始进入 Rust 的“抽象能力”阶段。
// 这里不再只看具体值怎么跑，而是开始看“同一套规则如何作用在多种类型上”。
// 运行这一章时，重点观察：泛型解决重复，trait 负责约束，trait object 负责运行时多态。
mod associated_types_in_traits;
mod choosing_associated_vs_generic_type;
mod derived_and_marker_traits;
mod generics;
mod lab;
mod super_traits;
mod trait_bounds;
mod trait_objects;
mod traits;

fn main() {
    println!("Chapter 06: Flexibility and Abstraction with Generics and Traits");
    println!();

    // 顺序上先讲“参数化类型”，再讲“共享行为”，最后补“约束”和“多态成本”。
    generics::run();
    traits::run();
    trait_bounds::run();
    super_traits::run();
    trait_objects::run();
    derived_and_marker_traits::run();
    associated_types_in_traits::run();
    choosing_associated_vs_generic_type::run();
    lab::run();
}
