//! 逆变（contravariance）：**反向**的子类型关系。
//!
//! 如果 `F<T>` 在 T 上**逆变**：`A: B` 时 `F<B>: F<A>`（关系反转）。
//!
//! 实际例子：**函数参数**位置。
//!
//! ```text
//! fn(&'static str)  →  可以当 fn(&'a str) 用吗？
//! 不可以。但反过来可以：fn(&'a str) 接受任意短寿，比 fn(&'static str) 能力更强（更"严格"）。
//! ```
//!
//! 所以 `fn(T)` 在 T 上**逆变**——能接受更长（更泛用）寿命的函数，能当"只接受更短"的函数用。
//!
//! 这在纯 Rust 日常代码里碰到不多；主要在高级类型/生命周期推导里出现。

pub fn run() {
    println!("== Contravariance ==");

    println!("-- fn(&'a T) 在 'a 上逆变（直觉）--");
    // 取两个 fn 指针类型
    let f_any_str: fn(&str) = |s| println!("  any: {s}");
    let _f_static_only: fn(&'static str) = |s| println!("  static: {s}");

    // f_any_str 能接受任意 &str → 当然也能当 fn(&'static str) 用
    let use_as_static_only: fn(&'static str) = f_any_str;
    use_as_static_only("literal");

    // 反过来不行：只吃 'static 的函数，不能用作"接受任意寿"的函数。
    // let use_as_any: fn(&str) = f_static_only; // ← 编译失败
    println!("  fn(&str) 可以当 fn(&'static str) 用（逆变）");
    println!();
}
