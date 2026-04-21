//! 不转型、只**检查类型**：`TypeId`。
//!
//! `std::any::TypeId::of::<T>()` 返回一个稳定（同一 Rust 编译）的类型标识。
//! 对比 `any.type_id()` 就能判断"是不是这个类型"——但不取回具体值。

use std::any::{Any, TypeId};

fn check(x: &dyn Any) {
    let t = x.type_id();
    if t == TypeId::of::<i32>() {
        println!("  -> i32");
    } else if t == TypeId::of::<String>() {
        println!("  -> String");
    } else if t == TypeId::of::<f64>() {
        println!("  -> f64");
    } else if t == TypeId::of::<&'static str>() {
        println!("  -> &'static str");
    } else {
        println!("  -> other TypeId = {t:?}");
    }
}

pub fn run() {
    println!("== Checking Type with TypeId ==");

    check(&1_i32);
    check(&"literal");
    check(&String::from("hello"));
    check(&3.14_f64);
    println!();
}
