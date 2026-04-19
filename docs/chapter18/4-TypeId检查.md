# 4. 用 TypeId 检查类型

> - **所属章节**：第 18 章 · Downcasting
> - **代码位置**：`chapters/chapter18/src/topic_04_checking_type_without_downcasting_using_typeid.rs`
> - **上一篇**：[3. Trait Object 间转换](./3-TraitObject间转换.md)
> - **下一篇**：本章最后一篇

---

## TypeId：只检查类型，不取值

```rust
use std::any::{Any, TypeId};

fn check_type(x: &dyn Any) {
    if x.type_id() == TypeId::of::<i32>() {
        println!("是 i32");
    } else if x.type_id() == TypeId::of::<String>() {
        println!("是 String");
    } else {
        println!("是其他类型: {:?}", x.type_id());
    }
}

check_type(&42_i32);
check_type(&String::from("hello"));
check_type(&3.14_f64);
```

---

## TypeId vs downcast_ref

| 用途 | TypeId | downcast_ref |
|-----|-------|-------------|
| 检查类型（不获取值）| ✅ | ✅（但开销稍高）|
| 获取类型的值 | ❌ | ✅ |
| 需要的操作数 | `x.type_id()` | `x.downcast_ref::<T>()` |

---

## 第 18 章完成

- 回到目录：[第 18 章：Downcasting](./README.md)
