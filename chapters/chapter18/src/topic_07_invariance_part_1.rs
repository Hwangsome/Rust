//! 不变（invariance）part 1：**`&mut T` 在 T 上是不变的**。
//!
//! 为什么？因为可变引用既会"读"又会"写"。
//!
//! - 如果 `&mut T` 协变：可能通过长寿引用写入短寿数据，导致悬垂
//! - 如果 `&mut T` 逆变：可能通过短寿引用读走长寿数据，做同样的坏事
//!
//! 所以 `&mut T` **必须完全匹配**——这就是不变。
//!
//! 这解释了为什么很多你以为应该合法的代码会失败：涉及 `&mut` 时，
//! 生命周期必须"分毫不差"，不能自动放宽。

/// 演示 &mut Vec<&'a str>：只用不同的 lifetime 示范常见"协变不能帮你"的场景。
/// 不强制要求外部 lifetime 与内部相同，以免触发 NLL 之外的经典不变锁问题。
fn accept_mut<'a, 'b>(_x: &'a mut Vec<&'b str>) {}

pub fn run() {
    println!("== Invariance (part 1): &mut T ==");

    println!("-- &mut T 对 T 不变 --");
    let a = String::from("a");
    let mut v: Vec<&str> = vec![&a];
    accept_mut(&mut v);
    println!("  len = {}", v.len());
    println!();

    println!("关键理解:");
    println!("  如果 &mut Vec<&'long T> 能当作 &mut Vec<&'short T>（协变），");
    println!("  调用方就可能把短寿引用 push 进长寿 Vec → 悬垂");
    println!("  所以 &mut T 必须精确匹配 → 不变");
    println!();
}
