# 5. Result

> 类型：**Study note**
> 关键词：`Result<T, E>`、`Ok`、`Err`
> 上一篇：[4. Option](./4-Option.md)
> 下一篇：[6. HashMap](./6-HashMap.md)

## 一分钟结论

- `Result<T, E>` 表示“成功得到 `T`，失败得到 `E`”
- 它是 Rust 错误处理的核心类型之一
- 和 `Option<T>` 相比，它多了“失败原因”

## 证据来源

- 对应模块：[result_type.rs](../../chapters/chapter03/src/result_type.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
parsed successfully = 42
```

## 定义

`Result<T, E>` 也是一个 enum，最核心的两个分支：

- `Ok(T)`
- `Err(E)`

## 作用

- 显式表达操作成功或失败
- 把失败原因保留下来
- 让调用方必须决定如何处理错误

## 原理

当前示例：

```rust
let parsed: Result<i32, _> = "42".parse();
```

含义：

- 如果解析成功，拿到整数
- 如果失败，拿到错误值

处理方式：

```rust
match parsed {
    Ok(value) => println!("{value}"),
    Err(error) => println!("{error}"),
}
```

## 最小示例

```rust
fn main() {
    let parsed: Result<i32, _> = "42".parse();

    match parsed {
        Ok(value) => println!("ok = {}", value),
        Err(error) => println!("err = {}", error),
    }
}
```

## 注意点

### 1. `Result` 和 `Option` 的核心区别是“有没有错误信息”

- `Option`：有 / 没有
- `Result`：成功 / 失败，且失败有原因

### 2. `Err` 不是程序崩溃

它只是一个普通值分支，是否 panic 是调用方后续的选择。

### 3. 解析、IO、网络、文件系统等场景都大量依赖 `Result`

这是 Rust 工程代码里出镜率最高的类型之一。

## 常见错误

### ❌ 错误 1：把 `Err` 当成异常抛出式思维

在 Rust 里，它首先是类型系统中的一个分支。

### ❌ 错误 2：把 `Option` 和 `Result` 混着用

要先想清楚：这里缺的是“值”，还是“操作成功信息”。

### ❌ 错误 3：只关心成功路径

工程代码真正稳不稳，往往取决于失败路径写得怎么样。

## 我的理解

- `Option` 把“没值”显式化
- `Result` 把“失败”显式化
- 这两者组合起来，已经覆盖了大量控制流与错误处理场景

## 下一步

下一篇看标准库提供的常见集合：`HashMap`。

- 继续阅读：[6. HashMap](./6-HashMap.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
