//! 函数参数中的可变性：`fn f(mut x: T)` 到底在修改什么？
//!
//! 这一节想澄清一个非常常见的误解：
//! **参数写 `mut` 只影响函数内部的那个绑定，不会"原地改"调用方的变量。**
//!
//! 想在函数里改调用方的数据，有两个正统做法：
//! 1. 用 `&mut T`（可变借用）——调用方仍持有所有权，函数临时改一下再还
//! 2. 用返回值把新值送回调用方
//!
//! 运行时请重点对比 Copy 与非 Copy 类型在"按值传参 + 参数 mut"下的行为差异。

/// 参数写 `mut`：只给函数内部**它自己那份**绑定开了可变权限。
fn increase_copy(mut value: i32) -> i32 {
    // `value` 在这个函数里是一份独立的、可改的 i32（因为 i32 是 Copy）。
    value += 1;
    value
}

/// 对非 Copy 类型（`String`）按值传参：整个值会被 move 进函数。
/// 调用方的绑定失效；函数内部可以继续改自己这份。
fn append_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

/// 想**原地改**调用方的数据？用 `&mut T`。
/// 这里演示最简单的情况：把一个数字加上 10。
fn add_ten_in_place(value: &mut i32) {
    // `*value` 解引用后操作的是"调用方那个 i32 本体"，不是函数内部的副本。
    *value += 10;
}

pub fn run() {
    println!("-- (1) 参数 mut 只影响函数内部的那份绑定 --");
    let score = 10;
    let new_score = increase_copy(score);
    println!("调用前 score = {score}（Copy，原值仍在）");
    println!("函数返回 new_score = {new_score}");
    println!();

    println!("-- (2) 按值传 String：所有权被 move 进函数 --");
    let greeting = String::from("hello");
    let shouted = append_exclamation(greeting);
    // 下面这行会报 E0382：greeting 已经被 move 进函数了。
    // println!("greeting = {greeting}");
    println!("shouted = {shouted}");
    println!();

    println!("-- (3) 想原地改，就用 &mut T --");
    let mut counter = 0;
    add_ten_in_place(&mut counter);
    add_ten_in_place(&mut counter);
    println!("after two add_ten_in_place(&mut counter): counter = {counter}");
    println!();
}
