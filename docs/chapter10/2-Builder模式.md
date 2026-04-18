# 2. Builder 模式

- Cargo package: `chapter10`
- Run chapter: `cargo run -p chapter10`
- Chapter entry: `chapters/chapter10/src/main.rs`
- Reference module: `chapters/chapter10/src/topic_02_builder_pattern.rs`
- Chapter lab: `chapters/chapter10/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_02_builder_pattern.rs` 文件头讲清了 builder 模式解决的痛点（一堆 `new_xxx` 重载）和完整结构：
- 必填字段放 `CustomerBuilder::builder(name)` 起点
- 可选字段做成链式 setter（返回 `Self`）
- `build()` 做最终校验并产出 `Customer`
- 可以改为返回 `Result<Self, E>` 让链全用 `?` 串起来

## 定义

Builder pattern 是把对象创建拆成“逐步设置字段，最后 build()”的一种设计方式。

## 作用

- 避免构造函数参数列表过长
- 区分必填字段和可选字段
- 让调用端更接近“声明式配置”

## 原理

Builder 往往先保存一个“半成品状态”，每个 setter 方法返回 builder 自身，最后由 `build()` 统一产出最终对象。

## 最小示例

```rust
let casual_user = Customer::builder("Joseph".to_string())
    .username("joe123")
    .membership(MembershipType::Casual)
    .build();
```

## 注意点

- builder 不是为了链式语法本身，而是为了初始化边界清晰
- 要明确哪些字段必须填，哪些字段可以缺省
- `build()` 里可以做最终校验

## 常见错误

- 其实字段很少，却为了“模式完整”硬上 builder
- builder 里没有任何约束，最后和直接公开字段差不多
- 设计得太复杂，让使用者还不如直接写 struct literal

## 我的理解

builder 模式解决的是“构造函数扩张”问题。尤其在配置对象和参数较多的领域模型里，它很实用。

## 下一步

继续看 [简化大型 Struct](./3-简化大型Struct.md)，理解除了 builder 之外，结构设计本身还能怎么改。
