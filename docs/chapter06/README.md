# 第 6 章：Flexibility and Abstraction with Generics and Traits

这一章开始真正进入 Rust 的抽象层。前面几章更像“值怎么流动、类型怎么声明、模块怎么组织”，这里开始回答另一类问题：

- 一套逻辑如何复用到多种类型上
- 一个类型怎样声明“我支持哪些能力”
- 泛型、trait、trait bound、trait object 分别解决什么问题
- 什么时候该用关联类型，什么时候该用泛型参数

## 本章目标

- 建立“泛型负责参数化、trait 负责行为约束”的基本模型
- 理解静态分发和动态分发的边界
- 区分 trait bound、super trait、trait object、关联类型的使用场景
- 为后面学习迭代器、错误处理和更复杂 API 打基础

## 推荐阅读顺序

1. [泛型](./1-泛型.md)
2. [Trait](./2-Trait.md)
3. [Trait 约束](./3-Trait约束.md)
4. [Super Trait](./4-SuperTrait.md)
5. [Trait Object](./5-TraitObject.md)
6. [Derive 与 Marker Trait](./6-Derive与MarkerTrait.md)
7. [Trait 中的关联类型](./7-Trait中的关联类型.md)
8. [关联类型与泛型参数的取舍](./8-关联类型与泛型参数的取舍.md)

## 对应代码与实验

- Cargo package：`chapter06`
- 运行方式：`cargo run -p chapter06`
- 章节入口：[chapters/chapter06/src/main.rs](../../chapters/chapter06/src/main.rs)
- 练习模块：[chapters/chapter06/src/lab.rs](../../chapters/chapter06/src/lab.rs)

主题模块：

- [generics.rs](../../chapters/chapter06/src/generics.rs)
- [traits.rs](../../chapters/chapter06/src/traits.rs)
- [trait_bounds.rs](../../chapters/chapter06/src/trait_bounds.rs)
- [super_traits.rs](../../chapters/chapter06/src/super_traits.rs)
- [trait_objects.rs](../../chapters/chapter06/src/trait_objects.rs)
- [derived_and_marker_traits.rs](../../chapters/chapter06/src/derived_and_marker_traits.rs)
- [associated_types_in_traits.rs](../../chapters/chapter06/src/associated_types_in_traits.rs)
- [choosing_associated_vs_generic_type.rs](../../chapters/chapter06/src/choosing_associated_vs_generic_type.rs)

## 这一章最容易混淆的地方

- 泛型不是“任何类型都行”，它经常要配合 trait bound
- trait 不是对象本身，它更像“行为合同”
- trait object 和泛型都能做多态，但一个偏编译期，一个偏运行期
- 关联类型不是泛型的替代品，而是“这个实现里只有一种合理输出类型”时的更清晰表达
