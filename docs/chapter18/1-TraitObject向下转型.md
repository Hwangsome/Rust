# 1. Trait Object 向下转型：`dyn Any`

`std::any::Any` 让运行时"类型判断 + 转回具体类型"成为可能：

- `downcast_ref::<T>() -> Option<&T>`
- `downcast_mut::<T>() -> Option<&mut T>`
- `Box<dyn Any>::downcast::<T>() -> Result<Box<T>, Box<dyn Any>>`

## 对应代码

- [topic_01_downcasting_trait_objects.rs](../../chapters/chapter18/src/topic_01_downcasting_trait_objects.rs)
