# 1. Struct 基础

> 类型：**Study note**
> 关键词：struct、field、instance、data modeling
> 下一篇：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)

## 一分钟结论

- `struct` 用来把一组相关字段组织成一个**有名字的整体**
- 它比 tuple 更适合表达“这些字段一起构成一个实体”
- `struct` 先解决“数据怎么放”，后面 `impl` 再解决“行为怎么挂”

## 证据来源

- 对应模块：[topic_01_structs_basics.rs](../../chapters/chapter03/src/topic_01_structs_basics.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
username = bill
active = true
```

## 扩展演示输出（当前代码已升级）

`topic_01_structs_basics.rs` 现在按 5 个子场景演示：具名字段 struct → 字段简写（field init shorthand）→ 更新语法（`..old`）→ 元组结构体（强类型别名思路）→ 单元结构体。

```text
-- (1) 具名字段 struct --
username = alice
整体打印: User { username: "alice", email: "alice@example.com", active: true, login_count: 0 }

-- (2) 字段简写 --
通过字段简写构造: User { username: "bob", email: "bob@example.com", active: true, login_count: 0 }

-- (3) 更新语法 `..old` --
通过更新语法构造 alice_v2: User { username: "alice", email: "alice_v2@example.com", active: true, login_count: 100 }

-- (4) 元组结构体 --
red = (255, 0, 0)

-- (5) 单元结构体 --
单元结构体的值只有一种形态: AlwaysEqual
```

## 定义

`struct` 是 Rust 里最基础的自定义类型之一。它允许你把多个字段组合成一个更有语义的值。

## 作用

- 用字段名替代“靠位置猜含义”的数据结构
- 把相关数据聚合成一个实体
- 为后续方法、模式匹配和 API 设计打基础

## 原理

```rust
struct User {
    username: String,
    active: bool,
}
```

这个定义表达的是：

- `User` 是一个新类型
- 它由 `username` 和 `active` 两个字段组成
- 每个字段都有明确名字和类型

相较于 tuple：

- tuple 更轻量
- struct 更有语义

## 最小示例

```rust
struct User {
    username: String,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("bill"),
        active: true,
    };

    println!("{}", user.username);
    println!("{}", user.active);
}
```

## 注意点

### 1. 字段有自己的所有权语义

如果字段类型是 `String`、`Vec<T>` 这类拥有所有权的值，就仍然遵守所有权规则。

### 2. `struct` 不是“对象系统”的完整替代词

它先解决“数据建模”，行为是后面用 `impl` 补上的。

### 3. 字段名不是装饰

字段名的语义价值正是 `struct` 相比 tuple 的最大优势之一。

## 常见错误

### ❌ 错误 1：能用 tuple 就一直用 tuple

如果数据有明确语义，用 `struct` 会更清楚。

### ❌ 错误 2：把 `struct` 当成“只有数据，没有约束”

它其实是建模入口，后面的方法、构造函数和可见性都会围绕它展开。

### ❌ 错误 3：忽略字段类型本身的所有权代价

`struct` 不会屏蔽你之前学过的所有权规则。

## 我的理解

- `struct` 的价值不在于“能装多个字段”
- 而在于“给这组字段起了一个类型级别的名字”
- 一旦类型名出现，后面的 API 设计就开始有边界了

## 下一步

下一篇开始看 `impl`。也就是：有了数据结构之后，怎么把行为挂到它上面。

- 继续阅读：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
