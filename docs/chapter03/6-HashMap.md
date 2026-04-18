# 6. HashMap

> 类型：**Study note**
> 关键词：`HashMap<K, V>`、key-value、`get`
> 上一篇：[5. Result](./5-Result.md)
> 下一篇：[7. 模式匹配上下文](./7-模式匹配上下文.md)

## 一分钟结论

- `HashMap<K, V>` 是按 key 存取值的集合
- 它适合查表、统计、配置、索引等场景
- `get()` 返回的不是裸值，而通常是 `Option<&V>`

## 证据来源

- 对应模块：[topic_06_hashmaps.rs](../../chapters/chapter03/src/topic_06_hashmaps.rs)
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
scores = {"rust": 100, "math": 95}
rust score = Some(100)
```

## 扩展演示输出（当前代码已升级）

`topic_06_hashmaps.rs` 覆盖 `HashMap` 6 类日常操作：`new` + `insert` vs `HashMap::from(...)` 构造 → `insert` 覆盖 vs `entry(...).or_insert(...)` 惯用法 → **词频统计经典写法**（`*counts.entry(w).or_insert(0) += 1`）→ `get` 返回 `Option<&V>` → 遍历与 `keys()` / `values()` → `remove` 取走旧值。

```text
-- (3) 基于当前值更新：entry + *value += 1 --
word counts = {"fun": 1, "and": 1, "fast": 1, "rust": 2, "is": 2}

-- (4) 查询: get 返回 Option<&V> --
scores.get("rust")   = Some(100)
scores.get("python") = None

-- (6) 移除 --
remove("math") 返回旧值 = Some(95)
```

## 定义

`HashMap<K, V>` 是标准库提供的键值对容器：

- `K`：键
- `V`：值

## 作用

- 用 key 快速定位值
- 表达配置项、统计表、缓存等结构
- 把“按位置访问”转换成“按名称访问”

## 原理

最小使用流程：

```rust
let mut scores = HashMap::new();
scores.insert("math", 95);
scores.insert("rust", 100);
```

查询：

```rust
scores.get("rust")
```

返回的是：

- `Some(&100)` 这种“查到了”
- 或 `None` 这种“没查到”

## 最小示例

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("math", 95);
    scores.insert("rust", 100);

    println!("{:?}", scores.get("rust"));
}
```

## 注意点

### 1. `get` 返回的是 `Option`

因为查不到 key 是正常情况。

### 2. `HashMap` 不是有序表

打印顺序不要默认当作插入顺序。

### 3. key-value 模型适合“查找”，不一定适合“按序遍历”

选容器时要先想场景。

## 常见错误

### ❌ 错误 1：默认 key 一定存在

这会导致你对 `get` 的返回值掉以轻心。

### ❌ 错误 2：把打印顺序当作逻辑顺序

`HashMap` 的重点不是顺序，而是键查找。

### ❌ 错误 3：能用 struct 的地方硬用 HashMap

如果字段固定且语义明确，struct 往往更合适。

## 我的理解

- `HashMap` 的本质不是“装东西的容器”
- 而是“把查找关系变成显式结构”
- 它和 struct / enum 不冲突，解决的是不同层面的建模问题

## 下一步

下一篇开始把“模式匹配”从 `match` 这个单点，扩展到更大的上下文里看。

- 继续阅读：[7. 模式匹配上下文](./7-模式匹配上下文.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
