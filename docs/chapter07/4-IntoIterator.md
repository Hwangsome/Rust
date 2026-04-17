# 4. IntoIterator

- Cargo package: `chapter07`
- Run chapter: `cargo run -p chapter07`
- Chapter entry: `chapters/chapter07/src/main.rs`
- Reference module: `chapters/chapter07/src/into_iter.rs`
- Chapter lab: `chapters/chapter07/src/lab.rs`

## 定义

`IntoIterator` 负责回答“一个值如何转换成迭代器”。`for value in collection` 背后，先发生的就是 `collection.into_iter()`。

## 作用

- 让自定义类型支持 `for` 循环
- 把“容器”和“迭代器”两层角色分开
- 为拥有值、借用值、可变借用值的不同遍历方式留出空间

## 原理

`IntoIterator` 定义了两件事：元素类型 `Item`，以及真正返回的迭代器类型 `IntoIter`。这让一个“可遍历类型”可以把具体遍历逻辑交给专门的迭代器实现。

## 最小示例

```rust
impl IntoIterator for Playlist {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.songs.into_iter()
    }
}
```

## 注意点

- `IntoIterator` 不是 `Iterator`
- `into_iter(self)` 常常意味着会消费原值
- 标准库集合同时会为拥有值、`&T`、`&mut T` 提供不同版本实现

## 常见错误

- 只记得 `Iterator`，忽略 `IntoIterator`
- 没意识到 `for` 循环可能拿走所有权
- 给自定义类型实现了 `Iterator` 却不明白为什么 `for` 还不够自然

## 我的理解

如果说 `Iterator` 是“怎么一个个给值”，那 `IntoIterator` 就是“怎么进入这套给值流程”。

## 下一步

继续看 [遍历集合](./5-遍历集合.md)，把 `iter`、`iter_mut`、`into_iter` 的区别摆到一起。
