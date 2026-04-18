//! Trait object 向下转型（downcasting）：`dyn Any`。
//!
//! `dyn Trait` 擦除了具体类型——正常情况下你无法从它拿回原类型。
//! 但标准库提供 **`std::any::Any`** 让你在运行时做"类型判断 + 转回具体类型"。
//!
//! API：
//!
//! - `any.downcast_ref::<T>() -> Option<&T>`
//! - `any.downcast_mut::<T>() -> Option<&mut T>`
//! - `Box<dyn Any>::downcast::<T>() -> Result<Box<T>, Box<dyn Any>>`
//!
//! 注意：downcast 仅对 `dyn Any` 有效，不能对任意 `dyn Trait` 用。
//! 如果你想让自己的 trait 支持 downcast，让它 `: 'static + Any` 并暴露 `&dyn Any` 访问器。

use std::any::Any;

pub fn run() {
    println!("== Downcasting Trait Objects ==");

    let boxed_any: Box<dyn Any> = Box::new(42_i32);

    // 尝试转回 i32
    if let Some(n) = boxed_any.downcast_ref::<i32>() {
        println!("  downcast i32 => {n}");
    }

    // 尝试转成 String —— 失败
    if boxed_any.downcast_ref::<String>().is_none() {
        println!("  downcast to String fails => None");
    }

    // 拿回所有权版
    let owned: Box<dyn Any> = Box::new(String::from("rust"));
    match owned.downcast::<String>() {
        Ok(s) => println!("  owned downcast to String => {s}"),
        Err(_) => println!("  (not expected)"),
    }
    println!();
}
