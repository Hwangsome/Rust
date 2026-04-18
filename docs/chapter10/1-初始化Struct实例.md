# 1. 初始化 Struct 实例

- Cargo package: `chapter10`
- Run chapter: `cargo run -p chapter10`
- Chapter entry: `chapters/chapter10/src/main.rs`
- Reference module: `chapters/chapter10/src/topic_01_initializing_struct_instances.rs`
- Chapter lab: `chapters/chapter10/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_01_initializing_struct_instances.rs` 文件头注释系统对比 4 种构造方式：`#[derive(Default)]` → 自定义 `new(...) -> Result<Self, E>` 做校验 → 结构更新语法 `..Student::default()` → `unwrap_or_default()` 回退。核心观点："Rust 没有语言级构造函数——`new` 只是约定俗成的关联函数名，你可以写任意多个构造函数。"

## 定义

Rust 没有语言级构造函数，但常见做法是通过关联函数 `new()`、`Default` 和结构更新语法来完成初始化。

## 作用

- 给初始化流程加校验
- 提供默认值
- 让“只改少数字段”的创建方式更简洁

## 原理

`new()` 负责表达定制逻辑，`Default` 负责提供基线状态，`..Default::default()` 负责在已有基线之上只覆盖个别字段。

## 最小示例

```rust
#[derive(Debug, Default)]
struct Student {
    id: u8,
    age: u8,
    name: String,
}
```

## 注意点

- `new()` 不一定返回 `Self`，也可以返回 `Result<Self, E>`
- `Default` 适合存在合理默认值的类型
- 私有字段也可以在 `impl` 里被安全初始化

## 常见错误

- 把所有初始化都硬塞进一个巨大构造函数
- 没有默认值也强行实现 Default
- 为了省事跳过校验逻辑

## 我的理解

初始化方式选得好，类型的使用体验会立刻提升。很多“这个类型好不好用”的感受，其实从构造阶段就决定了。

## 下一步

继续看 [Builder 模式](./2-Builder模式.md)，处理字段开始变多的情况。
