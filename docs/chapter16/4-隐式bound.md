# 4. 隐式生命周期 Bound

> - **所属章节**：第 16 章 · Variance
> - **代码位置**：`chapters/chapter16/src/topic_04_background_for_variance_implied_bounds.rs`
> - **上一篇**：[3. 生命周期边界](./3-生命周期边界.md)
> - **下一篇**：[5. 协变](./5-协变.md)

---

## 编译器自动添加的 bound

当你写 `&'a T` 时，Rust 隐式添加 `T: 'a`——因为如果 T 里有比 `'a` 短的引用，那这个 `&'a T` 就是不安全的。

```rust
// 你写的：
fn f<'a, T>(x: &'a T) { }

// 编译器理解为：
fn f<'a, T: 'a>(x: &'a T) { }
// 要求 T 里所有引用都活得不短于 'a
```

---

## 实际意义

这就是为什么你不需要每次都显式写 `T: 'a`——编译器从 `&'a T` 就知道了：

```rust
struct Cache<'a, T> {
    data: &'a T, // 隐式加了 T: 'a
}
// 等价于：
// struct Cache<'a, T: 'a> { data: &'a T }
```

---

## 下一步

- 继续阅读：[5. 协变](./5-协变.md)
