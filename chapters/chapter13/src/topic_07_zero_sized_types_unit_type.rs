//! 零大小类型 (ZST) 之一：**Unit** `()`。
//!
//! `()` 是 Rust 最基础的 ZST：它只有**一个**值，而且在内存里**完全不占空间**。
//!
//! 常见出场：
//! - 没有返回值的函数的"返回类型"就是 `()`
//! - `if/else` 里带分号的块返回 `()`
//! - `HashSet<T>` 本质是 `HashMap<T, ()>`——用 `()` 做 value 意思"我只关心 key 在不在"
//! - channel / 事件 / future 里用 `Result<(), E>` 表示"成功但没携带数据"

use std::collections::{HashMap, HashSet};
use std::mem::size_of;

pub fn run() {
    println!("== ZST: Unit Type `()` ==");

    println!("-- (1) 大小为 0 --");
    println!("  size_of::<()>() = {}", size_of::<()>());
    println!();

    println!("-- (2) 带分号块返回 () --");
    let v: () = {
        let _ = 1 + 1;
    };
    println!("  {v:?}");
    println!();

    println!("-- (3) HashSet<T> 在内部就是 HashMap<T, ()> --");
    let mut s: HashSet<&str> = HashSet::new();
    s.insert("rust");
    s.insert("go");
    println!("  set = {s:?}");

    // 如果你看源码，会发现 HashSet 大致长这样：
    // pub struct HashSet<T, S = ...> { map: HashMap<T, (), S> }
    // 用 () 做 value 零额外成本——这就是 ZST 的价值。
    let _illustration: HashMap<&str, ()> = HashMap::new();
    println!();

    println!("-- (4) Result<(), E> 表示‘成功但没数据’ --");
    fn save() -> Result<(), String> { Ok(()) }
    println!("  save() = {:?}", save());
    println!();
}
