# 6. 发布你的 Crate

> 类型：**Study note**
> 关键词：`cargo publish`、metadata、`--dry-run`
> 上一篇：[5. 使用外部依赖](./5-使用外部依赖.md)

## 一分钟结论

- 发布 crate 不是敲一次命令就完了
- 真正重要的是：**元数据、README、许可证、版本号、API 稳定性**
- `cargo publish --dry-run` 是发布前非常值得养成的检查动作

## 证据来源

- 对应模块：[publishing_your_crate.rs](../../chapters/chapter04/src/publishing_your_crate.rs)
- 运行章节：`cargo run -p chapter04`

关键输出：

```text
发布前通常会先检查 package metadata、README、license 和版本号。
常见流程包括 cargo package、cargo publish --dry-run、cargo publish。
```

## 定义

发布 crate，是把你的包作为可复用产物交给其他 Rust 项目消费的过程。

## 作用

- 让别人能通过 Cargo 依赖你的包
- 把内部代码变成外部可消费 API
- 把“本地项目”提升成“生态中的公共模块”

## 原理

一个最小发布流程通常是：

1. 检查 `Cargo.toml` 元数据
2. 准备 README、license、版本号
3. 运行 `cargo package`
4. 运行 `cargo publish --dry-run`
5. 最后再执行 `cargo publish`

## 最小示例

```bash
cargo package
cargo publish --dry-run
cargo publish
```

## 注意点

### 1. “能发布”不等于“适合发布”

如果 API 还在频繁变化，或者文档不足，公开发布可能只会放大维护压力。

### 2. 元数据不是装饰

README、license、description、repository 都会直接影响别人如何理解和使用你的 crate。

### 3. 发布出去就是契约

版本号一旦出去，兼容性承诺也跟着开始了。

## 常见错误

### ❌ 错误 1：跳过 `--dry-run`

这会让本可以提前发现的问题拖到正式发布时才暴露。

### ❌ 错误 2：只准备代码，不准备文档和元信息

对外发布时，文档常常比源码第一眼更重要。

### ❌ 错误 3：没有版本策略就开始发布

后面升级和兼容性会迅速变乱。

## 我的理解

- 发布 crate 本质上是在发布“接口和承诺”
- 代码只是其中一部分，文档和版本治理同样重要

## 下一步

第 4 章结束后，最自然的下一步就是测试：一旦代码组织出来，对行为验证就会变成刚需。

- 回到目录：[第 4 章：Organizing your Code](./README.md)
- 继续阅读：[第 5 章：Testing](../chapter05/README.md)
