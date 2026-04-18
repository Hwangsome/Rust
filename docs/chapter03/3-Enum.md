# 3. Enum

> 类型：**Study note**
> 关键词：enum、variant、state、`match`
> 上一篇：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
> 下一篇：[4. Option](./4-Option.md)

## 一分钟结论

- `enum` 用来表达“一个值只能是多种状态中的一种”
- 它特别适合建模互斥状态
- 和 `match` 搭配时最自然

## 证据来源

- 对应模块：[topic_03_enums.rs](../../chapters/chapter03/src/topic_03_enums.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
当前状态: 通行
```

## 扩展演示输出（当前代码已升级）

`topic_03_enums.rs` 现在包含三件事：
1. 简单 enum（`TrafficLight`）+ `impl` 里写 `describe` / `next` 方法
2. `match` 的**穷尽性检查**（漏一个分支 → E0004）
3. 每个分支带不同数据的 `Message` enum —— Rust 真正的 ADT 建模能力

```text
-- (1) 简单 enum + match --
当前是 Green: 通行
下一个状态: Yellow: 准备

-- (3) 每个分支携带不同数据 --
[Quit] 退出
[Move] 移动到 (10, 20)
[Write] 写入文本: hello
[ChangeColor] RGB(255, 128, 0)
```

## 定义

`enum` 是把多个可能分支组织到一个类型里的方式。

例如：

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

## 作用

- 表达互斥状态
- 让状态空间更明确
- 逼迫使用者处理不同分支

## 原理

当你写：

```rust
let light = TrafficLight::Green;
```

表示的是：

- 当前值属于 `TrafficLight`
- 但具体处于 `Green` 这个 variant

和 `match` 搭配：

```rust
match light {
    TrafficLight::Red => ...,
    TrafficLight::Yellow => ...,
    TrafficLight::Green => ...,
}
```

这让状态处理逻辑变得显式。

## 最小示例

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("停止"),
        TrafficLight::Yellow => println!("准备"),
        TrafficLight::Green => println!("通行"),
    }
}
```

## 注意点

### 1. `enum` 不是“几个常量”

它代表的是一种类型及其分支集合。

### 2. `match` 的价值不只是语法美观

它让你把每个分支的处理逻辑显式写出来。

### 3. 后续很多核心类型本质上也是 enum

例如后面马上会学到：

- `Option`
- `Result`

## 常见错误

### ❌ 错误 1：把 enum 只当作标签

它通常是在建模状态机，而不只是“多几个名字”。

### ❌ 错误 2：遗漏分支处理

Rust 借 `match` 的穷尽性检查帮你尽早发现这种问题。

### ❌ 错误 3：该用 enum 表达状态，结果拆成多个 bool

那样很容易出现非法组合状态。

## 我的理解

- `struct` 擅长表达“一个东西有哪些字段”
- `enum` 擅长表达“一个东西当前处于哪种状态”
- 这两者合起来，已经能覆盖大量业务建模场景

## 下一步

下一篇看 `Option<T>`。它是最经典的“标准库里的 enum”。

- 继续阅读：[4. Option](./4-Option.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
