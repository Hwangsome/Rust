# 3. Trait Object 之间的转换

Rust 没有直接 `&dyn A → &dyn B`。必须：

1. `as_any()` 拿到 `&dyn Any`
2. downcast 到具体类型
3. 再 `as &dyn B` 重新 upcast

## 对应代码

- [topic_03_downcasting_for_conversion_between_trait_objects.rs](../../chapters/chapter18/src/topic_03_downcasting_for_conversion_between_trait_objects.rs)
