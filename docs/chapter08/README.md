# 第 8 章：Memory Management Features

这一章把很多初学 Rust 时最容易紧张的词放到一起了：生命周期、Box、Rc、RefCell。它们看起来像四组分散语法，实际上都在回答同一组问题：

- 谁拥有值
- 值会活多久
- 谁能读它
- 谁能改它

## 本章目标

- 理解生命周期是在描述引用关系，不是在“延长内存寿命”
- 理解 Box、Rc、RefCell 各自解决的问题
- 看清 `Rc<RefCell<T>>` 为什么经常一起出现
- 为后面的链表、树形结构和错误排查打基础

## 推荐阅读顺序

1. [具体生命周期](./1-具体生命周期.md)
2. [泛型生命周期](./2-泛型生命周期.md)
3. [生命周期省略](./3-生命周期省略.md)
4. [Struct 中的生命周期](./4-Struct中的生命周期.md)
5. [Box 智能指针](./5-Box智能指针.md)
6. [Box 的典型用法](./6-Box的典型用法.md)
7. [Rc 智能指针](./7-Rc智能指针.md)
8. [RefCell](./8-RefCell.md)
9. [RefCell 示例](./9-RefCell示例.md)

## 对应代码与实验

- Cargo package：`chapter08`
- 运行方式：`cargo run -p chapter08`
- 章节入口：[chapters/chapter08/src/main.rs](../../chapters/chapter08/src/main.rs)
- 练习模块：[chapters/chapter08/src/lab.rs](../../chapters/chapter08/src/lab.rs)

主题模块：

- [topic_01_concrete_lifetimes.rs](../../chapters/chapter08/src/topic_01_concrete_lifetimes.rs)
- [topic_02_generic_lifetimes.rs](../../chapters/chapter08/src/topic_02_generic_lifetimes.rs)
- [topic_03_lifetime_elision.rs](../../chapters/chapter08/src/topic_03_lifetime_elision.rs)
- [topic_04_lifetimes_in_structs.rs](../../chapters/chapter08/src/topic_04_lifetimes_in_structs.rs)
- [topic_05_box_smart_pointer.rs](../../chapters/chapter08/src/topic_05_box_smart_pointer.rs)
- [topic_06_box_pointer_usecases.rs](../../chapters/chapter08/src/topic_06_box_pointer_usecases.rs)
- [topic_07_rc_smart_pointer.rs](../../chapters/chapter08/src/topic_07_rc_smart_pointer.rs)
- [topic_08_refcell.rs](../../chapters/chapter08/src/topic_08_refcell.rs)
- [topic_09_refcell_example.rs](../../chapters/chapter08/src/topic_09_refcell_example.rs)

## 本章提醒

- 生命周期先从“引用关系”理解，不要一上来就把它神秘化
- `Rc` 解决共享所有权，不解决可变性
- `RefCell` 解决内部可变性，但把借用检查推迟到运行时
- `Rc<RefCell<T>>` 常见，不等于总是最佳选择
