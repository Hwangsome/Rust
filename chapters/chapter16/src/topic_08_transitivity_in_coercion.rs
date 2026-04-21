//! Coercion 的传递性：coercion **可以链式发生**。
//!
//! 例如：`Box<String>` 要传给 `&str` 参数，中间会发生这样一条链：
//!
//! ```text
//! Box<String>     →  String     (Deref)
//! &String         →  &str       (Deref)
//! ```
//!
//! 配合 reference、deref、unsize 三类 coercion，链可以变得很长但完全合法。
//! Rust 编译器会**尝试所有可能的 coercion 路径**，只要能走通就成功。
//!
//! 但要注意：chain 仅在 coercion 允许的场景里自动发生——在泛型参数上完全不试。

fn want_str(s: &str) { println!("  want_str: {s}"); }
fn want_slice(xs: &[i32]) { println!("  want_slice: {xs:?}"); }

pub fn run() {
    println!("== Transitivity in Coercion ==");

    println!("-- (1) Box<String> → &str --");
    let boxed = Box::new(String::from("hello"));
    want_str(&boxed); // &Box<String> → &String → &str
    println!();

    println!("-- (2) &Vec<i32> → &[i32] --");
    let v = vec![1, 2, 3];
    want_slice(&v); // &Vec<i32> → &[i32]
    println!();

    println!("-- (3) Box<Vec<i32>> → &[i32] --");
    let bv = Box::new(vec![10, 20, 30]);
    want_slice(&bv); // &Box<Vec<i32>> → &Vec<i32> → &[i32]（三层？其实是 deref 两层 + auto-ref）
    println!();

    println!("-- (4) 记忆 --");
    println!("  只要每一步单独是合法 coercion，它们就能链起来");
    println!();
}
