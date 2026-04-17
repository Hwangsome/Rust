# 6. 分层结果类型：Option<Result<T, E>>

- Cargo package: `chapter11`
- Run chapter: `cargo run -p chapter11`
- Chapter entry: `chapters/chapter11/src/main.rs`
- Reference module: `chapters/chapter11/src/layered_outcomes_result_option_part2.rs`
- Chapter lab: `chapters/chapter11/src/lab.rs`

## 定义

`Option<Result<T, E>>` 表示：

- `None`：这次操作根本没有发生
- `Some(Ok(T))`：发生了，而且成功
- `Some(Err(E))`：发生了，但失败

## 作用

- 表达“操作是否尝试过”本身也是一层信息
- 适合可选表单字段、按条件执行的解析逻辑
- 把“没执行”和“执行失败”分开

## 原理

外层 `Option` 先回答“有没有尝试”，内层 `Result` 再回答“尝试后成没成功”。这和上一节的三态模型完全不是一回事。

## 最小示例

```rust
fn handle_user_registration(
    name: &str,
    age_input: Option<&str>,
) -> Option<Result<u32, ParseIntError>>
```

## 注意点

- 它适合“可选操作”，不适合“必执行查询”
- 设计时要先判断外层该放 `Option` 还是 `Result`
- 两层含义不要写反

## 常见错误

- 看到两层包装就以为只是写法差异
- 不分“没有执行”与“执行失败”
- 用错层级顺序，导致调用端语义很别扭

## 我的理解

这一节很能训练语义判断力。包装顺序一换，整个 API 的含义就变了。

## 下一步

继续看 [anyhow](./7-anyhow.md)，开始进入实际工程里常见的错误处理工具。
