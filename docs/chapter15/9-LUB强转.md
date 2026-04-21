# 9. LUB 强转：分支类型统一

> - **所属章节**：第 14 章 · Coercion in Rust
> - **代码位置**：`chapters/chapter15/src/topic_09_least_upper_bound_coercion.rs`
> - **上一篇**：[8. Coercion 传递性](./8-Coercion传递性.md)
> - **下一篇**：本章最后一篇

---

## if/match 分支的类型统一（LUB）

当 `if` 或 `match` 的多个分支类型不同时，编译器会寻找"最小上界类型"（Least Upper Bound）：

```rust
// Never 类型 → 取另一分支的类型
let x: i32 = if true { 42 } else { panic!("never") };
// panic! 类型是 !（底类型），可以"变成" i32

// &str 分支统一
let s: &str = if true { "hello" } else { "world" };
// 两个 &str 分支，统一为 &str

// 不同具体类型 → 需要显式：
// let val = if cond { 42i32 } else { "str" }; // ❌
// 改用 dyn Display：
use std::fmt::Display;
let val: &dyn Display = if true { &42i32 } else { &"str" };
println!("{val}");
```

---

## 注意

`!`（never type）可以参与任何类型的 LUB，因为它是所有类型的子类型。

---

## 第 14 章完成

第 14 章（Coercion）的核心在于理解：

1. 哪些地方会自动发生 coercion
2. Deref 链式 coercion 的工作方式
3. 泛型参数不会触发 coercion
4. `!` 参与分支统一

- 回到目录：[第 14 章：Coercion in Rust](./README.md)
- 下一章：[第 14 章：并发](../chapter15/README.md)
