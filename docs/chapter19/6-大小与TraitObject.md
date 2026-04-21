# 6. 大小与 Trait Object

> - **所属章节**：第 18 章 · Trait Object Limitations
> - **代码位置**：`chapters/chapter19/src/topic_06_size_and_trait_objects.rs`
> - **上一篇**：[5. 无 self 方法限制](./5-无self方法.md)
> - **下一篇**：[7. 部分对象安全](./7-部分对象安全.md)

---

## `dyn Trait` 是 DST，必须在指针后面

```rust
use std::mem::size_of;

trait Animal { fn name(&self) -> &str; }
struct Dog;
impl Animal for Dog { fn name(&self) -> &str { "dog" } }

println!("&Dog         = {} bytes", size_of::<&Dog>());        // 8（细指针）
println!("&dyn Animal  = {} bytes", size_of::<&dyn Animal>()); // 16（胖指针）
println!("Box<dyn Animal> = {} bytes", size_of::<Box<dyn Animal>>());  // 16

// ❌ dyn Animal 不能按值使用
// fn f(x: dyn Animal) { } // 大小不确定

// ✅ 通过指针
fn f(x: &dyn Animal) { println!("{}", x.name()); }
fn g(x: Box<dyn Animal>) { println!("{}", x.name()); }
```

---

## 下一步

- 继续阅读：[7. 部分对象安全](./7-部分对象安全.md)
