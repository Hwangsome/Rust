//! Drop check：当一个值被 drop 时，编译器会验证它内部借用的**其他值**仍然活着。
//!
//! Rust 不允许 drop 时触发悬垂引用：
//!
//! ```ignore
//! struct Tag<'a>(&'a String);
//! let name = String::from("rust");
//! let tag  = Tag(&name);
//! drop(name); // ❌ E0505：tag 还需要 name 活着
//! // tag 在这里自动被 drop，它的 &name 会悬垂
//! ```
//!
//! 更精细的规则：`#[may_dangle]` 属性允许某些带生命周期字段的类型在 drop 时放宽要求（标准库内部 Vec / Box 用到）。

struct Tag<'a>(#[allow(dead_code)] &'a String);

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        println!("  dropping Tag (still has live inner ref)");
    }
}

pub fn run() {
    println!("== Drop Check ==");
    let name = String::from("rust");
    let _tag = Tag(&name);
    // _tag 在本函数结束时自动 drop，此时 name 仍然活着 → 合法
    println!("  end of scope: _tag drops first, then name");
    println!();
}
