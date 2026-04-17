# 4. Super Trait

- Cargo package: `chapter06`
- Run chapter: `cargo run -p chapter06`
- Chapter entry: `chapters/chapter06/src/main.rs`
- Reference module: `chapters/chapter06/src/topic_04_super_traits.rs`
- Chapter lab: `chapters/chapter06/src/lab.rs`

## 定义

Super trait 是“依赖别的 trait 的 trait”。例如 `trait ShapeCard: Draw + Named`，表示想实现 `ShapeCard`，必须先实现 `Draw` 和 `Named`。

## 作用

- 组合多个基础能力
- 避免重复声明方法
- 让高层接口复用底层接口

## 原理

super trait 不会自动生成底层 trait 的实现，它只是声明依赖关系。只有当类型已经满足这些前置 trait，编译器才允许它实现更高层的 trait。

## 最小示例

```rust
trait Draw {
    fn draw(&self) -> String;
}

trait Named {
    fn name(&self) -> &'static str;
}

trait ShapeCard: Draw + Named {
    fn summary(&self) -> String {
        format!("{} => {}", self.name(), self.draw())
    }
}
```

## 注意点

- super trait 更像接口组合，不是传统继承树
- 它适合表达“高层能力建立在低层能力之上”
- 如果组合关系很松散，硬绑成 super trait 反而会让接口变重

## 常见错误

- 以为实现 super trait 会自动帮你实现底层 trait
- 把很多无关 trait 强行组合，导致实现成本升高
- 误把 super trait 当成面向对象继承

## 我的理解

super trait 的价值在于“复用约束关系”。当多个能力总是一起出现时，把它们汇总成一个高层入口会更清晰。

## 下一步

继续看 [Trait Object](./5-TraitObject.md)，对比“编译期抽象”和“运行时多态”。
