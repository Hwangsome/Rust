# 1. Coercion 基础 + Deref 强转

> 关键词：coercion、Deref、implicit、coercion site

## 一分钟结论

- **Coercion**（强转）是编译器在特定位置自动做的类型转换，不用显式 `as`
- 最常见：**Deref 强转**——`&String → &str`、`&Vec<T> → &[T]`、`&Box<T> → &T`
- 可以**链式发生**：`&Box<String> → &String → &str`（两层）

## 对应代码

- [topic_01_coercion_basics_deref_coercion.rs](../../chapters/chapter13/src/topic_01_coercion_basics_deref_coercion.rs)

## 自己实现 Deref 就能参与强转

```rust
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> { type Target = T; fn deref(&self) -> &T { &self.0 } }
```

之后 `&MyBox<T>` 就能像 `&T` 一样用——这就是 Rust 智能指针体验这么丝滑的底层原因。
