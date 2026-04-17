# 4. Primitive 数据类型：整数、浮点、布尔、字符

> 类型：**Study note**
> 关键词：integer、float、bool、char、overflow
> 上一篇：[3. 变量：不可变、`mut`、遮蔽](./3-变量.md)
> 下一篇：[5. Compound 数据类型：元组与数组](./5-Compound数据类型.md)

## 一分钟结论

- 这一章说的 Primitive 数据类型，Rust 官方更常叫 **scalar types**
- 它们一次只表示**一个值**
- 最基础的四类是：**整数、浮点、布尔、字符**
- `char` 不是 1 字节 ASCII，它表示 **Unicode scalar value**
- 整数溢出在 debug 下会直接 panic，这个坑非常值得早记住

## 对应代码

- Cargo package: `chapter01`
- Run chapter: `cargo run -p chapter01`
- Chapter entry: [chapters/chapter01/src/main.rs](../../chapters/chapter01/src/main.rs)
- Reference module: [chapters/chapter01/src/topic_03_primitive_data_types.rs](../../chapters/chapter01/src/topic_03_primitive_data_types.rs)
- Chapter lab: [chapters/chapter01/src/lab.rs](../../chapters/chapter01/src/lab.rs)

当前仓库采用“每章一个 Cargo package”的结构，所以本节不是独立 binary，而是 `chapter01` 中的一个主题模块。

## 证据来源

### 本次实验

`u8` 正常取值：

```rust
fn main() {
    let guess: u8 = 255;
    println!("{}", guess);
}
```

运行结果：

```text
255
```

`char` 可以保存 Unicode 字符：

```rust
fn main() {
    let c = '中';
    println!("{}", c);
}
```

运行结果：

```text
中
```

`u8` 溢出：

```rust
fn main() {
    let mut x: u8 = 255;
    x += 1;
    println!("{}", x);
}
```

运行结果节选：

```text
thread 'main' panicked
attempt to add with overflow
```

## 定义

Primitive 数据类型表示**单个值**。在 Rust 入门语境里，可以先把它理解成四组：

| 类别 | 示例 | 用途 |
| --- | --- | --- |
| 整数 | `i32`、`u8` | 计数、索引、数值计算 |
| 浮点 | `f32`、`f64` | 带小数的数值 |
| 布尔 | `bool` | 条件判断 |
| 字符 | `char` | 单个 Unicode 字符 |

## 作用

这一节不是让你死记所有类型名，而是先建立三个判断：

- 当前值是不是“单个值”
- 这个值有没有明确范围或符号要求
- 这个值会不会在边界上出问题，比如溢出

## 原理

### 整数

整数类型分两类：

- `i` 开头：有符号整数，例如 `i32`
- `u` 开头：无符号整数，例如 `u8`

数字表示位宽，例如 `u8` 表示 8 位无符号整数，范围是 `0..=255`。

### 浮点

浮点数最常见的是：

- `f32`
- `f64`

入门阶段通常直接优先使用 `f64`。

### 布尔

`bool` 只有两个值：

- `true`
- `false`

它经常出现在 `if`、`while` 之类的条件判断里。

### 字符

`char` 表示一个 Unicode scalar value，不等于“单字节字符”。

所以：

- `'a'` 是 `char`
- `'中'` 也是 `char`

## 最小示例

```rust
fn main() {
    let integer_value: i32 = -42;
    let unsigned_value: u8 = 255;
    let float_value: f64 = 3.1415;
    let is_rust_fun: bool = true;
    let chinese_char: char = '中';

    println!("{integer_value}");
    println!("{unsigned_value}");
    println!("{float_value}");
    println!("{is_rust_fun}");
    println!("{chinese_char}");
}
```

## 常见坑

### ❌ 以为 `char` 只能放 ASCII

不对。Rust 的 `char` 可以表示单个 Unicode 字符。

### ❌ 忽视整数溢出

在 debug 构建里，整数溢出会直接 panic。  
这是 Rust 明确帮你暴露边界问题的一种方式。

## 我的理解

这一节最重要的不是背全类型表，而是先形成“值的形状”意识：

- 一个值如果只是单个数字、真假或字符，它通常属于 primitive / scalar 类型
- 选类型时，不只是写得过编译，还要考虑范围和语义

## 下一步

- 继续阅读：[5. Compound 数据类型：元组与数组](./5-Compound数据类型.md)
- 回到目录：[第 1 章：Quick Startup](./README.md)
