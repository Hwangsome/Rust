# 第 7 章：Functional Programming Aspects

这一章开始把 Rust 里偏“函数式”的能力串起来看。闭包、函数指针、迭代器、组合子乍一看像四五个分散主题，实际上它们围绕的是同一件事：**把行为当成值，把数据处理写成流水线**。

## 本章目标

- 理解闭包和普通函数的边界
- 区分 `fn`、`Fn`、`FnMut`、`FnOnce` 的使用语境
- 建立 Iterator / IntoIterator 的基本模型
- 看清 `iter`、`iter_mut`、`into_iter` 的所有权差异
- 学会用组合子写最小可读的数据变换链

## 推荐阅读顺序

1. [闭包](./1-闭包.md)
2. [函数指针](./2-函数指针.md)
3. [迭代器](./3-迭代器.md)
4. [IntoIterator](./4-IntoIterator.md)
5. [遍历集合](./5-遍历集合.md)
6. [组合子](./6-组合子.md)
7. [遍历 Option](./7-遍历Option.md)

## 对应代码与实验

- Cargo package：`chapter07`
- 运行方式：`cargo run -p chapter07`
- 章节入口：[chapters/chapter07/src/main.rs](../../chapters/chapter07/src/main.rs)
- 练习模块：[chapters/chapter07/src/lab.rs](../../chapters/chapter07/src/lab.rs)

主题模块：

- [topic_01_closures.rs](../../chapters/chapter07/src/topic_01_closures.rs)
- [topic_02_function_pointers.rs](../../chapters/chapter07/src/topic_02_function_pointers.rs)
- [topic_03_iterators.rs](../../chapters/chapter07/src/topic_03_iterators.rs)
- [topic_04_into_iter.rs](../../chapters/chapter07/src/topic_04_into_iter.rs)
- [topic_05_iterating_through_collections.rs](../../chapters/chapter07/src/topic_05_iterating_through_collections.rs)
- [topic_06_combinators.rs](../../chapters/chapter07/src/topic_06_combinators.rs)
- [topic_07_iterating_through_option.rs](../../chapters/chapter07/src/topic_07_iterating_through_option.rs)

## 这一章的主线

- 闭包回答“行为能不能像值一样传来传去”
- Iterator 回答“数据能不能按统一协议逐个产出”
- 组合子回答“这些步骤能不能串起来形成流水线”

理解了这三件事，后面读标准库、集合 API 和错误处理链式写法会顺很多。
