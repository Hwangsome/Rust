// 这个文件演示生命周期省略规则。
// 运行时要观察：`first_word` 没写生命周期参数，但编译器仍能根据输入输出关系推断出来。
// 这也是为什么很多日常 API 看起来没有显式 `'a`，却依然和生命周期有关。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

pub fn run() {
    println!("== Lifetime Elision ==");

    let sentence = "borrow checker makes aliasing rules explicit";
    println!("first word => {}", first_word(sentence));
    println!();
}
