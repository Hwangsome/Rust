// 这一节演示 Rust attribute 的最小用法。
// attribute 常被翻成“编译器指令”或“属性”，它们通过 `#[]` 语法告诉编译器怎么处理代码。
#![allow(unused_variables)]

// 这个 attribute 只作用在当前函数上。
#[allow(dead_code)]
fn square(x: i32) -> i32 {
    let temp = 42;
    x * x
}

pub fn run() {

    // 这里故意保留一个普通变量，配合文件级 attribute 演示“允许未使用变量”。
    let message = String::from("attributes use #[] syntax");
    println!("{message}");
    println!("#![allow(...)] 通常作用于整个文件或模块。");
    println!("#[allow(...)] 可以作用于函数、类型或单个项。");
    println!();
}
