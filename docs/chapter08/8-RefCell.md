# 8. RefCell

- Cargo package: `chapter08`
- Run chapter: `cargo run -p chapter08`
- Chapter entry: `chapters/chapter08/src/main.rs`
- Reference module: `chapters/chapter08/src/topic_08_refcell.rs`
- Chapter lab: `chapters/chapter08/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_08_refcell.rs` 现在用 4 个子场景 + 一个**真实场景 `LazyCache`** 演示"外部 `&self`、内部改状态"的惯用法：
- 在不可变绑定下修改内部 Vec
- 多个 `.borrow()` 可以并存
- `.borrow()` 与 `.borrow_mut()` 重叠会**运行时 panic**（不是编译错误）
- `LazyCache::get_or_compute(&self, ...)`：签名不变，内部仍然能缓存

```text
values = [1, 2, 3, 4]
a = [1, 2, 3, 4], b = [1, 2, 3, 4]
  [cache] 计算并缓存: 49
  [cache] 命中: 49
```

## 定义

`RefCell<T>` 提供内部可变性。它允许在外部是不可变绑定时，仍然在内部执行可变借用。

## 作用

- 解决“接口层看起来不可变，但内部需要修改状态”的场景
- 让某些难以静态表达的借用关系改为运行时检查
- 常与 Rc 组合使用

## 原理

普通引用规则在编译期检查，`RefCell` 会把这套规则推迟到运行时：`borrow()` 表示共享借用，`borrow_mut()` 表示独占可变借用。如果规则冲突，程序会 panic。

## 最小示例

```rust
let values = RefCell::new(vec![1, 2, 3]);
values.borrow_mut().push(4);
```

## 注意点

- RefCell 不是绕过借用规则，而是改成运行时检查
- 运行时违规会 panic
- 单线程场景下很常见，多线程场景要换成 Mutex / RwLock 一类工具

## 常见错误

- 以为用了 RefCell 就不用再考虑借用冲突
- 在逻辑复杂处长期持有 `borrow_mut()`
- 不知道 panic 来自运行时借用规则，而误判成别的问题

## 我的理解

RefCell 的价值不在于“更自由”，而在于“有些关系编译器不容易静态证明，但程序员自己知道是安全的”。

## 下一步

继续看 [RefCell 示例](./9-RefCell示例.md)，把它和 Rc 组合起来。
