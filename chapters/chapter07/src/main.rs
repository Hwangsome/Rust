// 第 7 章聚焦“函数式风格”的几块核心拼图：闭包、函数指针、迭代器和组合子。
// 运行时重点观察：很多“像语法糖”的写法，本质上都依赖 trait 和所有权规则。
// 如果这一章吃透，后面读标准库 API 会轻松很多。
mod topic_01_closures;
mod topic_06_combinators;
mod topic_02_function_pointers;
mod topic_04_into_iter;
mod topic_05_iterating_through_collections;
mod topic_07_iterating_through_option;
mod topic_03_iterators;
mod lab;

fn main() {
    println!("Chapter 07: Functional Programming Aspects");
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
