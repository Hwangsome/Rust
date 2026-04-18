# 7. 泛型与 Coercion：不会自动发生

> 关键词：泛型参数、AsRef、Borrow

## 一分钟结论

- **Coercion 不会在泛型参数位置自动发生**
- 想让泛型函数接受多种兼容类型，要用 trait bound（`AsRef` / `Borrow` / `Into`）
- 这是区分 "coercion" 和 "conversion" 的关键分界

## 对应代码

- [topic_07_coercion_in_generics.rs](../../chapters/chapter13/src/topic_07_coercion_in_generics.rs)

## 实战

```rust
fn print_generic<T: AsRef<[i32]>>(s: T) { ... }
// 下面三种调用都合法：
print_generic([1, 2, 3]);
print_generic(vec![4, 5, 6]);
print_generic(&[7, 8, 9] as &[i32]);
```
