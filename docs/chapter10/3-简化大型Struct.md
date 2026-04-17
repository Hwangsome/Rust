# 3. 简化大型 Struct

- Cargo package: `chapter10`
- Run chapter: `cargo run -p chapter10`
- Chapter entry: `chapters/chapter10/src/main.rs`
- Reference module: `chapters/chapter10/src/simplifying_structures.rs`
- Chapter lab: `chapters/chapter10/src/lab.rs`

## 定义

简化大型 struct 指的是把职责混杂、字段过多的结构拆成几个更小、更有边界的子结构。

## 作用

- 提高可读性
- 降低借用冲突概率
- 让函数只依赖自己真正需要的那部分数据

## 原理

如果一个函数只关心 `meta`，另一个函数只关心 `stats`，那把它们拆成两个字段后，编译器就能更容易判断“这两次借用互不冲突”。

## 最小示例

```rust
struct DocumentState {
    meta: Meta,
    stats: Stats,
}
```

## 注意点

- 拆分的前提是边界真实存在，不是为了绕编译器而硬拆
- 结构更清晰时，借用规则往往会顺手变简单
- 这是建模收益，不只是语法收益

## 常见错误

- 大 struct 什么都装，最后每个函数都拿整块数据
- 为了临时绕过借用冲突而做没有语义的拆分
- 忽略不同字段是否真的属于同一职责

## 我的理解

这节最有价值的点是：borrow checker 报错很多时候不是“Rust 太严格”，而是数据结构的边界确实还没设计好。

## 下一步

回到 [第 10 章目录](./README.md)，然后进入 [第 11 章：Error Handling](../chapter11/README.md)。
