# 8. 解构的 Struct 参数

> 类型：**Study note**
> 关键词：struct pattern、destructure、function parameter
> 上一篇：[7. 模式匹配上下文](./7-模式匹配上下文.md)
> 下一篇：[9. 引用的转换与赋值](./9-引用的转换与赋值.md)

## 一分钟结论

- 函数参数位置本身就是模式位置
- 所以 struct 也可以在参数里直接解构
- 这样做能减少样板，但也会让“函数拿到的到底是什么”更值得留意

## 证据来源

- 对应模块：[topic_08_destructured_struct_parameters.rs](../../chapters/chapter03/src/topic_08_destructured_struct_parameters.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
point is on y-axis, y = 7
coords from destructured param = (5, 6)
```

## 定义

解构的 struct 参数，就是在函数参数列表中直接写 struct 模式：

```rust
fn print_coord(Point { x, y }: Point) { ... }
```

## 作用

- 直接把需要的字段绑定出来
- 少写一层 `point.x` / `point.y`
- 强调“我只关心这几个字段”

## 原理

参数位置本质上不是“只能放变量名”，而是可以放模式。

所以这两种写法在语义上有联系：

```rust
fn f(point: Point) {
    let Point { x, y } = point;
}
```

```rust
fn f(Point { x, y }: Point) { ... }
```

当前代码把第二种直接用了出来。

## 最小示例

```rust
struct Point {
    x: i32,
    y: i32,
}

fn print_coord(Point { x, y }: Point) {
    println!("({}, {})", x, y);
}
```

## 注意点

### 1. 参数位置的解构不是魔法

它只是把解构提前到了函数签名里。

### 2. 解构的语义仍然受所有权影响

如果字段类型不是 `Copy`，就要特别关注它是否被移动。

### 3. 不是所有场景都适合直接解构

如果函数体很复杂，先收一个命名明确的参数，有时更好读。

## 常见错误

### ❌ 错误 1：把参数解构当成“只是更酷的写法”

它其实在改变你读取和绑定字段的方式。

### ❌ 错误 2：忘了参数本身也遵守所有权规则

解构发生在参数列表，不代表 move / borrow 规则失效。

### ❌ 错误 3：为了简洁牺牲可读性

如果解构模式太复杂，拆回函数体里反而更清楚。

## 我的理解

- 这一节真正想表达的是：模式匹配已经深入到 Rust 的很多语法位置
- 参数解构只是其中一个非常直观的例子

## 下一步

下一篇回到引用本身，看 reborrow 和引用赋值这些容易混淆的点。

- 继续阅读：[9. 引用的转换与赋值](./9-引用的转换与赋值.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
