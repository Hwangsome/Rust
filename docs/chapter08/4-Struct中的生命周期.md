# 4. Struct 中的生命周期

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/topic_04_lifetimes_in_structs.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_04_lifetimes_in_structs.rs` 增加了 `Comparison<'a>` 演示多个引用字段共享同一个 `'a`，并解释 struct 实例不能活得比借用的数据久（若尝试则 E0597）。

```text
first slice sum   = 6
updated slice sum = 15
longer of two = world!
```

## 定义

当 struct 字段里存的是引用，而不是拥有值时，struct 本身就必须带生命周期参数。

## 作用

- 把“这个 struct 借来的数据至少要活多久”写进类型定义
- 避免 struct 持有悬垂引用
- 让方法签名延续同一套生命周期关系

## 原理

`struct ArrayProcessor<'a> { data: &'a [i32] }` 的意思是：这个 struct 并不拥有 `data`，它只是借用一段切片，所以底层切片必须至少活过 `'a`。

## 最小示例

```rust
struct ArrayProcessor<'a> {
    data: &'a [i32],
}
```

## 注意点

- 带引用字段的 struct 往往不是“长期持有型”对象
- 如果你发现生命周期很难写，可能说明该 struct 更适合直接拥有数据
- `impl<'a>` 和 `struct<'a>` 通常要配套出现

## 常见错误

- 想在 struct 里长期保存局部变量引用
- 把生命周期问题误认为语法问题
- 不判断是否真的该用引用字段

## 我的理解

很多生命周期难题的源头其实是“我到底该借用还是拥有”。只要把这个问题想清楚，struct 的设计会简单很多。

## 下一步

继续看 [Box 智能指针](./5-Box智能指针.md)，开始进入所有权工具箱。
