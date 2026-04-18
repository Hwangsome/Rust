# 9. Least Upper Bound (LUB) 强转

> 关键词：LUB、分支合流、never type、&dyn Trait

## 一分钟结论

当 `if`/`match` 的多个分支类型不同，编译器会尝试找一个两者都能 coerce 成的**共同类型**：

- 两支都返回 `&'static str` → 取 `&'static str`
- 一支是 `!` → 取另一支类型（`!` 能变任何类型）
- 两支分别是 `&A` / `&B` 且都实现 Trait → 需要**显式** `as &dyn Trait`，编译器不会自动引入 trait object
- 找不到 → E0308

## 对应代码

- [topic_09_least_upper_bound_coercion.rs](../../chapters/chapter13/src/topic_09_least_upper_bound_coercion.rs)

## 实战提示

遇到分支类型不同时：
1. 先想想是不是应该都 return 同一种类型
2. 如果确实要异构，显式 `as &dyn Trait` 或 `Box<dyn Trait>`
