# 8. Vec 与动态数组

> - **所属章节**：第 8 章 · Functional Programming Aspects
> - **Cargo package**：`chapter08`
> - **运行方式**：`cargo run -p chapter08`
> - **代码位置**：`chapters/chapter08/src/topic_08_vec.rs`
> - **上一篇**：[4. IntoIterator](./4-IntoIterator.md)
> - **下一篇**：[5. 遍历集合](./5-遍历集合.md)
> - **关键词**：`Vec<T>`、`vec!`、`Vec::new`、`push`、`pop`、切片 `&[T]`、`get`、容量

---

## 这一节解决什么问题

你已经会用**固定长度数组** `[T; N]` 和**切片引用** `&[T]`。日常代码里更需要的是：**长度可变**、在堆上增长的序列——这就是 **`Vec<T>`**（dynamic vector，动态向量）。

它和后面「用迭代器遍历集合」直接相关：`collect::<Vec<_>>()` 是最常见的收集方式之一。

---

## 一分钟结论

- **`Vec<T>`**：堆上、连续内存、长度可变；拥有所有权（`Vec` 离开作用域时释放堆内存）。
- **`vec![a, b, c]`**：创建 `Vec`，元素类型一般自动推断。
- **`Vec::new()`**：空向量；常需类型标注 `Vec<i32>` 或靠第一次 `push` 推断 `T`。
- **`push` / `pop`**：尾部增删；`len` / `is_empty` 表示当前元素个数。
- **`v[i]`**：越界在 debug 下 **panic**；安全访问用 **`v.get(i) -> Option<&T>`**。
- **`&Vec<T>`** 可 **Deref** 到 **`&[T]`**，很多 API 只接收切片，传 `&v` 即可。

---

## 与数组、切片的关系

| 类型 | 长度 | 典型位置 |
|------|------|----------|
| `[T; N]` | 编译期固定 | 栈（或嵌入 struct） |
| `Vec<T>` | 运行期可变 | 堆（`Vec` 本身在栈上，存指针、长度、容量） |
| `&[T]` | 运行期已知，不拥有数据 | 任意连续内存的**视图** |

---

## 详细说明

### 创建

```rust
let v = vec![1, 2, 3];
let mut empty: Vec<i32> = Vec::new();
empty.push(0);
```

### 容量与长度

- **`len()`**：当前元素个数。
- **`capacity()`**：已分配、无需再分配即可容纳的元素槽位（可能 ≥ `len`）。
- 频繁 `push` 可能触发扩容：分配新缓冲区、搬移元素、释放旧区——大批量插入时可考虑 `with_capacity(n)`（第 8 章性能/集合调优会再提）。

### 与迭代器、遍历

`Vec` 实现 `IntoIterator` 等，因此可以 `for x in v`、`v.iter()` 等。三种借用形态（`iter` / `iter_mut` / `into_iter`）见 **[5. 遍历集合](./5-遍历集合.md)**。

---

## 下一步

继续阅读 **[5. 遍历集合](./5-遍历集合.md)**，把 `Vec` / `HashMap` 上的 **`iter()` / `iter_mut()` / `into_iter()`** 一次理清。

- 回到目录：[第 8 章 README](./README.md)
