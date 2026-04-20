# 5. Box<T> 智能指针：堆分配与独占所有权

> - **所属章节**：第 9 章 · Memory Management Features
> - **Cargo package**：`chapter09`
> - **运行方式**：`cargo run -p chapter09`
> - **代码位置**：`chapters/chapter09/src/topic_05_box_smart_pointer.rs`
> - **上一篇**：[4. Struct 中的生命周期](./4-Struct中的生命周期.md)
> - **下一篇**：[6. Box 的典型用法](./6-Box的典型用法.md)
> - **关键词**：`Box<T>`、堆分配、递归类型、`Deref`、`Drop`、`Box<dyn Trait>`

---

## 这一节解决什么问题

默认情况下，Rust 把值存在**栈**上——速度快，但栈大小有限（通常 8MB），而且编译期必须知道大小。

`Box<T>` 让你把数据放到**堆**上：

1. **栈上只有指针**（8字节），实际数据在堆上
2. **大对象**：避免大量数据在栈上复制
3. **递归类型**：`enum List { Cons(i32, Box<List>), Nil }` 只有用 Box 才能确定大小
4. **Trait Object**：`Box<dyn Draw>` 存放不同具体类型

---

## 一分钟结论

- `Box::new(value)` 把 value 移到堆上，返回 `Box<T>`（8字节指针）
- `Box<T>` 有独占所有权，离开作用域时自动释放堆内存
- `*box_value` 解引用获取底层值；方法调用自动解引用
- 主要用途：大对象、递归类型、Trait Object（`Box<dyn Trait>`）
- `Box<T>` 本身是 `Sized`（8字节）；当 T 是 DST 时，Box 变成胖指针（16字节）

---

## 与其他语言对比

| 语言 | 堆分配的方式 |
|-----|----------|
| C | `malloc()` + 手动 `free()`（必须记得释放）|
| C++ | `new` + `delete`；或 `std::unique_ptr<T>`（RAII）|
| Java | `new MyClass()`（所有对象都在堆上，GC 管理）|
| Rust | `Box::new(value)`（RAII，离开作用域自动释放）|

---

## 核心概念与心智模型

```
栈 vs 堆：

let x: i32 = 42;  // 栈上
┌──────────────────────────────────────────────────┐
│  栈                                               │
│  x: ┌────────┐                                  │
│     │   42   │  4 字节，访问极快                 │
│     └────────┘                                  │
└──────────────────────────────────────────────────┘

let b: Box<i32> = Box::new(42);  // 堆上
┌──────────────────────────────────────────────────┐
│  栈                                               │
│  b: ┌────────────────┐                           │
│     │  ptr → 0x1234  │  8 字节（指针）            │
│     └────────────────┘                           │
└──────────────────────────────────────────────────┘
         │
         ▼ 堆
┌──────────────────────────────────────────────────┐
│     ┌────────┐                                   │
│     │   42   │  4 字节，通过指针访问              │
│     └────────┘                                   │
└──────────────────────────────────────────────────┘
```

---

## 详细原理

### 1. 基础用法

```rust
// 把值放到堆上
let b = Box::new(5_i32);
println!("b = {b}");       // 自动解引用：5
println!("*b = {}", *b);   // 显式解引用：5

// Box 离开作用域时，堆内存自动释放（通过 Drop trait）
{
    let big = Box::new([0_u8; 1_000_000]);  // 1MB 在堆上
    println!("big 数组长度 = {}", big.len());
}  // ← 自动释放 1MB

// 解引用 move
let b = Box::new(String::from("hello"));
let s: String = *b;  // move 堆上的数据到 s，b 失效
println!("{s}");
```

### 2. 递归类型（Box 解决无限大小问题）

```rust
// ❌ 没有 Box：编译器无法确定大小
// enum List {
//     Cons(i32, List),  // 大小 = i32 + List = i32 + i32 + List = 无穷大！
//     Nil,
// }

// ✅ 有 Box：下一个节点放在堆上，大小固定
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),  // i32 + 8字节指针 = 固定大小！
    Nil,
}

let list = List::Cons(1,
    Box::new(List::Cons(2,
        Box::new(List::Cons(3,
            Box::new(List::Nil))))));

println!("{list:?}");
```

---

## 完整运行示例

```rust
use std::mem::size_of;

// 递归枚举
#[derive(Debug)]
enum BinaryTree {
    Leaf(i32),
    Node(Box<BinaryTree>, i32, Box<BinaryTree>),
}

impl BinaryTree {
    fn sum(&self) -> i32 {
        match self {
            BinaryTree::Leaf(n) => *n,
            BinaryTree::Node(left, n, right) => left.sum() + n + right.sum(),
        }
    }

    fn depth(&self) -> usize {
        match self {
            BinaryTree::Leaf(_) => 1,
            BinaryTree::Node(l, _, r) => 1 + l.depth().max(r.depth()),
        }
    }
}

fn main() {
    println!("=== Box 大小 ===");
    println!("i32:         {} bytes", size_of::<i32>());
    println!("Box<i32>:    {} bytes", size_of::<Box<i32>>());
    println!("Box<str>:    {} bytes", size_of::<Box<str>>());  // DST → 16 bytes
    println!();

    println!("=== 基础堆分配 ===");
    let b = Box::new(42_i32);
    println!("Box<i32> = {b}");  // 自动解引用

    // 转移所有权出 Box
    let n: i32 = *b;
    println!("moved out: {n}");
    println!();

    println!("=== 递归树结构 ===");
    //       10
    //      /  \
    //     5    15
    //    / \
    //   3   7
    let tree = BinaryTree::Node(
        Box::new(BinaryTree::Node(
            Box::new(BinaryTree::Leaf(3)),
            5,
            Box::new(BinaryTree::Leaf(7)),
        )),
        10,
        Box::new(BinaryTree::Leaf(15)),
    );

    println!("树的深度: {}", tree.depth());
    println!("树的节点和: {}", tree.sum());  // 3+5+7+10+15 = 40
    println!();

    println!("=== RAII：离开作用域自动释放 ===");
    {
        let _big = Box::new(vec![0_i32; 100_000]);  // 400KB
        println!("  大数组已分配");
    }  // ← 自动释放 400KB
    println!("  大数组已释放");
}
```

---

## 实际工程场景

### 1. 大型配置对象避免栈复制

```rust
#[derive(Clone)]
struct Config {
    rules: Vec<String>,  // 可能很大
    settings: std::collections::HashMap<String, String>,
}

// 函数间传递大型 Config：用 Box 避免复制整个结构体
fn process(config: Box<Config>) {
    // ...
}
```

### 2. `Box<dyn Error>` 统一错误类型

```rust
fn do_work() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("config.txt")?;  // io::Error → Box<dyn Error>
    let n: i32 = file.trim().parse()?;                  // ParseIntError → Box<dyn Error>
    println!("n = {n}");
    Ok(())
}
```

---

## 注意点

### Box vs 栈分配的选择

```
使用 Box 的情况：
  - 递归类型（必须用）
  - 大型对象（减少栈压力）
  - 需要 Trait Object（Box<dyn T>）
  - 需要确定的堆分配生命周期

不需要 Box 的情况：
  - 小型值（i32, bool, 小 struct）→ 栈分配更快
  - 编译期大小已知且不大 → 直接栈分配
```

---

## 下一步

- 继续阅读：[6. Box 的典型用法](./6-Box的典型用法.md)
- 回到目录：[第 9 章：内存管理特性](./README.md)
