# 第 12 章：Error Handling

这一章是 Part 2 的收束点。前面学的所有权、借用、trait、迭代器，到这里都会以另一种方式重新出现：**失败怎么表达，边界怎么设计，信息要保留到什么程度**。

## 本章目标

- 建立 Rust 错误处理的分层思路
- 理解 `panic!`、`Result`、`Option` 分别适合什么语义
- 理解 `?` 的传播机制和统一错误类型要求
- 区分 `anyhow` 与 `thiserror` 的边界
- 理解错误处理会反过来影响 API 设计和方法链形状

## 推荐阅读顺序

1. [Rust 错误处理思路](./1-Rust错误处理思路.md)
2. [传播错误](./2-传播错误.md)
3. [统一多种错误类型](./3-统一多种错误类型.md)
4. [链式调用中的问号运算符](./4-链式调用中的问号运算符.md)
5. [分层结果类型：Result<Option<T>, E>](./5-分层结果类型：ResultOption.md)
6. [分层结果类型：Option<Result<T, E>>](./6-分层结果类型：OptionResult.md)
7. [anyhow](./7-anyhow.md)
8. [thiserror](./8-thiserror.md)
9. [错误处理里的方法链约束](./9-错误处理里的方法链约束.md)

## 对应代码与实验

- Cargo package：`chapter12`
- 运行方式：`cargo run -p chapter12`
- 章节入口：[chapters/chapter12/src/main.rs](../../chapters/chapter12/src/main.rs)
- 练习模块：[chapters/chapter12/src/lab.rs](../../chapters/chapter12/src/lab.rs)

主题模块：

- [topic_01_rust_error_handling_approach.rs](../../chapters/chapter12/src/topic_01_rust_error_handling_approach.rs)
- [topic_02_propagating_errors.rs](../../chapters/chapter12/src/topic_02_propagating_errors.rs)
- [topic_03_multiple_error_types.rs](../../chapters/chapter12/src/topic_03_multiple_error_types.rs)
- [topic_04_chaining_question_marks.rs](../../chapters/chapter12/src/topic_04_chaining_question_marks.rs)
- [topic_05_layered_outcomes_result_option_part1.rs](../../chapters/chapter12/src/topic_05_layered_outcomes_result_option_part1.rs)
- [topic_06_layered_outcomes_result_option_part2.rs](../../chapters/chapter12/src/topic_06_layered_outcomes_result_option_part2.rs)
- [topic_07_anyhow_example.rs](../../chapters/chapter12/src/topic_07_anyhow_example.rs)
- [topic_08_thiserror_example.rs](../../chapters/chapter12/src/topic_08_thiserror_example.rs)
- [topic_09_method_chaining_constraints.rs](../../chapters/chapter12/src/topic_09_method_chaining_constraints.rs)

## 本章主线

- 先区分失败的语义：是 bug、缺值、还是可恢复错误
- 再决定失败怎么往上传
- 最后才决定要不要引入 `anyhow` / `thiserror` 这类工具
