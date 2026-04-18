# 9. Sealed Trait (封闭 trait)

用私有模块里的 trait 做基类，让外部 crate **无法实现**你的 public trait。

```rust
mod private { pub trait Sealed {} }
pub trait MyTrait: private::Sealed { ... }
```

## 对应代码

- [topic_09_sealed_traits.rs](../../chapters/chapter17/src/topic_09_sealed_traits.rs)
