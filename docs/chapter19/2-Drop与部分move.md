# 2. Drop 与部分 Move

> - **所属章节**：第 19 章 · Dropcheck
> - **代码位置**：`chapters/chapter19/src/topic_02_drop_check_and_partial_move.rs`
> - **上一篇**：[1. Drop Check](./1-DropCheck.md)
> - **下一篇**：[3. Drop 顺序（part 1）](./3-Drop顺序part1.md)

---

## 有 Drop 的类型不能部分 Move

```rust
struct Pair {
    first: String,
    second: String,
}

// 没有 Drop：可以部分 move
let pair = Pair { first: "a".into(), second: "b".into() };
let first = pair.first;  // ✅ 部分 move
// pair.second 还可用

// 有 Drop 的类型
struct PairWithDrop {
    first: String,
    second: String,
}
impl Drop for PairWithDrop {
    fn drop(&mut self) { println!("dropped!"); }
}

let pair = PairWithDrop { first: "a".into(), second: "b".into() };
// let first = pair.first; // ❌ 不允许！因为有 Drop，必须整体 drop
```

---

## 为什么

如果允许部分 move，`drop()` 在被调用时，已被 move 走的字段就是未初始化的内存——`drop()` 如果访问它就是 UB。

---

## 下一步

- 继续阅读：[3. Drop 顺序（part 1）](./3-Drop顺序part1.md)
