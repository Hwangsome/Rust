//! Drop check 与部分 move：struct 字段部分被 move 后，整体不会再自动 drop 那部分。
//!
//! ```ignore
//! struct Pair { a: String, b: String }
//! let p = Pair { a: "1".into(), b: "2".into() };
//! let a = p.a;  // move 了 p.a
//! // drop(p) 不会再 drop p.a，但会 drop p.b
//! ```

#[derive(Debug)]
struct Pair { a: String, b: String }

impl Drop for Pair {
    fn drop(&mut self) {
        // Drop 只有在**完整持有**时才会运行。Rust 在编译期保证不会在部分 move 后还调用 Drop。
        println!("  drop Pair");
    }
}

pub fn run() {
    println!("== Drop Check & Partial Move ==");

    {
        let p = Pair { a: "a-field".into(), b: "b-field".into() };
        println!("  created {p:?}");
        // 这里不做 partial move，走正常 drop
    }
    println!();

    println!("-- Partial move: Rust 静态禁止 'drop after partial move' --");
    println!("  如果 Pair 带 `impl Drop`，partial move 后整个 struct 不能再用（也不会再 drop）");
    println!("  `let Pair {{ a, .. }} = p;` 这种拆解式 move **不适用于**带 Drop 的 struct");
    println!();
}
