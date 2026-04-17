# 4. Option

> 类型：**Study note**
> 关键词：`Option<T>`、`Some`、`None`
> 上一篇：[3. Enum](./3-Enum.md)
> 下一篇：[5. Result](./5-Result.md)

## 一分钟结论

- `Option<T>` 表示“有值”或“没值”
- 它把“可能为空”这件事写进了类型系统
- `Some(value)` 和 `None` 都必须被正视

## 证据来源

- 对应模块：[topic_04_option_type.rs](../../chapters/chapter03/src/topic_04_option_type.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
value inside Some = Rust
```

## 定义

`Option<T>` 是标准库里的一个 enum，本质上有两个分支：

- `Some(T)`
- `None`

## 作用

- 显式表达“这个值可能不存在”
- 避免空指针式的隐式约定
- 逼迫调用方处理缺失值情形

## 原理

示例：

```rust
let maybe_name: Option<&str> = Some("Rust");
```

这里类型本身就在说：

- 这个变量可能装着一个 `&str`
- 也可能什么都没有

处理时最基本的方式是：

```rust
match maybe_name {
    Some(name) => println!("{name}"),
    None => println!("no value"),
}
```

## 最小示例

```rust
fn main() {
    let maybe_name: Option<&str> = Some("Rust");

    match maybe_name {
        Some(name) => println!("{}", name),
        None => println!("no value"),
    }
}
```

## 注意点

### 1. `Option<T>` 不是异常系统

它表达的是“值是否存在”，不是“为什么失败了”。

### 2. `Some` / `None` 都是正常情况

不要把 `None` 当成奇怪的异常分支，它常常就是业务的一部分。

### 3. `Option<T>` 和 `T` 不是同一层类型

你不能把 `Option<i32>` 当成 `i32` 直接用。

## 常见错误

### ❌ 错误 1：把 `Option<T>` 当普通 `T`

这会导致你总想跳过分支处理。

### ❌ 错误 2：一看到 `Option` 就本能 `unwrap`

这样会把类型系统给你的保护直接丢掉。

### ❌ 错误 3：把“不存在”和“出错”混为一谈

前者更适合 `Option`，后者更适合 `Result`。

## 我的理解

- `Option<T>` 的价值不在语法，而在于它逼着你把“没值”当成一等公民
- 这也是 Rust 很多 API 让人一开始觉得“啰嗦”，但后面更稳的原因

## 下一步

下一篇看 `Result<T, E>`。它和 `Option<T>` 很像，但承担的是另一类语义。

- 继续阅读：[5. Result](./5-Result.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
