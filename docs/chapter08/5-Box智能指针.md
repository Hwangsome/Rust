# 5. Box 智能指针

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/topic_05_box_smart_pointer.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 定义

`Box<T>` 是最基础的智能指针之一，表示“这个值在堆上，由当前 owner 独占拥有”。

## 作用

- 给大对象或递归对象提供稳定大小的句柄
- 显式表达单一所有权
- 为 trait object 提供常见承载方式

## 原理

递归类型如果直接把自身作为字段嵌入，会导致“无限大小”问题。`Box<T>` 让当前结构里只保留一个固定大小的指针，而把真正的下一级数据放到堆上。

## 最小示例

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

## 注意点

- Box 不是垃圾回收器
- 它解决的是堆分配和所有权表达，不解决共享所有权
- `Box<T>` 仍然遵守普通所有权规则

## 常见错误

- 看到堆分配就联想到共享或自动回收
- 以为 Box 会绕开 move
- 需要共享时还坚持只用 Box

## 我的理解

Box 更像“把值搬到堆上并继续独占拥有”的工具。它简单，但非常关键，是后面很多指针模型的起点。

## 下一步

继续看 [Box 的典型用法](./6-Box的典型用法.md)，把 Box 放回真实场景。
