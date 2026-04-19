# 第 3 章：Custom and Library Provided

这一章开始进入“用类型建模”的阶段。前两章还在讨论值、所有权和借用；这一章开始回答：

- 什么时候该自己定义类型
- 什么时候直接用标准库类型
- 为什么 `Option` / `Result` / `HashMap` 这么常见
- 方法、模式匹配、方法链为什么会互相影响

## 本章目标

- 学会用 `struct` 和 `enum` 表达业务状态
- 分清 `Option` 和 `Result` 各解决什么问题
- 能看懂 `HashMap`、模式匹配和解构的基本用法
- 对“方法链为什么有时突然链不下去”建立直觉

## 推荐阅读顺序

1. [Struct 基础](./1-Struct基础.md)（含 **`#[derive(Debug)]`** 专节：是什么 / 为什么 / 怎么样）
2. [为 Struct 添加功能](./2-为Struct添加功能.md)
3. [Enum](./3-Enum.md)
4. [Option](./4-Option.md)
5. [Result](./5-Result.md)
6. [HashMap](./6-HashMap.md)
7. [模式匹配上下文](./7-模式匹配上下文.md)
8. [解构的 Struct 参数](./8-解构的Struct参数.md)
9. [引用的转换与赋值](./9-引用的转换与赋值.md)
10. [方法链的约束](./10-方法链的约束.md)
11. [`self` 与 `Self`（是什么 / 为什么 / 怎么样）](./11-self与Self.md)

## 对应代码与运行方式

- Cargo package：`chapter03`
- 运行方式：`cargo run -p chapter03`
- 章节入口：[chapters/chapter03/src/main.rs](../../chapters/chapter03/src/main.rs)
- 练习模块：[lab.rs](../../chapters/chapter03/src/lab.rs)

## 本章关键输出

```text
username = bill
active = true
selling price = 5123
```

```text
当前状态: 通行
value inside Some = Rust
parsed successfully = 42
rust score = Some(100)
```

```text
once a method returns Result, chaining shape changes => Ok(TextBuilder { value: "Rust Language" })
```

`topic_11_self_explained`（文档第 11 篇对照）：

```text
== self and Self (doc 11) ==
peek (等价于 Counter::peek(&c)): 0
after bump: 1
into_inner -> 1
reorder 消费原 Acc，返回新 Acc: Acc { n: 15 }
```

## 本章常见误区

- 以为 `struct` 只是“字段打包器”
- 以为 `enum` 只是“几个常量”
- 以为 `Option` 和 `Result` 只是语法噪音
- 以为方法链只是“写起来更顺”的语法糖

这一章真正要建立的是：**类型本身就在表达约束和语义**。
