//! 多 trait 组合：`&dyn (TraitA + TraitB)` 做不到。
//!
//! Rust 的 trait object 基本只支持**一个**主 trait + 若干**自动 trait**（Send / Sync / ...）：
//!
//! ```ignore
//! &dyn Read            // ✅
//! &dyn Read + Send     // ✅（Send 是 auto trait）
//! &dyn Read + Display  // ❌（两个非 auto trait）
//! ```
//!
//! 解决方法：
//! 1. 定义一个**组合 trait**：`trait ReadDisplay: Read + Display {}`
//! 2. 改用泛型 + trait bound

use std::fmt::Display;

trait Area { fn area(&self) -> u32; }
trait Named { fn name(&self) -> &str; }

/// 组合 trait：把 Area + Named 打包成一个新 trait，然后对 dyn ShapeInfo 进行统一处理
trait ShapeInfo: Area + Named {}
impl<T: Area + Named> ShapeInfo for T {}

struct Square;
impl Area for Square { fn area(&self) -> u32 { 25 } }
impl Named for Square { fn name(&self) -> &str { "square" } }

fn describe(shape: &dyn ShapeInfo) {
    println!("  [{}] area = {}", shape.name(), shape.area());
}

fn show(x: &dyn Display) { println!("  {x}"); }

pub fn run() {
    println!("== Multiple Traits ==");

    describe(&Square);
    show(&42);

    println!();
    println!("  &dyn A + B 不行；用组合 trait `trait AB: A + B {{}}` 绕开");
    println!();
}
