# 9. Sealed Trait：防止外部实现

> - **所属章节**：第 18 章 · Trait Object Limitations
> - **代码位置**：`chapters/chapter19/src/topic_09_sealed_traits.rs`
> - **上一篇**：[8. 运算符重载](./8-运算符重载.md)
> - **下一篇**：本章最后一篇

---

## 什么是 Sealed Trait

有时你希望只有你的 crate 里的类型才能实现某个 trait（防止用户实现破坏不变量）。这叫 sealed trait 模式。

---

## 实现方式

```rust
// 私有 sealed 模块
mod private {
    pub trait Sealed {}
}

// 公开 trait 要求实现 Sealed（外部看不到 Sealed）
pub trait MyTrait: private::Sealed {
    fn do_something(&self);
}

// 只有你的类型才能实现：
pub struct TypeA;
pub struct TypeB;

// 自己的类型 impl Sealed
impl private::Sealed for TypeA {}
impl private::Sealed for TypeB {}

impl MyTrait for TypeA {
    fn do_something(&self) { println!("TypeA"); }
}

impl MyTrait for TypeB {
    fn do_something(&self) { println!("TypeB"); }
}

// 外部代码无法实现 MyTrait，因为看不到 private::Sealed
// impl MyTrait for ExternalType {}  // ❌ private::Sealed 对外不可见
```

---

## 标准库中的例子

`std::fmt::Debug` 和 `std::fmt::Display` 使用了类似机制防止外部修改内部实现细节。

---

## 第 18 章完成

- 回到目录：[第 18 章：Trait Object Limitations](./README.md)
