# 6. 大小与 Trait Object

`dyn Trait` 是 unsized，必须藏在指针后面：`&dyn`、`Box<dyn>`、`Rc<dyn>` 等。

## 对应代码

- [topic_06_size_and_trait_objects.rs](../../chapters/chapter17/src/topic_06_size_and_trait_objects.rs)
