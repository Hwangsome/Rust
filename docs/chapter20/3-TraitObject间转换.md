# 3. Trait Object 之间的转换

> - **所属章节**：第 19 章 · Downcasting
> - **代码位置**：`chapters/chapter20/src/topic_03_downcasting_for_conversion_between_trait_objects.rs`
> - **上一篇**：[2. Downcasting 完整示例](./2-Downcasting完整示例.md)
> - **下一篇**：[4. TypeId 检查](./4-TypeId检查.md)

---

## `&dyn A` 转 `&dyn B` 必须经过具体类型

Rust 不支持直接 `dyn A as dyn B`。路径：

```
&dyn A → downcast → &ConcreteType → upcast → &dyn B
```

```rust
use std::any::Any;

trait Readable: Any {
    fn read(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
trait Writable { fn write(&mut self, data: &str); }

struct FileHandle { content: String }

impl Readable for FileHandle {
    fn read(&self) -> String { self.content.clone() }
    fn as_any(&self) -> &dyn Any { self }
}
impl Writable for FileHandle { fn write(&mut self, data: &str) { self.content = data.into(); } }

fn convert_readable_to_writable(r: &dyn Readable) -> Option<&dyn Writable> {
    // 不能直接转换，必须先知道具体类型
    None // 简化：实际需要 Registry 或 as_any downcast + 再 upcast
}
```

---

## 下一步

- 继续阅读：[4. TypeId 检查](./4-TypeId检查.md)
