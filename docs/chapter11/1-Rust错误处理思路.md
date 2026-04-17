# 1. Rust 错误处理思路

- Cargo package: `chapter11`
- Run chapter: `cargo run -p chapter11`
- Chapter entry: `chapters/chapter11/src/main.rs`
- Reference module: `chapters/chapter11/src/rust_error_handling_approach.rs`
- Chapter lab: `chapters/chapter11/src/lab.rs`

## 定义

Rust 把错误处理显式放进类型和控制流里，而不是默认依赖异常机制。

## 作用

- 让失败路径可见
- 区分不可恢复错误和可恢复错误
- 逼着调用者明确处理边界

## 原理

一般来说：

- `panic!` 代表程序不该继续的情况
- `Result<T, E>` 代表可能失败但可恢复
- `Option<T>` 代表值可能不存在，但这不是错误

## 最小示例

```rust
match File::open("missing.txt") {
    Ok(file) => { /* use file */ }
    Err(error) => { /* handle error */ }
}
```

## 注意点

- `panic!` 不是普通业务分支
- 可恢复错误要优先考虑 `Result`
- “没有值”不一定意味着“出错了”

## 常见错误

- 把所有失败都写成 panic
- 看见 `None` 就当成异常
- 想让错误处理完全隐藏在函数内部

## 我的理解

Rust 的错误处理风格像是在不断追问：这个失败到底是什么语义。语义清楚了，类型通常也就跟着清楚了。

## 下一步

继续看 [传播错误](./2-传播错误.md)，把失败从“就地处理”推进到“向上返回”。
