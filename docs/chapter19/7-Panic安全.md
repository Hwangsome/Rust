# 7. Panic 安全

- Rust 默认 panic 模式是"栈展开"——每层栈帧的 Drop 都会跑
- 不要在 Drop 里 panic（会升级为 abort）
- `catch_unwind` 用于边界场景（FFI、测试框架），不能替代 Result

## 对应代码

- [topic_07_panic_safety.rs](../../chapters/chapter19/src/topic_07_panic_safety.rs)
