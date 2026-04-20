# 7. 遍历 Option：`Option` 融入迭代器

> - **所属章节**：第 8 章 · Functional Programming Aspects
> - **Cargo package**：`chapter08`
> - **运行方式**：`cargo run -p chapter08`
> - **代码位置**：`chapters/chapter08/src/topic_07_iterating_through_option.rs`
> - **上一篇**：[6. 组合子](./6-组合子.md)
> - **下一篇**：本章最后一篇
> - **关键词**：`Option`、`Iterator`、`flatten`、`filter_map`、`extend`、`transpose`

---

## 这一节解决什么问题

处理 `Vec<Option<T>>` 时，你需要把 `None` 去掉，只保留 `Some` 里的值。最朴素的写法：

```rust
let mut result = Vec::new();
for item in data {
    if let Some(x) = item {
        result.push(x);
    }
}
```

Rust 有更优雅的方式——`Option` 实现了 `IntoIterator`，可以把它当作"0 或 1 个元素的迭代器"使用：

```rust
let result: Vec<i32> = data.into_iter().flatten().collect();
```

---

## 一分钟结论

- `Option<T>` 实现了 `IntoIterator`：`Some(x)` 产出一个元素，`None` 产出零个
- `Option` 组合子：`map`、`and_then`、`unwrap_or`、`filter` 等让它可以流水线处理
- `flatten()`：把 `Iterator<Item = Option<T>>` 变成 `Iterator<Item = T>`（去掉所有 None）
- `filter_map(f)`：把 `map` + `flatten` 合二为一（f 返回 Option）
- `Option::transpose()`：在 `Option<Result<T,E>>` 和 `Result<Option<T>,E>` 之间转换

---

## 详细原理

### 1. Option 作为迭代器

```rust
let some_value: Option<i32> = Some(42);
let no_value: Option<i32> = None;

// Option 可以用在 for 循环里
for x in some_value {
    println!("got: {x}");  // 打印一次
}
for x in no_value {
    println!("got: {x}");  // 不打印
}

// extend 配合 Option
let mut v = vec![1, 2, 3];
v.extend(Some(4));    // 加入 4
v.extend(None::<i32>); // 什么都不加
println!("{v:?}"); // [1, 2, 3, 4]
```

### 2. `flatten`：去掉所有 None

```rust
let items: Vec<Option<i32>> = vec![Some(1), None, Some(2), None, Some(3)];

// 方法 1：flatten
let result: Vec<i32> = items.iter().flatten().copied().collect();
println!("{result:?}"); // [1, 2, 3]

// 方法 2：filter_map（更简洁）
let result: Vec<i32> = items.into_iter().filter_map(|x| x).collect();

// 实际上等价于
let result: Vec<i32> = items.into_iter().flatten().collect();
```

### 3. `filter_map`：解析+过滤

```rust
let strings = vec!["1", "abc", "3", "four", "5"];

// 尝试解析每个字符串为数字，失败的跳过
let numbers: Vec<i32> = strings.iter()
    .filter_map(|s| s.parse().ok())
    .collect();
println!("{numbers:?}"); // [1, 3, 5]
```

### 4. Option 的组合子流水线

```rust
// 模拟数据库查询
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn get_email(name: &str) -> Option<String> {
    Some(format!("{name}@example.com"))
}

// 链式处理：找用户 → 获取邮件 → 转大写
let result = find_user(1)
    .as_deref()
    .and_then(|name| get_email(name))
    .map(|email| email.to_uppercase());

println!("{result:?}"); // Some("ALICE@EXAMPLE.COM")

let not_found = find_user(99)
    .as_deref()
    .and_then(|name| get_email(name))
    .map(|email| email.to_uppercase());

println!("{not_found:?}"); // None
```

---

## 完整运行示例

