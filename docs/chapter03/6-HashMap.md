# 6. HashMap<K, V>：键值对存储

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_06_hashmaps.rs`
> - **上一篇**：[5. Result](./5-Result.md)
> - **下一篇**：[7. 模式匹配上下文](./7-模式匹配上下文.md)
> - **关键词**：`HashMap`、`insert`、`get`、`entry`、`or_insert`、`contains_key`、遍历

---

## 这一节解决什么问题

需要"按名字查分数"、"统计词频"、"存储配置"这类场景时，数组/Vec 不够用——你需要一个按**键**快速查找**值**的数据结构。

`HashMap<K, V>` 是 Rust 标准库里的哈希表实现，底层用开放地址法（sip hash）。

---

## 一分钟结论

- `HashMap::new()` 创建空 HashMap；或用 `HashMap::from([...])` 直接初始化
- `insert(k, v)` 插入（存在则覆盖，返回旧值 `Option<V>`）
- `get(&k)` 查找，返回 `Option<&V>`
- `entry(k).or_insert(v)` 不存在时才插入（常用于计数/累计）
- 遍历：`for (k, v) in &map` / `map.keys()` / `map.values()`
- Key 类型必须实现 `Hash + Eq`

---

## 详细原理

### 1. 创建和插入

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Alice".to_string(), 95);
scores.insert("Bob".to_string(), 87);

// from 初始化（数组 / 迭代器）
let scores2: HashMap<&str, i32> = [("Alice", 95), ("Bob", 87)].into_iter().collect();

// 只在不存在时插入
scores.entry("Carol".to_string()).or_insert(0);
```

### 2. 查找

```rust
// get 返回 Option<&V>
match scores.get("Alice") {
    Some(s) => println!("Alice 的分数: {s}"),
    None    => println!("找不到 Alice"),
}

// contains_key 只检查存在性
if scores.contains_key("Bob") {
    println!("Bob 在表中");
}

// 带默认值
let carol_score = scores.get("Carol").copied().unwrap_or(0);
```

### 3. `entry` API：最常用的高级操作

```rust
// 计数（词频统计）
let text = "hello world hello rust world";
let mut word_count: HashMap<&str, i32> = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}

// entry API：
// - entry(key)：获取这个 key 的"占位符"
// - or_insert(v)：不存在则插入 v，然后返回 &mut V
// - or_insert_with(f)：不存在则调用 f() 生成值
// - or_default()：不存在则插入 T::default()
```

---

## 完整运行示例

```rust
use std::collections::HashMap;

fn main() {
    println!("=== 基础操作 ===");
    let mut inventory: HashMap<&str, u32> = HashMap::new();
    inventory.insert("苹果", 100);
    inventory.insert("香蕉", 50);
    inventory.insert("橙子", 75);

    println!("库存:");
    let mut items: Vec<_> = inventory.iter().collect();
    items.sort_by_key(|(k, _)| *k);
    for (item, qty) in &items {
        println!("  {item}: {qty}");
    }
    println!();

    println!("=== get 查找 ===");
    for item in ["苹果", "梨", "葡萄"] {
        match inventory.get(item) {
            Some(qty) => println!("  {item}: {qty} 件"),
            None => println!("  {item}: 无库存"),
        }
    }
    println!();

    println!("=== entry API ===");
    // 卖出商品（或添加新商品）
    let sales = [("苹果", 30), ("葡萄", 20), ("香蕉", 10)];
    for (item, qty) in sales {
        let current = inventory.entry(item).or_insert(0);
        if *current >= qty {
            *current -= qty;
        } else {
            println!("  {item} 库存不足，只有 {current} 件");
        }
    }
    println!("销售后库存: {:?}", inventory);
    println!();

    println!("=== 词频统计 ===");
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut freq: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }

    // 按频率排序输出
    let mut sorted: Vec<_> = freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("词频 Top 5:");
    for (word, count) in sorted.iter().take(5) {
        println!("  {word}: {count}次");
    }
    println!();

    println!("=== HashMap 的所有权 ===");
    let key = String::from("owned_key");
    let val = String::from("owned_val");
    let mut map: HashMap<String, String> = HashMap::new();

    // insert 会 move String（非 Copy 类型）
    map.insert(key, val);
    // println!("{key}"); // ❌ key 已被 move 进 map
    println!("  map 有 {} 个条目", map.len());
}
```

---

## 性能特性


| 操作       | 时间复杂度   |
| -------- | ------- |
| `insert` | 平均 O(1) |
| `get`    | 平均 O(1) |
| `remove` | 平均 O(1) |
| 遍历       | O(n)    |


默认使用 SipHash（抗哈希泛洪攻击），可以通过 `with_hasher` 换用更快的哈希函数。

---

## 注意点与陷阱

### 陷阱：`insert` 会 move 非 Copy 的键值

```rust
let key = String::from("k");
let val = String::from("v");
map.insert(key, val);
// key 和 val 都被 move 进 map
// 如果需要保留原值，用 key.clone()
```

---

## 下一步

- 继续阅读：[7. 模式匹配上下文](./7-模式匹配上下文.md)
- 回到目录：[第 3 章：自定义类型](./README.md)

