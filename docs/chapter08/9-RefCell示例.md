# 9. RefCell 示例

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/topic_09_refcell_example.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_09_refcell_example.rs` 现在把 `Rc<RefCell<T>>` 的关键价值展示得更完整：
- 多个函数 / 多个 Rc 克隆共享同一份 `FileState`
- 任何一个克隆做 `borrow_mut().opened_by.push(...)`，其他克隆立刻能读到新状态
- 打印 `strong_count` 观察引用计数
- 末尾提示：**多线程场景要换成 `Arc<Mutex<T>>` / `Arc<RwLock<T>>`**

```text
strong_count = 1
notes.txt opened by ["Alice", "Bob"]
view2 看到 opened_by = ["Alice", "Bob", "Charlie"]
strong_count = 3
```

## 定义

这一节展示的是常见组合：`Rc<RefCell<T>>`。它不是新语法，而是把两种能力叠起来：

- `Rc`：共享所有权
- `RefCell`：运行时可变性

## 作用

- 让多个持有者共同访问并修改同一份状态
- 支持图、树、GUI 状态等共享结构
- 作为教学示例，帮助看清多个内存工具之间的分工

## 原理

外层 `Rc` 解决“谁拥有”，内层 `RefCell` 解决“怎么改”。因此两个调用者都能拿着同一个句柄，对同一底层值做修改。

## 最小示例

```rust
let shared_file = Rc::new(RefCell::new(FileState {
    name: "notes.txt".to_string(),
    opened_by: Vec::new(),
}));
```

## 注意点

- 组合工具越多，越要明确每一层到底在解决什么
- `Rc<RefCell<T>>` 很方便，但也容易把状态边界搞得太松
- 读写时仍要注意借用作用域尽量短

## 常见错误

- 只记住“共享可变状态”四个字，不理解两层含义
- 什么共享场景都先上 `Rc<RefCell<T>>`
- 忽视后续可能出现的引用环问题

## 我的理解

这一节像是把前面所有内容拼到一张图里：生命周期、所有权、共享、可变性，最终都会回到“这个值到底怎么被管理”。

## 下一步

回到 [第 8 章目录](./README.md)，然后进入 [第 9 章：Implementing Typical Data Structures](../chapter09/README.md)。
