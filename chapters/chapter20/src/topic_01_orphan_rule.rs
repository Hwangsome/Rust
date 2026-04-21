//! 孤儿规则（orphan rule）：`impl Trait for T` 的**合法位置**。
//!
//! 基本规则：**要给 T 实现 Trait，Trait 或 T 至少有一个必须在你的 crate 里定义。**
//!
//! 这条规则阻止了"两个无关的 crate 都给同一个外部 T 实现同一个外部 Trait"导致的冲突。
//!
//! | 定义方          | T 在哪   | Trait 在哪  | 能不能实现？      |
//! |------------|-------|---------|------------|
//! | 你的 crate     | 自己的   | 自己的     | ✅            |
//! | 你的 crate     | 自己的   | 外部的     | ✅（为自己类型实现外部 trait） |
//! | 你的 crate     | 外部的   | 自己的     | ✅（为外部类型实现自己 trait） |
//! | 你的 crate     | 外部的   | 外部的     | ❌           |
//!
//! 绕开方法：**newtype 模式**——把外部类型包一层自己的 struct。

/// newtype wrapper：让我们能给外部类型"实现" Display。
struct Wrap(Vec<i32>);

impl std::fmt::Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[wrap]{:?}", self.0)
    }
}

pub fn run() {
    println!("== Orphan Rule ==");
    let w = Wrap(vec![1, 2, 3]);
    println!("  {w}");
    println!("  通过 newtype 绕过孤儿规则，给 Vec 间接加 Display");
    println!();
}
