# 7. Rc<T>：单线程引用计数共享所有权

> - **所属章节**：第 9 章 · Memory Management Features
> - **Cargo package**：`chapter10`
> - **运行方式**：`cargo run -p chapter10`
> - **代码位置**：`chapters/chapter10/src/topic_07_rc_smart_pointer.rs`
> - **上一篇**：[6. Box 的典型用法](./6-Box的典型用法.md)
> - **下一篇**：[8. RefCell 内部可变性](./8-RefCell.md)
> - **关键词**：`Rc<T>`、引用计数、`strong_count`、`Weak<T>`、循环引用

---

## 这一节解决什么问题

Rust 的默认所有权规则：**一个值同一时刻只有一个所有者**。但有些场景需要多个所有者——比如图结构里的节点被多条边引用，GUI 里的数据被多个组件共享。

`Rc<T>`（Reference Counted）通过引用计数解决这个问题：

- 每个 `Rc::clone` 增加计数
- 每个 `Rc` 被 drop 时减少计数
- 计数归零时，数据才真正被释放

---

## 一分钟结论

- `Rc::new(value)` 创建共享数据，`Rc::clone(&rc)` 增加引用计数
- 所有 Rc 指向**同一块堆内存**（不是 clone 数据！）
- `Rc::strong_count(&rc)` 查看当前引用数
- `Rc<T>` 只能**只读**访问数据（没有 `&mut` 访问）
- `Rc<T>` **不是线程安全的**——跨线程请用 `Arc<T>`
- 循环引用会导致内存泄漏——用 `Weak<T>` 打破循环

---

## 与其他语言对比

| 语言 | 多所有权的实现 |
|-----|------------|
| Java | GC 追踪所有引用，任何活着的引用都阻止 GC |
| Python | 引用计数 + 循环 GC |
| C++ | `std::shared_ptr<T>`（类似 Rust 的 `Arc<T>`）|
| Rust | `Rc<T>`（单线程）/ `Arc<T>`（多线程）|

---

## 核心概念与心智模型

```
Rc<T> 的内存结构：

  rc1   rc2   rc3
  ↓      ↓      ↓
  ┌──────────────────────────────────┐
  │  引用计数: 3                      │
  │  数据: "hello"                   │ ← 堆上的同一份数据
  └──────────────────────────────────┘

所有 Rc 被 drop：计数 → 0 → 数据被释放
```

---

## 详细原理

### 1. 基础用法

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared data"));
println!("创建后: strong_count = {}", Rc::strong_count(&a));  // 1

let b = Rc::clone(&a);
println!("clone 后: strong_count = {}", Rc::strong_count(&a));  // 2

{
    let c = Rc::clone(&a);
    println!("再 clone: strong_count = {}", Rc::strong_count(&a));  // 3
    // c 在这里 drop
}
println!("c drop 后: strong_count = {}", Rc::strong_count(&a));  // 2

// a, b 都指向同一份数据
println!("a = {a}, b = {b}");
assert_eq!(a, b);
```

### 2. 图结构（多所有者）

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node { value, children: Vec::new() })
    }
}

let leaf1 = Node::new(3);
let leaf2 = Node::new(4);

// leaf1 被两个父节点共享
let parent1 = Rc::new(Node {
    value: 1,
    children: vec![Rc::clone(&leaf1)],
});

let parent2 = Rc::new(Node {
    value: 2,
    children: vec![Rc::clone(&leaf1), Rc::clone(&leaf2)],
});

println!("leaf1 被 {} 个节点引用", Rc::strong_count(&leaf1)); // 3（leaf1 + parent1 + parent2）
```

---

## 完整运行示例

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== Rc 基础：引用计数 ===");
    let value = Rc::new(vec![1, 2, 3]);
    println!("创建: count = {}", Rc::strong_count(&value));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    println!("两次 clone: count = {}", Rc::strong_count(&value));

    println!("a = {a:?}, b = {b:?}");  // 都指向同一个 Vec
    println!("a.as_ptr() == b.as_ptr(): {}", Rc::ptr_eq(&a, &b));  // true

    drop(b);
    println!("drop b: count = {}", Rc::strong_count(&value));  // 2
    println!();

    println!("=== Rc<RefCell<T>>：多所有者+可变 ===");
    let shared: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));

    let consumer1 = Rc::clone(&shared);
    let consumer2 = Rc::clone(&shared);

    // consumer1 追加数据
    consumer1.borrow_mut().push(4);

    // consumer2 也看到了更新（同一份数据！）
    println!("consumer2 看到: {:?}", consumer2.borrow());  // [1, 2, 3, 4]
    println!();

    println!("=== 循环引用问题（用 Weak 解决）===");
    use std::rc::Weak;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,  // 弱引用：不增加引用计数
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // leaf 的 parent 设为 branch 的弱引用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch strong count = {}", Rc::strong_count(&branch));  // 1
    println!("branch weak count = {}", Rc::weak_count(&branch));      // 1

    // 通过弱引用访问父节点
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("leaf 的父节点值: {}", parent.value);
    }
}
```

---

## Rc vs Arc 对照

| 特性 | `Rc<T>` | `Arc<T>` |
|-----|---------|---------|
| 线程安全 | ❌（`!Send`、`!Sync`）| ✅ |
| 引用计数操作 | 非原子（更快）| 原子操作（线程安全）|
| 性能 | 稍快 | 稍慢（原子开销）|
| 使用场景 | 单线程 | 多线程 |

**原则**：单线程用 `Rc`，跨线程用 `Arc`。

---

## 下一步

- 继续阅读：[8. RefCell 内部可变性](./8-RefCell.md)
- 回到目录：[第 9 章：内存管理特性](./README.md)
