# 2. Trait

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/traits.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

Trait 是 Rust 用来描述共享行为的机制。它不关心某个类型“是什么”，而关心“它承诺能做什么”。

## 作用

- 给不同类型建立统一接口
- 让函数按行为编程，而不是按具体类型编程
- 为泛型提供能力边界

## 原理

当一个类型 `impl Shape for Rectangle` 时，等于它向编译器声明：这个类型满足 `Shape` 的接口要求。之后所有依赖 `Shape` 的代码都可以安全调用它提供的方法。

## 最小示例

```rust
trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32 { 0 }
}
```

## 注意点

- trait 可以有“必须实现的方法”，也可以有“默认实现的方法”
- trait 本身不是对象，只有具体类型实现它之后才有可调用实例
- 设计 trait 时，要尽量围绕“能力”命名，而不是围绕某个具体类型命名

## 常见错误

- 把 trait 理解成类继承
- 以为 trait 默认实现会自动适配所有业务场景
- 在 trait 里塞入过多不相关能力，导致接口发散

## 我的理解

trait 更像一份“行为合同”。一个类型只要签了这份合同，就能进入同一组 API。Rust 的抽象感，很大一部分来自这里。

## 下一步

继续看 [Trait 约束](./3-Trait约束.md)，它会把 trait 从“类型能力描述”推进到“函数入场条件”。
