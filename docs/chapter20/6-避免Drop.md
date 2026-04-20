# 6. 避免 Drop：`ManuallyDrop` 与 `forget`

> - **所属章节**：第 20 章 · Dropcheck
> - **代码位置**：`chapters/chapter20/src/topic_06_avoiding_drop.rs`
> - **上一篇**：[5. Drop 顺序与自引用](./5-Drop顺序与自引用.md)
> - **下一篇**：[7. Panic 安全](./7-Panic安全.md)

---

## 三种阻止 Drop 的方法

```rust
use std::mem::{ManuallyDrop, forget};

struct Resource(&'static str);
impl Drop for Resource {
    fn drop(&mut self) { println!("drop {}", self.0); }
}

// 1. ManuallyDrop：不自动 drop，可以选择性手动 drop
let mut r = ManuallyDrop::new(Resource("manual"));
println!("ManuallyDrop 不会自动 drop");
unsafe { ManuallyDrop::drop(&mut r); } // 手动决定何时 drop

// 2. forget：直接忘记（内存泄漏！慎用）
let r = Resource("forgotten");
forget(r); // r 的 drop 永远不会被调用
println!("forgotten 后不会打印 drop 消息");

// 3. Box::leak：泄漏为 'static 引用
let boxed = Box::new(Resource("leaked"));
let leaked: &'static mut Resource = Box::leak(boxed);
println!("leaked.0 = {}", leaked.0);
// leaked 指向的内存永远不会被释放（除非程序退出）
```

---

## 何时使用

- `ManuallyDrop`：FFI、自定义内存管理、unsafe 代码
- `forget`：所有权转移到了 FFI 层，Rust 不应该 drop
- `Box::leak`：需要 'static 引用的场景（如全局配置）

---

## 下一步

- 继续阅读：[7. Panic 安全](./7-Panic安全.md)
