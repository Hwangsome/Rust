# 5. Drop 顺序与自引用

自引用 struct 在 Rust 里几乎不能安全构造——因为 drop 顺序固定，必须保证先 drop 的字段不被后 drop 的字段引用。

需要真的自引用 → `Pin` + unsafe，或用 `ouroboros` 等 crate。

## 对应代码

- [topic_05_drop_order_and_self_referencing.rs](../../chapters/chapter19/src/topic_05_drop_order_and_self_referencing.rs)
