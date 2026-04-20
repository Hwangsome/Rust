//! 避免 drop：`std::mem::ManuallyDrop<T>` 与 `Box::leak`。
//!
//! 正常情况下 Rust 强制自动 drop。但在 FFI / 内存池 / 自引用等高级场景，你可能想**阻止** drop：
//!
//! - `ManuallyDrop<T>`：包一层，drop 不会自动运行；要手动 `ManuallyDrop::drop(&mut x)`
//! - `mem::forget(x)`：直接忘记这个值，Rust 不再追踪——**慎用**，容易泄漏资源
//! - `Box::leak(b)`：把 `Box<T>` "泄漏"成 `&'static mut T`
#![allow(unsafe_code)]

use std::mem::ManuallyDrop;

struct Dropper(&'static str);
impl Drop for Dropper {
    fn drop(&mut self) { println!("  drop {}", self.0); }
}

pub fn run() {
    println!("== Avoiding Drop ==");

    println!("-- (1) 正常 drop --");
    let _a = Dropper("normal");

    println!("-- (2) ManuallyDrop 让 drop 不自动触发 --");
    let mut m = ManuallyDrop::new(Dropper("manual"));
    // 离开作用域不会 drop 它
    println!("  (not dropping m automatically)");
    // 想真的 drop 必须显式调用
    unsafe { ManuallyDrop::drop(&mut m); }
    println!();

    println!("-- (3) Box::leak → &'static mut --");
    let leaked: &'static mut Dropper = Box::leak(Box::new(Dropper("leaked")));
    println!("  leaked.0 = {}", leaked.0);
    println!("  leaked 永远不会 drop（程序结束前都活着）");
    println!();
}
