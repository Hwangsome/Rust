# 5. Trait Object

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/trait_objects.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

Trait object 是把“实现了某个 trait 的具体值”装进一个统一的运行时类型里，例如 `Box<dyn Draw>`。

## 作用

- 在一个集合里装不同具体类型
- 以统一接口处理异构对象
- 在无法提前枚举具体类型时提供运行时多态

## 原理

泛型多态通常在编译期决定调用目标，trait object 则在运行时通过虚表分发方法调用。代价是多了一层间接调用，但换来更灵活的异构容器能力。

## 最小示例

```rust
let widgets: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: "Submit" }),
    Box::new(Label { text: "Ready" }),
];
```

## 注意点

- 只有对象安全的 trait 才能做 trait object
- 一旦进入 `dyn Trait`，很多编译期信息就被擦掉了
- trait object 常和 `Box`、`Rc` 之类的指针类型一起出现

## 常见错误

- 看到“多态”就默认用 trait object
- 忽略动态分发成本和对象安全限制
- 试图把带泛型方法的 trait 直接做成 `dyn Trait`

## 我的理解

trait object 适合“同一接口，不同实现”的运行时集合；泛型更适合“同一算法，不同类型”的编译期复用。

## 下一步

继续看 [Derive 与 Marker Trait](./6-Derive与MarkerTrait.md)，补齐 trait 体系里最常见的两个辅助概念。
