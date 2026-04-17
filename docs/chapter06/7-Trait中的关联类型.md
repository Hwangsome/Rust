# 7. Trait 中的关联类型

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/topic_07_associated_types_in_traits.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

关联类型是写在 trait 里的类型占位符，例如 `type Distance;`。它表示“每个实现都要给这个 trait 的某个类型槽位填上具体类型”。

## 作用

- 让 trait 签名更清晰
- 避免把所有类型都暴露成泛型参数
- 表达“对某个实现来说，这个输出类型只有一种合理选择”

## 原理

当 `impl DistanceThreeHours for Kmh` 时，会同时确定 `type Distance = u32`。以后这个实现里所有相关方法都统一使用这一个具体类型。

## 最小示例

```rust
trait DistanceThreeHours {
    type Distance;

    fn distance_in_three_hours(&self) -> Self::Distance;
}
```

## 注意点

- 关联类型强调“每个实现一套固定类型”
- 它常见于 `Iterator::Item` 这种场景
- 如果同一个左值类型需要支持多个右值类型，单纯用关联类型可能不够灵活

## 常见错误

- 把关联类型和泛型参数看成完全等价
- 需要多套实现时仍强行用关联类型
- 忽略关联类型带来的可读性提升

## 我的理解

关联类型让 trait 看起来更像“带有内部约定的接口”。它把实现内部稳定不变的类型关系固定下来，代码会更容易读。

## 下一步

继续看 [关联类型与泛型参数的取舍](./8-关联类型与泛型参数的取舍.md)，把这两个工具的边界彻底分开。
