# 2. Downcasting 完整示例

> - **所属章节**：第 18 章 · Downcasting
> - **代码位置**：`chapters/chapter18/src/topic_02_downcasting_example.rs`
> - **上一篇**：[1. Trait Object 向下转型](./1-TraitObject向下转型.md)
> - **下一篇**：[3. Trait Object 间转换](./3-TraitObject间转换.md)

---

## 让自定义 trait 支持 downcast

```rust
use std::any::Any;

trait Widget: Any {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any; // 提供 Any 访问
}

struct Button { label: String }
struct Slider { min: f64, max: f64 }

impl Widget for Button {
    fn name(&self) -> &str { "Button" }
    fn as_any(&self) -> &dyn Any { self }
}
impl Widget for Slider {
    fn name(&self) -> &str { "Slider" }
    fn as_any(&self) -> &dyn Any { self }
}

let widgets: Vec<Box<dyn Widget>> = vec![
    Box::new(Button { label: "OK".into() }),
    Box::new(Slider { min: 0.0, max: 100.0 }),
];

for w in &widgets {
    println!("Widget: {}", w.name());

    if let Some(btn) = w.as_any().downcast_ref::<Button>() {
        println!("  是 Button，label = {}", btn.label);
    }
    if let Some(sld) = w.as_any().downcast_ref::<Slider>() {
        println!("  是 Slider，范围 [{}, {}]", sld.min, sld.max);
    }
}
```

---

## 下一步

- 继续阅读：[3. Trait Object 间转换](./3-TraitObject间转换.md)
