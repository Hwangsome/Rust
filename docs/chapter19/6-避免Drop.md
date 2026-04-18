# 6. 避免 Drop

- `ManuallyDrop<T>`：包一层，drop 不会自动运行
- `mem::forget(x)`：直接忘记（慎用，会泄漏）
- `Box::leak(b)`：把 `Box<T>` 泄漏成 `&'static mut T`

## 对应代码

- [topic_06_avoiding_drop.rs](../../chapters/chapter19/src/topic_06_avoiding_drop.rs)
