# 7. Panic 安全

> - **所属章节**：第 20 章 · Dropcheck
> - **代码位置**：`chapters/chapter21/src/topic_07_panic_safety.rs`
> - **上一篇**：[6. 避免 Drop](./6-避免Drop.md)
> - **下一篇**：本章最后一篇

---

## Panic 展开时的 Drop

当 `panic!` 发生时，Rust 默认**展开（unwind）**调用栈，沿途调用每个局部变量的 `drop()`。

```rust
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) { println!("Guard: 清理资源（即使在 panic 时）"); }
}

let result = std::panic::catch_unwind(|| {
    let _g = Guard;
    println!("即将 panic");
    panic!("出错了！");
    // _g 会在 panic 展开时被 drop
});
println!("panic 被捕获: {:?}", result.is_err());
```

---

## 关键规则

- **不要在 `Drop` 里 panic**！如果 drop 时又 panic，会立刻 abort（不是正常的 panic）
- `std::panic::catch_unwind` 可以捕获 panic，但不建议用于常规错误处理
- 用于边界（FFI、测试框架等）

---

## 两种 panic 模式

```toml
# Cargo.toml
[profile.release]
panic = "abort"   # panic 直接 abort，不展开（更小的二进制，更快）
# panic = "unwind" # 默认：展开并运行 Drop
```

---

## 第 20 章完成

- 回到目录：[第 20 章：Dropcheck](./README.md)
