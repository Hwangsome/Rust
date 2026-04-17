# 3. Trait 约束

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/topic_03_trait_bounds.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

Trait 约束是给泛型参数加上的行为条件，例如 `T: Shape + Debug`。它表示这个泛型不是“随便什么类型都能来”，而是必须满足指定 trait。

## 作用

- 让泛型函数可以安全调用 trait 方法
- 把抽象代码的使用边界写清楚
- 提前把不满足条件的类型挡在编译期

## 原理

泛型只说明“这里类型可以变化”，trait bound 进一步说明“变化范围仍然受约束”。编译器会在实例化泛型时检查该类型是否真的实现了这些 trait。

## 最小示例

```rust
fn shape_summary<T>(shape: &T)
where
    T: Shape + Debug,
{
    println!("{:?} => {}", shape, shape.area());
}
```

## 注意点

- `impl Trait` 常用于简化参数或返回值写法
- 返回 `impl Trait` 时，函数内部仍然必须返回单一具体类型
- 约束越多，复用边界越窄；约束太少，又可能无法表达真实需求

## 常见错误

- 以为 `impl Trait` 返回值可以分支返回不同具体类型
- 在函数签名里堆太多约束，导致可读性下降
- 泛型函数里需要的方法没写进约束，结果编译不过

## 我的理解

trait bound 的价值在于把“抽象”控制在合理范围内。它不是限制开发速度，而是防止泛型失控。

## 下一步

继续看 [Super Trait](./4-SuperTrait.md)，理解 trait 之间也能形成依赖关系。
