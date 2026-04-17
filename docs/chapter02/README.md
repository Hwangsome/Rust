# 第 2 章：Ownership and Borrowing

这一章是 Rust 入门里真正开始“和其他语言拉开差距”的地方。难点不在语法，而在心智模型：

- 值为什么会 move
- 为什么借用规则这么严格
- 为什么有时函数参数写值，有时写引用
- `*` 到底是在“取值”还是“做指针操作”

如果这一章没吃透，后面学 `struct`、`Vec`、`String`、迭代器和并发时都会反复卡住。

## 本章目标

- 建立所有权、借用、解引用的最小统一模型
- 区分 `move`、`clone`、`Copy`
- 理解函数调用为什么会成为所有权边界
- 记住借用规则不是语法限制，而是内存安全约束

## 推荐阅读顺序

1. [所有权基础](./1-所有权.md)
   先把“值在赋值时会发生什么”搞清楚。
2. [函数中的所有权](./2-函数中的所有权.md)
   再看函数调用为什么会形成所有权边界。
3. [借用基础](./3-借用基础.md)
   然后引入“使用值但不拿走值”的模型。
4. [函数中的借用](./4-函数中的借用.md)
   最后看 API 设计里为什么大量使用 `&T` 和 `&mut T`。
5. [解引用](./5-解引用.md)
   把引用和值之间的最后一步操作补齐。

## 对应代码与实验

- Cargo package：`chapter02`
- 运行方式：`cargo run -p chapter02`
- 章节入口：[chapters/chapter02/src/main.rs](../../chapters/chapter02/src/main.rs)
- 主题模块：
  - [topic_01_ownership_basics.rs](../../chapters/chapter02/src/topic_01_ownership_basics.rs)
  - [topic_02_ownership_in_functions.rs](../../chapters/chapter02/src/topic_02_ownership_in_functions.rs)
  - [topic_03_borrowing_basics.rs](../../chapters/chapter02/src/topic_03_borrowing_basics.rs)
  - [topic_04_borrowing_in_functions.rs](../../chapters/chapter02/src/topic_04_borrowing_in_functions.rs)
  - [topic_05_dereferencing.rs](../../chapters/chapter02/src/topic_05_dereferencing.rs)
- 练习模块：[lab.rs](../../chapters/chapter02/src/lab.rs)

## 本章关键输出

`cargo run -p chapter02` 的一组核心输出：

```text
clone 后两个 String 都可用: s1 = world, s2 = world
Copy 类型赋值后原值仍可用: x = 15, y = 15
```

```text
immutable refs => [4, 5, 6], [4, 5, 6]
mutable ref after immutable refs end => [4, 5, 6, 7]
```

```text
*reference 解引用后得到: 42
```

## 本章最容易混淆的地方

- “赋值”不一定是复制
- “函数传参”不一定只是把值借过去看看
- “引用能打印出来”不代表它和底层值是同一个东西
- “能创建多个引用”不代表任何组合都合法

## 我的建议读法

这一章最好按下面顺序理解，而不是反过来：

1. 先看 **值如何移动**
2. 再看 **为什么需要借用**
3. 再看 **函数签名如何表达借用**
4. 最后再看 **解引用**

因为 `*` 的意义只有放回所有权 / 借用体系里才清楚。
