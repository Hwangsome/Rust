# 5. Compound 数据类型：元组与数组

> 类型：**Study note**
> 关键词：tuple、array、index、out of bounds
> 上一篇：[4. Primitive 数据类型：整数、浮点、布尔、字符](./4-Primitive数据类型.md)
> 下一篇：[6. 函数与代码块](./6-函数与代码块.md)

## 一分钟结论

- Compound 数据类型用来把**多个值组合在一起**
- 这一章最重要的两个是：**tuple（元组）** 和 **array（数组）**
- **元组**可以混合类型，适合一组固定位置的值
- **数组**要求元素类型一致、长度固定
- 数组越界不是“小问题”，运行时会直接 panic

## 对应代码

- Cargo package: `chapter01`
- Run chapter: `cargo run -p chapter01`
- Chapter entry: [chapters/chapter01/src/main.rs](../../chapters/chapter01/src/main.rs)
- Reference module: [chapters/chapter01/src/compound_data_types.rs](../../chapters/chapter01/src/compound_data_types.rs)
- Chapter lab: [chapters/chapter01/src/lab.rs](../../chapters/chapter01/src/lab.rs)

当前仓库采用“每章一个 Cargo package”的结构，所以本节不是独立 binary，而是 `chapter01` 中的一个主题模块。

## 证据来源

### 本次实验

元组访问：

```rust
fn main() {
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    println!("{} {} {}", tup.0, tup.1, tup.2);
}
```

运行结果：

```text
500 6.4 z
```

数组访问：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
}
```

运行结果：

```text
1
```

数组越界：

```rust
fn main() {
    let a = [1, 2, 3];
    let idx: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", a[idx]);
}
```

运行结果节选：

```text
index out of bounds: the len is 3 but the index is 10
```

## 定义

Compound 数据类型表示“把多个值放到一个整体里”。当前最核心的两种：

| 类型 | 能否混合元素类型 | 长度是否固定 | 常见访问方式 |
| --- | --- | --- | --- |
| 元组 `tuple` | 可以 | 固定 | `.0`, `.1` 或解构 |
| 数组 `array` | 不可以 | 固定 | `[index]` |

## 作用

这一节解决的是“当一个值不够用时，怎样把多个值组织到一起”。

- 元组适合放一组位置固定、类型可能不同的数据
- 数组适合放一组类型相同、长度固定的数据

## 原理

### 元组

元组的特点是：

- 长度固定
- 每个位置的类型可以不同
- 常用在“临时打包几个值”的场景

例如：

```rust
let user_profile: (&str, i32, char) = ("alice", 20, 'A');
```

### 数组

数组的特点是：

- 元素类型必须一致
- 长度固定
- 通过索引访问

例如：

```rust
let scores: [i32; 5] = [90, 85, 88, 92, 95];
```

## 最小示例

```rust
fn main() {
    let user_profile: (&str, i32, char) = ("alice", 20, 'A');
    println!("{}", user_profile.0);

    let scores: [i32; 5] = [90, 85, 88, 92, 95];
    println!("{}", scores[0]);
}
```

## 常见坑

### ❌ 把数组当成可随意增长的容器

当前这章里的数组长度是固定的。  
如果你想要可变长集合，后面会进入 `Vec<T>`。

### ❌ 数组越界

```rust
let a = [1, 2, 3];
println!("{}", a[10]);
```

这会在运行时直接报错并 panic。

## 我的理解

这一节的重点不是“会写 tuple 和 array”，而是开始形成组合值的意识：

- 单个值不够表达信息时，就要考虑更高一层的数据组织
- Rust 会要求你在“类型是否一致、长度是否固定”这些事情上更明确

## 下一步

- 回到目录：[第 1 章：Quick Startup](./README.md)
- 后续再进入函数、控制流和更复杂的类型组织方式
