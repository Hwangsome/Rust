//! `Vec<T>`：堆上的**动态数组**，标准库里最常用的「可变长度、连续内存」序列。
//!
//! 和 **`[T; N]`**（栈上、长度写死在类型里）不同：`Vec` 在**堆**上分配，运行时可 `push` / `pop` 改变长度。
//! 和 **切片 `&[T]`** 的关系：`Vec<T>` 实现了 **`Deref<Target = [T]>`**，因此很多地方可以把 `&Vec<T>` **当成** `&[T]` 用（索引、`len()`、`get()` 等）。
//!
//! 本节演示创建、`push` / `pop`、索引与 `get`、与切片的联系；**三种遍历**（`iter` / `iter_mut` / `into_iter`）在 `topic_05_iterating_through_collections` 里展开。
//!
//! ## 阅读笔记
//!
//! - **`vec![a, b, c]`**：宏，类型推断为 `Vec<T>`（`T` 由各元素统一）。
//! - **`Vec::new()`**：空 Vec，通常要写类型标注或第一次 `push` 时推断 `T`。
//! - **`v[i]`**：越界在 debug 下 **panic**；需要「可能没有」时用 **`v.get(i) -> Option<&T>`**。
//! - **容量 `capacity()`**：可能比 `len()` 大；扩容会重新分配，旧缓冲区丢弃（元素已 move 或 copy 到新位置）。

pub fn run() {
    println!("== Vec ==");

    println!("-- (1) `vec!` 与 `Vec::new` --");
    let mut v1 = vec![1, 2, 3];
    let mut v2: Vec<i32> = Vec::new();
    v2.push(10);
    v2.push(20);
    println!("v1 = {v1:?}");
    println!("v2 = {v2:?}");
    println!();

    println!("-- (2) `push` / `pop` / `len` / `is_empty` --");
    v1.push(4);
    println!("after push: {v1:?}, len = {}", v1.len());
    let last = v1.pop();
    println!("pop -> {last:?}, remaining = {v1:?}");
    println!("is_empty on v2: {}", v2.is_empty());
    println!();

    println!("-- (3) 索引 `v[i]` 与 `get(i)` --");
    let words = vec!["alpha", "beta", "gamma"];
    println!("words[1] = {}", words[1]);
    println!("words.get(1) = {:?}", words.get(1));
    println!("words.get(99) = {:?}", words.get(99));
    println!();

    println!("-- (4) `&Vec<T>` 用作 `&[T]`（切片视图）--");
    fn print_len(s: &[i32]) {
        println!("  slice len = {}, first = {:?}", s.len(), s.first());
    }
    let nums = vec![7, 8, 9];
    print_len(&nums);
    println!();

    println!("-- (5) `collect` 成 `Vec`（与迭代器衔接）--");
    let squares: Vec<i32> = (0..5).map(|x| x * x).collect();
    println!("squares = {squares:?}");
    println!();
}
