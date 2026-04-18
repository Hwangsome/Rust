# 2. Downcasting 完整示例

让自己的 trait 支持 downcast 的惯用法：

```rust
trait Widget: Any {
    fn as_any(&self) -> &dyn Any;
}
```

## 对应代码

- [topic_02_downcasting_example.rs](../../chapters/chapter18/src/topic_02_downcasting_example.rs)
