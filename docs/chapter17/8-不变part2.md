# 8. 不变（Invariance）Part 2：`Cell<T>` 与选型总结

> - **所属章节**：第 17 章 · Variance
> - **代码位置**：`chapters/chapter17/src/topic_08_invariance_part_2.rs`
> - **上一篇**：[7. 不变（part 1）](./7-不变part1.md)
> - **下一篇**：本章最后一篇

---

## `Cell<T>` 也是不变的

`Cell<T>` 提供内部可变性，类似于隐藏的 `&mut`。因此它对 T 也必须不变。

```rust
use std::cell::Cell;
// Cell<T>: Invariant over T
// 道理和 &mut T 一样：内部可变性意味着写操作
```

---

## 方差完整速查

| 类型 | T 上的方差 | 原因 |
|-----|-----------|-----|
| `&'a T` | 协变（T 和 'a 均是）| 只读 |
| `&'a mut T` | 不变（T），协变（'a）| 可写 |
| `Box<T>` | 协变 | 独占，类似只读 |
| `Vec<T>` | 协变 | 不允许外部改变生命周期 |
| `Cell<T>` | 不变 | 内部可变 |
| `RefCell<T>` | 不变 | 内部可变 |
| `Mutex<T>` | 不变 | 内部可变 |
| `fn(T) -> U` | T 逆变，U 协变 | 输入/输出方向不同 |
| `PhantomData<T>` | 协变 | 只影响类型系统，不写 |
| `PhantomData<*mut T>` | 不变 | 模拟可写 |
| `PhantomData<fn(T)>` | 逆变 | 模拟函数参数 |

---

## 记忆总结

```
只读       → 协变  （可以放松，更长可以用于更短）
函数参数   → 逆变  （能做更多的更好）
可写       → 不变  （必须完全匹配）
```

---

## 第 17 章完成

- 回到目录：[第 17 章：Variance](./README.md)
