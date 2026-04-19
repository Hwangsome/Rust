# 11. `self` 与 `Self`：是什么、为什么、怎么样

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_11_self_explained.rs`（本篇对照的最小 runnable）、`topic_02_adding_functionality_to_structs.rs`（业务向综合）、`topic_10_method_chaining_constraints.rs`（消费型链式）
> - **上一篇**：[10. 方法链的约束](./10-方法链的约束.md)
> - **建议前置阅读**：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
> - **下一篇**：[第 4 章：代码组织](../chapter04/README.md)
> - **关键词**：`self`、`Self`、receiver、`&self`、`&mut self`、`self` 按值、`method` 语法糖、关联函数

---

## 这一节解决什么问题

在 `impl Rectangle { fn area(&self) -> f64 { ... } }` 里，**`self` 到底是什么**？和 **`Self`** 差在哪？**为什么**不能像 Java/Python 那样默认写个 `this` 就随便改字段？**实际写代码时**该选 `&self` / `&mut self` / `self` 哪一种？

本篇用「**是什么 → 为什么 → 怎么样**」把 `self` / `Self` 一次钉牢，并和所有权、方法链对上号。

---

## 一分钟结论

| 符号 | 含义（一句话） |
|------|----------------|
| **`self`**（小写） | 方法的**第一个参数**：**接收者（receiver）**，表示“这次调用作用在**哪个实例**上”。 |
| **`Self`**（大写） | **当前 `impl` 所实现的类型**的别名；在 `impl Rectangle` 里 `Self` ≡ `Rectangle`。 |
| **`&self`** | `self: &Self` 的糖：只读借用实例，**不拿走所有权**。 |
| **`&mut self`** | `self: &mut Self` 的糖：可变借用实例，**可改字段**，且受借用规则约束。 |
| **`self`（单独出现）** | 按值接收 `Self`：**消费**调用者手里的那份实例（常用于 `into_*`、`build` 终结）。 |
| **没有 `self` 的函数** | **关联函数**：用 `Type::foo()` 调用，常用于 `new`。 |

补充一张**含 `mut self` 的四格速查表**（与英文笔记同构）：见 [2. 为 Struct 添加功能](./2-为Struct添加功能.md) 文中的 **「接收者速查」** 小节。

---

## 是什么

### 1. `self`：不是魔法关键字，而是**第一个参数的惯用名**

在 `impl T { ... }` 里，若写成：

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

这里的 **`&self`** 是 **`self: &Rectangle`** 的简写（在 `impl Rectangle` 中 **`Self` = `Rectangle`**）。也就是说：

- **`self`** 和 **`x`、`y`** 一样，本质是**参数名**；
- 约定俗成第一个参数叫 **`self`**，编译器才允许你用 **`r.area()`** 这种**方法调用语法**。

**点号调用的脱糖**（理解即可）：

```rust
let r = Rectangle { width: 3.0, height: 4.0 };
let a = r.area();
// 概念上等价于（示意）：
let a = Rectangle::area(&r);
```

所以：**`self` = “方法所作用的那份实例”在参数里的名字**；带不带引用，决定的是**按值还是借用**。

### 2. `Self`（大写）：**当前类型的名字**

- 在 **`impl Rectangle`** 里：**`Self` ≡ `Rectangle`**。
- 常见写法：`fn new(...) -> Self`、`fn with_width(self, w: f64) -> Self`。
- 好处：改类型名时少改一处；在**泛型 `impl`** 里 `Self` 会随具体实现变化，比写死类型名更安全。

### 3. `mut self` 与 `&mut self`：别混

- **`fn f(&mut self)`**：接收者是 **`&mut Self`** —— 典型“修改方法”。
- **`fn g(mut self)`**：接收者是 **`Self`（按值）**，但参数绑定 **`self` 本身可变**（例如先改字段再 `return self`），**调用后原变量仍被消费**。

---

## 为什么

### 1. Rust 要把「读 / 写 / 吃掉」说清楚

很多语言里实例方法隐含 `this`，且常常默认可改字段。Rust 把**能否改、会不会消费**写进**函数签名**：

- **`&self`**：我只需要**读**；调用方仍拥有 `self`，可多次调用。
- **`&mut self`**：我要**独占写**；同一时刻不能有别处再借同一份实例（与第 2 章借用规则一致）。
- **`self`**：我要**拿走所有权**做转换或终结（`into_*`、`build`）；调用后原绑定**不能再用**（除非 `Copy`）。

这样**接口即文档**，也和**并发、优化**（编译器知道别名情况）一致。

### 2. 没有 `self` 的函数：还没实例，也能“做事”

**关联函数**（如 `Rectangle::new`）没有接收者，因为**构造阶段**还没有 `Rectangle` 值可借。这是 **`Type::`** 调用与 **`.method()`** 分家的根本原因。

### 3. 方法链、Builder 和 `self` 形态绑定

- 返回 **`Self` / `&mut Self`** 才能继续链；
- 某步变成 **`Result<Self>`** 链的形状就变了（见 [10. 方法链的约束](./10-方法链的约束.md)）。

---

## 怎么样

### 1. 怎么选接收者（实战口诀）

| 场景 | 接收者 | 调用后原变量 |
|------|--------|----------------|
| 只读查询、计算、打印 | **`&self`** | 仍可用 |
| 要改字段、累积状态 | **`&mut self`** | 仍可用（可变借用结束即可） |
| 类型转换、合并、终结 Builder | **`self`** | 被消费（除非返回又交回所有权） |
| 构造函数、纯静态工具 | **无 `self`**，`fn new() -> Self` | 不涉及已有实例 |

### 2. 小例子：`&` / `&mut` / 按值 `self`

```rust
struct Counter(u32);

