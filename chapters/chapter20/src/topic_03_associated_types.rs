//! Trait 对象对**关联类型**的限制。
//!
//! 写 `dyn Iterator` 是不行的——因为 `Iterator::Item` 是未指定的关联类型，编译器生成 vtable 时不知道具体类型。
//!
//! 必须写具体：`dyn Iterator<Item = i32>`。

pub fn run() {
    println!("== Associated Types & dyn ==");

    // ❌ let v: Box<dyn Iterator> = ... // 不行，Item 类型未定
    // ✅ 必须指定：
    let v: Box<dyn Iterator<Item = i32>> = Box::new((1..=3).into_iter());
    let sum: i32 = v.sum();
    println!("  sum of 1..=3 via dyn Iterator<Item=i32> = {sum}");
    println!();
}
