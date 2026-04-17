// 第 2 章开始进入 Rust 最核心也最“反直觉”的部分：
// 值的所有权、借用规则，以及引用为什么必须显式解引用。
mod topic_03_borrowing_basics;
mod topic_04_borrowing_in_functions;
mod topic_05_dereferencing;
mod lab;
mod topic_01_ownership_basics;
mod topic_02_ownership_in_functions;

fn main() {
    println!("Chapter 02: Ownership and Borrowing");
    println!();

    // 先看所有权，再看借用，最后再看解引用，顺序不能反过来。
    topic_01_ownership_basics::run();
    topic_02_ownership_in_functions::run();
    topic_03_borrowing_basics::run();
    topic_04_borrowing_in_functions::run();
    topic_05_dereferencing::run();
    lab::run();
}
