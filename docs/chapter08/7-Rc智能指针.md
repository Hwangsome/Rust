# 7. Rc 智能指针

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/topic_07_rc_smart_pointer.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_07_rc_smart_pointer.rs` 现在把 `strong_count` 在创建 / `clone` / `drop` 过程中的变化清晰打出来，并说明 **Rc 不跨线程**——多线程请换 `Arc`。

```text
创建后 strong_count = 1
两次 clone 后 strong_count = 3
内层作用域内 strong_count = 4
drop temp 后 strong_count = 3
```

## 定义

`Rc<T>` 是单线程下的引用计数智能指针，用来表达多个 owner 共享同一个值。

## 作用

- 让多个地方共同拥有同一份数据
- 避免手动维护共享资源寿命
- 在树、图、配置共享等结构里很常见

## 原理

每次 `Rc::clone` 都不会复制底层值，只会让强引用计数加一。最后一个强引用离开作用域后，底层值才会真正被释放。

## 最小示例

```rust
let shared_title = Rc::new(String::from("Rust patterns"));
let reader_a = Rc::clone(&shared_title);
let reader_b = Rc::clone(&shared_title);
```

## 注意点

- `Rc::clone` 克隆的是指针，不是底层数据
- Rc 只适合单线程
- Rc 本身不提供可变访问

## 常见错误

- 看到 clone 就以为复制了完整数据
- 把 Rc 当成“共享 + 可变”的一站式工具
- 在多线程环境继续使用 Rc

## 我的理解

Rc 回答的是“多个 owner 怎么安全地共同拥有同一个值”，但它故意不回答“那谁来改这个值”。

## 下一步

继续看 [RefCell](./8-RefCell.md)，补上“内部可变性”这块拼图。
