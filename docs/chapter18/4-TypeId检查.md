# 4. 不转型只检查类型：`TypeId`

- `TypeId::of::<T>()` 给出类型唯一标识
- `x.type_id()` 拿到 `&dyn Any` 对应的类型标识
- 用 `==` 比较——无需 downcast 就能走分支

## 对应代码

- [topic_04_checking_type_without_downcasting_using_typeid.rs](../../chapters/chapter18/src/topic_04_checking_type_without_downcasting_using_typeid.rs)
