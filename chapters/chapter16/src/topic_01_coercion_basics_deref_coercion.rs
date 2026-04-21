//! Coercion 基础 + Deref 强转。
//!
//! **Coercion（强转）**指的是 Rust 在**特定位置**自动把一个类型转成另一个兼容类型，不需要显式 `as`。
//! 它和 "**conversion**"（`From`/`Into`/`as`）不同：coercion 是**隐式**的，由编译器在特定"coercion site"触发。
//!
//! **Deref coercion** 是最常见的一种：当一个类型实现了 `Deref<Target = U>`，
//! `&T` 会在需要 `&U` 的位置自动转成 `&U`——可能链式多层。
//!
//! 标准库里的 Deref 链：
//! ```text
//! &String      -> &str            （String: Deref<Target = str>）
//! &Vec<T>      -> &[T]            （Vec<T>: Deref<Target = [T]>）
//! &Box<T>      -> &T              （Box<T>: Deref<Target = T>）
//! &Rc<T>       -> &T
//! &Arc<T>      -> &T
//! ```

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn print_str(s: &str) {
    println!("  print_str: {s}");
}

pub fn run() {
    println!("== Coercion Basics (Deref Coercion) ==");

    println!("-- (1) &String → &str --");
    let owned = String::from("hello");
    print_str(&owned); // &String 自动 deref 成 &str
    println!();

    println!("-- (2) &Vec<T> → &[T] --");
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().sum();
    println!("  sum = {sum} (通过 &Vec<i32> 得到 &[i32])");
    println!();

    println!("-- (3) 多层 deref 链 --");
    let boxed = MyBox(String::from("rust"));
    // &MyBox<String> → &String → &str（两层 deref）
    print_str(&boxed);
    println!();

    println!("-- 记忆要点 --");
    println!("  coercion 只在‘coercion site’触发：函数参数、let 显式类型、返回值等");
    println!("  外部看不到这个转换，但它能让 API 使用方更省心");
    println!();
}
