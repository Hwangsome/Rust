//! Trait 方法**必须有 `self` 接收者**才能通过 `dyn Trait` 调用。
//!
//! ```ignore
//! trait T {
//!     fn create() -> Self;  // 没有 &self → 不能通过 dyn T 调用
//!     fn run(&self);         // 有 &self → 可以
//! }
//! ```
//!
//! 原因：`dyn T` 在调用时需要通过 vtable 找方法；没有 self 就没有 vtable 入口。

trait Greet {
    fn hello(&self) -> &str; // OK for dyn
    fn default_msg() -> &'static str where Self: Sized { "default" }
    // `where Self: Sized` 把 default_msg 排除在 trait object 外，让整个 trait 依然 object-safe
}

struct En;
impl Greet for En { fn hello(&self) -> &str { "hello" } }

pub fn run() {
    println!("== Functions without self ==");

    let g: &dyn Greet = &En;
    println!("  {}", g.hello());
    // g.default_msg(); // ❌ 不能通过 dyn 调用，因为它没有 self
    println!("  `fn no_self()` 仍然可以在 trait 里存在，但必须用 `where Self: Sized` 排除出 object");
    println!();
}