impl Counter {
    fn peek(&self) -> u32 {
        self.0
    }

    fn inc(&mut self) {
        self.0 += 1;
    }

    fn into_inner(self) -> u32 {
        self.0   // 消费 self，把内部值交出去
    }
}

let mut c = Counter(0);
assert_eq!(c.peek(), 0);
c.inc();
assert_eq!(c.peek(), 1);
let n = c.into_inner(); // 之后不能再 c.peek()
assert_eq!(n, 1);
```

### 3. 常见坑

- **`mut self` 不是“可变引用”**：在按值的 `self` 上加 `mut`，只表示可以**改绑定 / 在方法里 move 字段**；和 **`&mut self`** 不同。
- **调用 `self` 方法后**再用原变量：会 **E0382**（已 move），除非类型 **`Copy`** 或你又拿到了新值。
- **`Self` 与 `self`**：写错大小写会直接类型错误；记住 **`Self`=类型，`self`=实例参数**。

### 4. 证据来源（与仓库代码对齐）

- **本篇专用最小示例**：`chapters/chapter03/src/topic_11_self_explained.rs`（`Counter` + `Acc`，对照 `Self` / `&self` / `&mut self` / 按值 `self` / `mut self`）。
- **业务向综合**：`chapters/chapter03/src/topic_02_adding_functionality_to_structs.rs`（`display` / `refuel` / `sell`）。
- **消费型链式**：`chapters/chapter03/src/topic_10_method_chaining_constraints.rs`。

```bash
cargo run -p chapter03
```

#### 关键输出（`topic_11_self_explained` 段，便于核对）

```text
== self and Self (doc 11) ==
-- (1) `Self` 与 `&self` / `&mut self` / 按值 `self` --
peek (等价于 Counter::peek(&c)): 0
after bump: 1
into_inner -> 1

-- (2) `mut self`（按值且绑定可变）≠ `&mut self`（可变借用）--
reorder 消费原 Acc，返回新 Acc: Acc { n: 15 }

提示：方法链里 `-> Self` / `-> &mut Self` 与 `self` 形态强相关，见 topic_10。
```

---

## 下一步

- 若还没读方法语法总览：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
- 方法链与 `Result`：[10. 方法链的约束](./10-方法链的约束.md)
- 进入模块与可见性：[第 4 章](../chapter04/README.md)
