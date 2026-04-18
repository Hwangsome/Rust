# 8. 不变 part 2：`Cell<T>` / `RefCell<T>`

内部可变性 ≈ 隐式 &mut，所以 `Cell<T>` / `RefCell<T>` 对 T 也不变。

## 三种方差速记

| 方差 | 典型 | 规则 |
|-----|-----|-----|
| 协变 | `&T`、`Box<T>`、`Vec<T>` | 长寿 → 短寿 OK |
| 逆变 | `fn(T)` 的参数 | 短寿 ← 长寿 OK |
| 不变 | `&mut T`、`Cell<T>` | 必须精确匹配 |

## 对应代码

- [topic_08_invariance_part_2.rs](../../chapters/chapter16/src/topic_08_invariance_part_2.rs)
