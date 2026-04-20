# 第 6 章：Generics（泛型）

在「类型怎么声明、模块怎么组织」之后，本章只聚焦一件事：**如何把同一套逻辑参数化成 `Vec<T>`、`fn foo<T>(...)` 这类写法**，以及编译期 **单态化** 带来的零成本抽象。

**Trait**（接口、多态、关联类型）已拆到 **[第 7 章：Traits](../chapter07/README.md)**，建议先完成本章再进入第 7 章。

## 本章目标

- 理解类型参数 `<T>` 与单态化（monomorphization）
- 能读懂 `impl<T> ...`、`impl ConcreteType` 针对具体类型的额外实现
- 知道何时需要 **turbofish** `::<T>` 补全推断
- 为第 7 章的 trait bound 打基础

## 推荐阅读顺序

1. [泛型](./1-泛型.md)

## 对应代码与实验

- Cargo package：`chapter06`
- 运行方式：`cargo run -p chapter06`
- 章节入口：[chapters/chapter06/src/main.rs](../../chapters/chapter06/src/main.rs)
- 练习模块：[chapters/chapter06/src/lab.rs](../../chapters/chapter06/src/lab.rs)

主题模块：

- [topic_01_generics.rs](../../chapters/chapter06/src/topic_01_generics.rs)

## 与第 7 章的分工

| 第 6 章 | 第 7 章 |
|--------|--------|
| 泛型语法、单态化、`Vec<T>` 与 `collect` 等前置 | `trait`、`impl`、trait bound、`dyn Trait`、关联类型 |

## 本章最容易混淆的地方

- 泛型不是「任何 `T` 都行」，常要配合 trait bound（第 7 章系统讲）
- `impl Point<i32, i32>` 只对这一种具体类型多实现方法——不要和 C++ 模板偏特化混为一谈
