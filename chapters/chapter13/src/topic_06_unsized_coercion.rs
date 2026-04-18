//! Unsized coercion（再访）：`&[T; N]` → `&[T]`、`&str`、`&dyn Trait` 的完整对照。
//!
//! 这一节和 chapter12 的"unsized coercion"主题有重叠，本章把它放在"coercion"
//! 的整体框架里再看一遍——重点是**它属于 coercion 家族里的一个种类**。

fn accept_slice(xs: &[i32]) -> i32 { xs.iter().sum() }
fn accept_str(s: &str) -> usize { s.len() }

pub fn run() {
    println!("== Unsized Coercion (revisit) ==");

    println!("-- (1) &[T; N] → &[T] --");
    let arr = [10, 20, 30, 40, 50];
    println!("  sum(&arr) = {}", accept_slice(&arr));
    println!();

    println!("-- (2) &String → &str --");
    let s = String::from("hello");
    println!("  len(&s) = {}", accept_str(&s));
    println!();

    println!("-- (3) Box<[T; N]> → Box<[T]> --");
    let boxed_arr: Box<[i32; 3]> = Box::new([1, 2, 3]);
    let boxed_slice: Box<[i32]> = boxed_arr; // 隐式转换
    println!("  boxed_slice = {boxed_slice:?}");
    println!();

    println!("-- (4) 总结 --");
    println!("  unsized coercion 是 coercion 家族的一种，触发时机和其他 coercion 一致");
    println!();
}
