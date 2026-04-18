//! Size in Rust：**Sized** vs **Unsized**（DST，动态大小类型）。
//!
//! Rust 的类型分两大阵营：
//!
//! | 分类        | 例子                       | 特征                                |
//! |-----------|--------------------------|-----------------------------------|
//! | **Sized**     | `i32`、`(i32, i32)`、`[i32; 3]`、`Box<T>` | 编译期大小已知，可以放在栈上    |
//! | **Unsized (DST)** | `str`、`[i32]`、`dyn Trait`     | 编译期大小不固定，**不能直接当值传**，只能通过胖指针 `&str` / `&[T]` / `&dyn Trait` 访问 |
//!
//! 关于大小的一些直觉：
//! - 引用 `&T` / `&mut T` 对 Sized T 是**一个指针宽度**（通常 8 字节）
//! - 对 Unsized T 是**胖指针**（2 个指针宽度）—— 因为要额外带长度或 vtable 指针
//! - Box<T> 同样规则：Sized 是 1 个指针，Unsized 是 2 个指针

use std::mem::size_of;

trait SomeTrait {}

struct Point {
    _x: bool,
    _y: i64,
}

pub fn run() {
    println!("== Size in Rust ==");

    println!("-- Sized types --");
    println!("  i32                = {} bytes", size_of::<i32>());
    println!("  (i32, i32)         = {} bytes", size_of::<(i32, i32)>());
    println!("  [i32; 3]           = {} bytes", size_of::<[i32; 3]>());
    println!("  Point              = {} bytes (注意字段对齐)", size_of::<Point>());
    println!();

    println!("-- 普通指针 (thin pointer) --");
    println!("  &i32               = {} bytes", size_of::<&i32>());
    println!("  &mut i32           = {} bytes", size_of::<&mut i32>());
    println!("  Box<i32>           = {} bytes", size_of::<Box<i32>>());
    println!("  fn(i32) -> i32     = {} bytes", size_of::<fn(i32) -> i32>());
    println!();

    println!("-- 胖指针 (fat pointer, 访问 unsized) --");
    println!("  &[i32]             = {} bytes (指针 + 长度)", size_of::<&[i32]>());
    println!("  &str               = {} bytes (指针 + 长度)", size_of::<&str>());
    println!("  &dyn SomeTrait     = {} bytes (指针 + vtable)", size_of::<&dyn SomeTrait>());
    println!("  Box<dyn SomeTrait> = {} bytes (指针 + vtable)", size_of::<Box<dyn SomeTrait>>());
    println!();
}
