//! 引用的"种类"：`&T` / `&mut T` / `Box<T>` / `Rc<T>` / `Arc<T>` / 裸指针。
//!
//! 这一节汇总之前章节的引用/指针类型，给出一张完整速查表。
#![allow(unsafe_code)]

use std::rc::Rc;
use std::sync::Arc;

pub fn run() {
    println!("== Types of References ==");

    let value = 42;

    println!("-- (1) &T / &mut T --");
    let r: &i32 = &value;
    println!("  &i32 = {r}");
    let mut v = 10;
    let m: &mut i32 = &mut v;
    *m += 1;
    println!("  *&mut i32 after inc = {v}");

    println!("-- (2) Box<T>：堆上独占 --");
    let b: Box<i32> = Box::new(7);
    println!("  Box<i32> = {}", *b);

    println!("-- (3) Rc<T>：单线程共享 --");
    let r = Rc::new(String::from("rc"));
    let r2 = Rc::clone(&r);
    println!("  r = {r}, r2 = {r2}, strong = {}", Rc::strong_count(&r));

    println!("-- (4) Arc<T>：多线程共享 --");
    let a = Arc::new(100_i32);
    let a2 = Arc::clone(&a);
    println!("  a = {a}, a2 = {a2}, strong = {}", Arc::strong_count(&a));

    println!("-- (5) 裸指针 *const T / *mut T --");
    let p: *const i32 = &value;
    // 裸指针解引用必须用 unsafe
    let x = unsafe { *p };
    println!("  *p (unsafe) = {x}");
    println!("  裸指针不受 borrow checker 管——只有在 FFI / 特殊性能场景使用");
    println!();
}
