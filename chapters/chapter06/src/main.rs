// 第 6 章：泛型——参数化类型与单态化（Trait 已拆到第 7 章）。
// 运行这一章时重点观察：同一套逻辑如何对不同具体类型生成多份代码。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后；更多与 trait 相关的练习见第 7 章。

mod lab;
mod topic_01_generics;

fn main() {
    println!("Chapter 06: Generics");
    println!();

    topic_01_generics::run();
    lab::run();
}
