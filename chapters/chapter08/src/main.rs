// 第 7 章聚焦“函数式风格”的几块核心拼图：闭包、函数指针、迭代器和组合子。
// 运行时重点观察：很多“像语法糖”的写法，本质上都依赖 trait 和所有权规则。
// 如果这一章吃透，后面读标准库 API 会轻松很多。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都只讲一个主题。
// 3. `lab.rs` 放在最后，是为了先看例子，再自己动手改。

mod lab;
mod topic_01_closures;
mod topic_02_function_pointers;
mod topic_03_iterators;
mod topic_04_into_iter;
mod topic_05_iterating_through_collections;
mod topic_06_combinators;
mod topic_07_iterating_through_option;

fn main() {
    println!("Chapter 08: Functional Programming Aspects");
    println!();

    // 顺序上先理解“可调用对象”，再理解“可迭代对象”，最后看组合子如何把操作串成流水线。
    topic_01_closures::run();
    topic_02_function_pointers::run();
    topic_03_iterators::run();
    topic_04_into_iter::run();
    topic_05_iterating_through_collections::run();
    topic_06_combinators::run();
    topic_07_iterating_through_option::run();
    lab::run();
}
