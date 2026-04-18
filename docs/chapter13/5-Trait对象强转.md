# 5. Trait 对象强转

> 关键词：&dyn Trait、Box<dyn Trait>、object-safe

## 一分钟结论

当 T 实现了 `Trait` 且 `Trait` 是 object-safe，Rust 自动做这些强转：

| 原类型 | 目标 |
|-------|-----|
| `&T` | `&dyn Trait` |
| `Box<T>` | `Box<dyn Trait>` |
| `Rc<T>` / `Arc<T>` | `Rc<dyn Trait>` / `Arc<dyn Trait>` |

## 对应代码

- [topic_05_trait_objects_coercion.rs](../../chapters/chapter13/src/topic_05_trait_objects_coercion.rs)

## 搭配集合使用

```rust
let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
```

每次 push 都隐式强转——所以 vec! 里可以写不同具体类型。
