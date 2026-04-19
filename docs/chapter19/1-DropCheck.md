# 1. Drop Check

> - **所属章节**：第 19 章 · Dropcheck
> - **Cargo package**：`chapter19`
> - **代码位置**：`chapters/chapter19/src/topic_01_drop_check.rs`

---

## Drop Check 是什么

当一个类型 `T` 的 `Drop` 实现访问了它持有的引用时，借用检查器需要确保：在 `T` 被 drop 的时候，这些引用指向的数据仍然有效。

```rust
struct Important<'a>(&'a str);

impl<'a> Drop for Important<'a> {
    fn drop(&mut self) {
        // Drop 时会访问 &'a str——所以借用检查器要求
        // 原始字符串数据在 Important drop 之后才能消失
        println!("重要: {}", self.0);
    }
}

let s = String::from("hello");
let i = Important(&s);
// 注意：i 必须在 s 之前被 drop
// Rust 的 drop 顺序（声明的逆序）自然保证了这一点
```

---

## 关键：drop 顺序是声明的逆序

```rust
{
    let a = String::from("a"); // 先声明
    let b = String::from("b"); // 后声明

    // 离开作用域时：先 drop b，再 drop a
    // 这保证了"后声明的引用先被 drop"的安全性
}
```

---

## 下一步

- 继续阅读：[2. Drop 与部分 Move](./2-Drop与部分move.md)
