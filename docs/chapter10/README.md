# 第 10 章：Implementing Typical Data Structures

这一章是 Part 2 最能检验前面内容有没有真正吃透的一章。因为一旦开始手写链表，前面学过的 `Box`、`Rc`、`RefCell`、`Weak` 会立刻从“概念题”变成“结构题”。

## 本章目标

- 理解单向链表为什么只需要 `Box`
- 理解双向链表为什么常引出 `Rc<RefCell<T>>`
- 学会用 `take()` 移动链表头节点
- 理解引用环为什么会发生，以及 `Weak` 怎么拆掉它

## 推荐阅读顺序

1. [单向链表（基础结构）](./1-单向链表（基础结构）.md)
2. [单向链表（添加与移除）](./2-单向链表（添加与移除）.md)
3. [双向链表（前端插入）](./3-双向链表（前端插入）.md)
4. [双向链表（移除与遍历）](./4-双向链表（移除与遍历）.md)
5. [引用环](./5-引用环.md)

## 对应代码与实验

- Cargo package：`chapter10`
- 运行方式：`cargo run -p chapter10`
- 章节入口：[chapters/chapter10/src/main.rs](../../chapters/chapter10/src/main.rs)
- 练习模块：[chapters/chapter10/src/lab.rs](../../chapters/chapter10/src/lab.rs)

主题模块：

- [topic_01_singly_link_list_part1.rs](../../chapters/chapter10/src/topic_01_singly_link_list_part1.rs)
- [topic_02_singly_link_list_part2.rs](../../chapters/chapter10/src/topic_02_singly_link_list_part2.rs)
- [topic_03_doubly_link_list_part1.rs](../../chapters/chapter10/src/topic_03_doubly_link_list_part1.rs)
- [topic_04_doubly_link_list_part2.rs](../../chapters/chapter10/src/topic_04_doubly_link_list_part2.rs)
- [topic_05_reference_cycles.rs](../../chapters/chapter10/src/topic_05_reference_cycles.rs)

## 本章观察重点

- 数据结构设计和所有权设计是同一件事
- 单向结构和双向结构的复杂度不只差在“多一根指针”
- 只要出现父子互指、前后互指，就要立刻考虑引用计数与引用环
