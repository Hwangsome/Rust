# 第 7 章：Traits（行为抽象与多态）

本章承接 **[第 6 章：泛型](../chapter06/README.md)**：泛型解决「类型参数化」，**trait** 解决「行为如何抽象、如何约束、何时用静态分发 / 动态分发」。

## 本章目标

- 会写 `trait` / `impl Trait for Type`、默认方法、孤儿规则的基本直觉
- 区分 trait bound、`super trait`、`dyn Trait` 与关联类型
- 理解 object-safe 与 trait object 的运行期成本
- 能在关联类型与泛型参数之间做初步取舍

## 推荐阅读顺序

1. [Trait](./1-Trait.md)
2. [Trait 约束](./2-Trait约束.md)
3. [Super Trait](./3-SuperTrait.md)
4. [Trait Object](./4-TraitObject.md)
5. [Derive 与 Marker Trait](./5-Derive与MarkerTrait.md)
6. [Trait 中的关联类型](./6-Trait中的关联类型.md)
7. [关联类型与泛型参数的取舍](./7-关联类型与泛型参数的取舍.md)

## 对应代码与实验

- Cargo package：`chapter07`
- 运行方式：`cargo run -p chapter07`
- 章节入口：[chapters/chapter07/src/main.rs](../../chapters/chapter07/src/main.rs)
- 练习模块：[chapters/chapter07/src/lab.rs](../../chapters/chapter07/src/lab.rs)

主题模块：

- [topic_01_traits.rs](../../chapters/chapter07/src/topic_01_traits.rs)
- [topic_02_trait_bounds.rs](../../chapters/chapter07/src/topic_02_trait_bounds.rs)
- [topic_03_super_traits.rs](../../chapters/chapter07/src/topic_03_super_traits.rs)
- [topic_04_trait_objects.rs](../../chapters/chapter07/src/topic_04_trait_objects.rs)
- [topic_05_derived_and_marker_traits.rs](../../chapters/chapter07/src/topic_05_derived_and_marker_traits.rs)
- [topic_06_associated_types_in_traits.rs](../../chapters/chapter07/src/topic_06_associated_types_in_traits.rs)
- [topic_07_choosing_associated_vs_generic_type.rs](../../chapters/chapter07/src/topic_07_choosing_associated_vs_generic_type.rs)

## 本章主线

- trait 回答「行为能不能像合同一样显式声明」
- trait object 回答「何时需要 `dyn` 与虚表分发」
- 关联类型回答「实现里『输出类型』只有一种时怎么写更干净」
