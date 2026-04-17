// 第 10 章讨论“如何把 struct 设计得更顺手”。
// 运行时重点观察：初始化方式、builder 模式和结构拆分，其实都在服务可维护性。
// 尤其是结构拆分，经常能直接缓解借用冲突。
mod builder_pattern;
mod initializing_struct_instances;
mod lab;
mod simplifying_structures;

fn main() {
    println!("Chapter 10: Useful Patterns for Handling Structs");
    println!();

    initializing_struct_instances::run();
    builder_pattern::run();
    simplifying_structures::run();
    lab::run();
}
