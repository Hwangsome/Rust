# 1. Trait Object 向下转型

> - **所属章节**：第 19 章 · Downcasting
> - **Cargo package**：`chapter19`
> - **代码位置**：`chapters/chapter19/src/topic_01_downcasting_trait_objects.rs`

---

## 为什么需要向下转型

`dyn Trait` 擦除了具体类型。有时你需要从"抽象"恢复到"具体"：

```rust
use std::any::Any;

let boxed: Box<dyn Any> = Box::new(42_i32);

// downcast_ref：不消耗，返回 Option<&T>
if let Some(n) = boxed.downcast_ref::<i32>() {
    println!("找到 i32: {n}");
}

// downcast：消耗，返回 Result<Box<T>, Box<dyn Any>>
let boxed: Box<dyn Any> = Box::new(String::from("hello"));
match boxed.downcast::<String>() {
    Ok(s) => println!("是 String: {s}"),
    Err(other) => println!("不是 String"),
}
```

---

## 条件：类型必须是 `'static`

`Any` trait 要求类型实现 `'static`（不含非 'static 引用）。

---

## 下一步

- 继续阅读：[2. Downcasting 完整示例](./2-Downcasting完整示例.md)
