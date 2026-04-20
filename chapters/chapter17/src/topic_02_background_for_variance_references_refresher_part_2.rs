//! 方差背景 part 2：**一个引用用在需要更短引用的位置**是合法的。
//!
//! 具体例子：`fn accept_short<'a>(x: &'a i32)` 在调用时，Rust 可以传给它一个 `&'static i32`——
//! 因为 `'static` 更长，所以缩减成 `'a` 不会违反"借用不超寿命"的规则。
//!
//! 这就是**协变**的直觉：**让你能把"长寿引用"降级当"短寿引用"用**，反过来不行。

fn print_int<'a>(x: &'a i32) {
    println!("  borrowed = {x}");
}

pub fn run() {
    println!("== Variance Background: References Refresher (part 2) ==");

    println!("-- (1) 把 &'static 传给要求 &'a 的函数 --");
    let literal: &'static i32 = &42;
    print_int(literal); // 合法：'static 活得更长
    println!();

    println!("-- (2) 借用在不同作用域传递 --");
    let outer_value = 100;
    {
        let inner_ref = &outer_value;
        print_int(inner_ref);
    }
    // outer_value 仍然可用
    println!("  outer_value = {outer_value}");
    println!();
}