```rust
fn main() {
    println!("=== Option 作为迭代器 ===");
    let opts: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];

    print!("extend 逐一加入: ");
    let mut v: Vec<i32> = Vec::new();
    for opt in &opts {
        v.extend(opt); // Some → 加入一个元素，None → 不加
    }
    println!("{v:?}");
    println!();

    println!("=== flatten：批量去 None ===");
    let cleaned: Vec<i32> = opts.iter().flatten().copied().collect();
    println!("flatten 结果: {cleaned:?}");
    println!();

    println!("=== filter_map：解析+过滤 ===");
    let mixed = vec!["42", "hello", "7", "world", "100", "-1"];
    let positives: Vec<i32> = mixed.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&n| n > 0)
        .collect();
    println!("正整数: {positives:?}");
    println!();

    println!("=== Option 组合子链 ===");
    fn parse_and_double(s: &str) -> Option<i32> {
        s.parse::<i32>().ok().map(|n| n * 2)
    }

    let inputs = ["5", "abc", "10", "0", "invalid"];
    let results: Vec<Option<i32>> = inputs.iter()
        .map(|s| parse_and_double(s))
        .collect();
    println!("raw results: {results:?}");

    let valid: Vec<i32> = results.into_iter().flatten().collect();
    println!("valid results: {valid:?}");
    println!();

    println!("=== transpose：Option<Result> ↔ Result<Option> ===");
    // 从字符串解析数字，可能失败，且字符串本身可能缺失
    let inputs: Vec<Option<&str>> = vec![Some("42"), None, Some("bad"), Some("10")];

    let parsed: Vec<Option<Result<i32, _>>> = inputs.iter()
        .map(|opt| opt.map(|s| s.parse::<i32>()))
        .collect();

    // 把 Option<Result<i32, E>> 转成 Result<Option<i32>, E>
    for (i, item) in parsed.iter().enumerate() {
        match item {
            None => println!("  [{i}] 缺失"),
            Some(Ok(n)) => println!("  [{i}] 成功: {n}"),
            Some(Err(_)) => println!("  [{i}] 解析失败"),
        }
    }
    println!();

    println!("=== 实用：处理可选配置 ===");
    struct Config {
        host: Option<String>,
        port: Option<u16>,
        timeout: Option<u64>,
    }

    let cfg = Config {
        host: Some("localhost".into()),
        port: None,   // 使用默认
        timeout: Some(30),
    };

    let host = cfg.host.as_deref().unwrap_or("127.0.0.1");
    let port = cfg.port.unwrap_or(8080);
    let timeout = cfg.timeout.unwrap_or(60);
    println!("  连接: {host}:{port} (超时 {timeout}s)");
}
```

---

## 实际工程场景

### 1. 批量解析（如 CSV 处理）

```rust
fn parse_row(row: &str) -> Option<(String, i32)> {
    let parts: Vec<&str> = row.split(',').collect();
    if parts.len() != 2 { return None; }
    let name = parts[0].trim().to_string();
    let score = parts[1].trim().parse::<i32>().ok()?;
    Some((name, score))
}

let csv = "Alice,95\nBob,abc\nCarol,87\nDave,\nEve,92";
let valid_records: Vec<(String, i32)> = csv.lines()
    .filter_map(|line| parse_row(line))
    .collect();

println!("{valid_records:?}");
// [("Alice", 95), ("Carol", 87), ("Eve", 92)]
// Bob 和 Dave 的无效数据被自动跳过
```

---

## 我的理解与记忆方法

**Option 在迭代器里的角色**：

```
Some(x) = 包含一个元素的迭代器
None    = 空迭代器

因此：
  flatten  = 把所有 Some 里的元素取出来，None 产生零个
  filter_map = 对每个元素 map（可能返回 Option），然后自动 flatten
```

---

## 下一步

第 8 章完成！你已经掌握了 Rust 函数式编程的完整工具箱。

- 回到目录：[第 8 章：函数式编程](./README.md)
- 下一章：[第 8 章：生命周期与智能指针](../chapter08/README.md)
