# 第 10 章：Useful Patterns for Handling Structs

这一章不再强调新语法，而是在讨论“怎样把 struct 设计得更像工程代码”。如果前面几章解决的是“Rust 能不能写”，这里开始解决“Rust 写出来后顺不顺手、是不是容易维护”。

## 本章目标

- 理解 Rust 里常见的初始化方式
- 看懂 builder pattern 解决的真实问题
- 理解结构拆分如何改善可读性和借用关系
- 为以后设计配置对象、领域模型和状态对象打基础

## 推荐阅读顺序

1. [初始化 Struct 实例](./1-初始化Struct实例.md)
2. [Builder 模式](./2-Builder模式.md)
3. [简化大型 Struct](./3-简化大型Struct.md)

## 对应代码与实验

- Cargo package：`chapter10`
- 运行方式：`cargo run -p chapter10`
- 章节入口：[chapters/chapter10/src/main.rs](../../chapters/chapter10/src/main.rs)
- 练习模块：[chapters/chapter10/src/lab.rs](../../chapters/chapter10/src/lab.rs)

主题模块：

- [initializing_struct_instances.rs](../../chapters/chapter10/src/initializing_struct_instances.rs)
- [builder_pattern.rs](../../chapters/chapter10/src/builder_pattern.rs)
- [simplifying_structures.rs](../../chapters/chapter10/src/simplifying_structures.rs)

## 本章提醒

- 初始化方式不是审美问题，而是 API 设计问题
- builder 的目标不是“链起来更酷”，而是避免参数爆炸
- struct 拆分的收益不只在借用层面，也在建模层面
