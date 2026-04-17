// 第 7 章聚焦“函数式风格”的几块核心拼图：闭包、函数指针、迭代器和组合子。
// 运行时重点观察：很多“像语法糖”的写法，本质上都依赖 trait 和所有权规则。
// 如果这一章吃透，后面读标准库 API 会轻松很多。
mod closures;
mod combinators;
mod function_pointers;
mod into_iter;
mod iterating_through_collections;
mod iterating_through_option;
mod iterators;
mod lab;

fn main() {
    println!("Chapter 07: Functional Programming Aspects");
    println!();

    // 顺序上先理解“可调用对象”，再理解“可迭代对象”，最后看组合子如何把操作串成流水线。
    closures::run();
    function_pointers::run();
    iterators::run();
    into_iter::run();
    iterating_through_collections::run();
    combinators::run();
    iterating_through_option::run();
    lab::run();
}
