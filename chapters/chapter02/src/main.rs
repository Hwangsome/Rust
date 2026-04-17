// 第 2 章开始进入 Rust 最核心也最“反直觉”的部分：
// 值的所有权、借用规则，以及引用为什么必须显式解引用。
mod borrowing_basics;
mod borrowing_in_functions;
mod dereferencing;
mod lab;
mod ownership_basics;
mod ownership_in_functions;

fn main() {
    println!("Chapter 02: Ownership and Borrowing");
    println!();

    // 先看所有权，再看借用，最后再看解引用，顺序不能反过来。
    ownership_basics::run();
    ownership_in_functions::run();
    borrowing_basics::run();
    borrowing_in_functions::run();
    dereferencing::run();
    lab::run();
}
