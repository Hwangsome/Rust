# 6. Box 的典型用法

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/box_pointer_usecases.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 定义

这一节不是重新定义 Box，而是总结它在工程里最常出现的几个使用点。

## 作用

- 放置较大的值，避免在栈上直接展开
- 承载 trait object
- 让 API 清楚表达“这里只有一个 owner”

## 原理

Box 把值移到堆上后，调用者只在栈上保留一个固定大小的指针。这样一来，不管真实值多大，当前变量和字段的大小都保持稳定。

## 最小示例

```rust
let boxed_numbers = Box::new([0_u8; 32]);

let stores: Vec<Box<dyn Storage>> = vec![
    Box::new(DiskStorage),
    Box::new(CloudStorage),
];
```

## 注意点

- “值很大”不是唯一判断条件，接口语义也很重要
- trait object 常和 Box 一起出现，但两者职责不同
- 如果后续需要多个 owner，就要考虑 Rc / Arc

## 常见错误

- 把 Box 当成性能优化默认选项
- 看见 `Box<dyn Trait>` 就忽略动态分发成本
- 需要共享时仍然只围着 Box 转

## 我的理解

Box 常见，不是因为它高级，而是因为它把“堆分配 + 单一所有权”这个常见组合表达得很直接。

## 下一步

继续看 [Rc 智能指针](./7-Rc智能指针.md)，进入“共享所有权”场景。
