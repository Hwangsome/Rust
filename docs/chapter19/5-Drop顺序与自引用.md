# 5. Drop 顺序与自引用

> - **所属章节**：第 19 章 · Dropcheck
> - **代码位置**：`chapters/chapter19/src/topic_05_drop_order_and_self_referencing.rs`
> - **上一篇**：[4. Drop 顺序（part 2）](./4-Drop顺序part2.md)
> - **下一篇**：[6. 避免 Drop](./6-避免Drop.md)

---

## 自引用在 Rust 里几乎不可能安全实现

```rust
// ❌ 试图让 struct 的字段引用自己的其他字段
struct SelfRef {
    data: String,
    pointer: *const String, // 指向 data 的裸指针（脆弱！）
}

// 问题：移动 SelfRef 时，data 移到了新地址，但 pointer 还指向旧地址
// → 悬垂指针！
```

---

## 为什么 drop 顺序让自引用危险

drop 顺序是确定的（按字段声明），但如果一个字段的 `Drop` 实现使用了另一个字段的引用，那么：
- 如果被引用的字段先 drop → drop 时已是悬垂引用 → UB

Rust 的 drop check 就是防止这种情况。

---

## 安全的自引用方案

1. `Pin<Box<T>>` + `unsafe`
2. 使用 `ouroboros` 或 `self_cell` crate
3. 重新设计，避免自引用（通常是最好的选择）

---

## 下一步

- 继续阅读：[6. 避免 Drop](./6-避免Drop.md)
