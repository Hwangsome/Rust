# 2. 多 Trait 组合限制

- `&dyn A + B` **不支持**（B 是非 auto trait 时）
- 解法：定义组合 trait `trait AB: A + B {}`

## 对应代码

- [topic_02_multiple_traits.rs](../../chapters/chapter17/src/topic_02_multiple_traits.rs)
