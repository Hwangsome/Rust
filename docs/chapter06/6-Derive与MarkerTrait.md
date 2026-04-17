# 6. Derive 与 Marker Trait

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/derived_and_marker_traits.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

- `derive`：让编译器自动为类型生成某些常见 trait 实现
- marker trait：没有方法，只表达“这个类型满足某组性质”

## 作用

- 减少样板代码
- 用类型系统表达约束，而不是只靠注释约定
- 把多个基础 trait 打包成更清晰的边界

## 原理

像 `Debug`、`Clone`、`Default`、`PartialEq` 这些 trait，在字段本身也满足条件时，经常可以自动推导。marker trait 则更像“标签接口”，没有行为，只负责出现在约束里。

## 最小示例

```rust
#[derive(Debug, Clone, Default, PartialEq)]
struct Config {
    retries: u8,
    verbose: bool,
}

trait Resettable: Clone + Default + PartialEq {}
```

## 注意点

- 不是所有 trait 都能 derive
- derive 出来的语义未必总符合业务含义
- marker trait 虽然没有方法，但不代表没价值

## 常见错误

- 无脑 derive 一切，结果把不该比较或不该复制的类型也加上了
- 以为 marker trait 只是“空壳”
- 忽略 derive 带来的行为含义，比如 `Clone`、`Eq`

## 我的理解

derive 解决“重复实现”，marker trait 解决“语义分组”。两者都在帮类型系统变得更可表达。

## 下一步

继续看 [Trait 中的关联类型](./7-Trait中的关联类型.md)，开始处理“trait 还可以携带什么类型信息”。
