# 2. 多 Trait 组合限制

> - **所属章节**：第 18 章 · Trait Object Limitations
> - **代码位置**：`chapters/chapter18/src/topic_02_multiple_traits.rs`
> - **上一篇**：[1. 孤儿规则](./1-孤儿规则.md)
> - **下一篇**：[3. 关联类型限制](./3-关联类型限制.md)

---

## `dyn A + B` 只支持一个非 auto trait

```rust
// ❌ 不能有两个非 auto trait
// let v: Box<dyn Display + Debug> = Box::new(42);

// ✅ 可以 non-auto trait + auto trait（Send/Sync/Unpin 等）
let v: Box<dyn std::fmt::Display + Send + Sync> = Box::new(42);

// ✅ 解决方案：定义组合 trait
use std::fmt::{Display, Debug};

trait DisplayDebug: Display + Debug {}
impl<T: Display + Debug> DisplayDebug for T {}

let v: Box<dyn DisplayDebug> = Box::new(42);
println!("{}", v); // Display
println!("{:?}", v); // Debug
```

---

## 为什么有这个限制

vtable 的布局是固定的：每个 `dyn Trait` 对应一张 vtable。两个不同 trait 需要两张 vtable，但胖指针只有一个 vtable 指针。所以只能通过组合 trait 的方式合并成一张 vtable。

---

## 下一步

- 继续阅读：[3. 关联类型限制](./3-关联类型限制.md)
