// 第 7 章：Trait 与抽象行为（从 `trait` 定义到 trait object、关联类型）。
// 第 6 章已单独覆盖泛型；本章专注「行为约束」与「多态」两条线。
// 运行顺序由 `main.rs` 决定；`lab.rs` 放在最后。

mod lab;
mod topic_01_traits;
mod topic_02_trait_bounds;
mod topic_03_super_traits;
mod topic_04_trait_objects;
mod topic_05_derived_and_marker_traits;
mod topic_06_associated_types_in_traits;
mod topic_07_choosing_associated_vs_generic_type;

fn main() {
    println!("Chapter 07: Traits");
    println!();

    topic_01_traits::run();
    topic_02_trait_bounds::run();
    topic_03_super_traits::run();
    topic_04_trait_objects::run();
    topic_05_derived_and_marker_traits::run();
    topic_06_associated_types_in_traits::run();
    topic_07_choosing_associated_vs_generic_type::run();
    lab::run();
}
