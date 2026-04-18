# 5. 分层结果类型：Result<Option<T>, E>

- Cargo package: `chapter11`
- Run chapter: `cargo run -p chapter11`
- Chapter entry: `chapters/chapter11/src/main.rs`
- Reference module: `chapters/chapter11/src/topic_05_layered_outcomes_result_option_part1.rs`
- Chapter lab: `chapters/chapter11/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_05_layered_outcomes_result_option_part1.rs` 文件头用表格明确 `Result<Option<T>, E>` 的 3 种结局对应的语义：

| 形状 | 语义 | 处理 |
|-----|------|-----|
| `Ok(Some(t))` | 查到 | 正常走业务 |
| `Ok(None)` | 合法完成但没数据 | 产品未找到 / 缓存 miss |
| `Err(e)` | 执行失败 | 记录、重试、向上抛 |

核心提醒：**`Ok(None)` 不是错误**——不要把"找不到"当成 `Err`。

## 定义

`Result<Option<T>, E>` 表示三种结果：

- `Err(E)`：操作失败
- `Ok(Some(T))`：操作成功且有值
- `Ok(None)`：操作成功但没有值

## 作用

- 表达“缺值不是错误”的查询类场景
- 保持错误和缺失语义分离
- 提升数据库、缓存、查找类 API 的可读性

## 原理

外层 `Result` 回答“操作本身有没有失败”，内层 `Option` 回答“如果操作成功，值存在不存在”。

## 最小示例

```rust
fn find_product(id: u32, connection_available: bool)
    -> Result<Option<Product>, DbError>
```

## 注意点

- `None` 不是“查找出错”
- 这类返回值很适合查库、查缓存、查配置
- 不要为了简化签名把 `None` 和 `Err` 混在一起

## 常见错误

- 用 `Result<T, E>` 强行表达“可能查不到”
- 把 `Ok(None)` 也按错误分支处理
- 把“没数据”和“系统故障”写进同一层语义

## 我的理解

这个模式很像真实系统里的查询行为：没找到很常见，但连接失败显然不是一回事。

## 下一步

继续看 [分层结果类型：Option<Result<T, E>>](./6-分层结果类型：OptionResult.md)，比较另一种完全不同的语义。
