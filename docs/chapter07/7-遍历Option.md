# 7. 遍历 Option

- Cargo package: `chapter07`
- Run chapter: `cargo run -p chapter07`
- Chapter entry: `chapters/chapter07/src/main.rs`
- Reference module: `chapters/chapter07/src/topic_07_iterating_through_option.rs`
- Chapter lab: `chapters/chapter07/src/lab.rs`

## 定义

`Option<T>` 可以被看作一个“长度为 0 或 1 的小集合”。因此它既能转换成迭代器，也能自然接到很多迭代器组合子后面。

## 作用

- 统一处理“可选值”和“普通集合”
- 避免为 `Some` / `None` 单独写太多分支
- 让流水线式写法覆盖更多场景

## 原理

`Some(value)` 在迭代器视角下会产出一个元素，`None` 则不产出任何元素。于是像 `extend`、`flatten` 这样的操作都能顺理成章地工作。

## 最小示例

```rust
let mut products = vec!["keyboard", "mouse"];
products.extend(Some("monitor"));
products.extend(None::<&str>);
```

## 注意点

- 这不是说 Option 真的是集合，只是它支持类似的迭代接口
- `flatten` 很适合消除 `Iterator<Item = Option<T>>` 里的空值
- 当语义本身是“缺失值”，Option 仍然比空集合更直接

## 常见错误

- 只把 Option 当成 `match` 的输入，不知道它能接迭代器
- 过度抽象，把简单的 `if let` 场景写得太绕
- 混淆“没有元素”和“操作失败”

## 我的理解

这一节让我更清楚：Rust 不是把每种数据结构都割裂开来，而是尽量用 trait 把它们连进统一模型。

## 下一步

回到 [第 7 章目录](./README.md)，然后进入 [第 8 章：Memory Management Features](../chapter08/README.md)。
