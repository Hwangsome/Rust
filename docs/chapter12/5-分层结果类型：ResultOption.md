# 5. 分层结果：`Result<Option<T>, E>`

> - **所属章节**：第 12 章 · Error Handling
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_05_layered_outcomes_result_option_part1.rs`
> - **上一篇**：[4. 链式调用中的 ? 运算符](./4-链式调用中的问号运算符.md)
> - **下一篇**：[6. 分层结果 Option<Result>](./6-分层结果类型：OptionResult.md)
> - **关键词**：`Result<Option<T>>`, 三态结果, 数据库查询, transpose

---

## 这一节解决什么问题

查询数据库时有三种结果：

1. **操作成功，找到了数据** → `Ok(Some(data))`
2. **操作成功，但没有数据** → `Ok(None)`
3. **操作失败**（网络错误、SQL 错误）→ `Err(e)`

这是 `Result<Option<T>, E>` 的典型场景——**操作一定会执行，但结果可能"找不到"**。

关键区分：`Ok(None)` 不是错误，只是"正常地没有结果"。不要把它当 `Err` 处理。

---

## 详细原理

```rust
fn find_user_by_id(id: u32) -> Result<Option<String>, String> {
    if id == 0 {
        return Err("ID 不能为 0".to_string());  // 真正的错误
    }
    match id {
        1 => Ok(Some("Alice".to_string())),
        2 => Ok(Some("Bob".to_string())),
        _ => Ok(None),  // 正常查询，但没有结果
    }
}

// 三种情况的处理
for id in [0, 1, 2, 99] {
    match find_user_by_id(id) {
        Ok(Some(user)) => println!("  [{}] 找到用户: {}", id, user),
        Ok(None)       => println!("  [{}] 用户不存在", id),
        Err(e)         => println!("  [{}] 查询失败: {}", id, e),
    }
}
```

---

## `transpose()` 转换

```rust
// Option<Result<T, E>> → Result<Option<T>, E>
let opt_result: Option<Result<i32, &str>> = Some(Ok(42));
let result_opt: Result<Option<i32>, &str> = opt_result.transpose();
println!("{result_opt:?}"); // Ok(Some(42))

let opt_err: Option<Result<i32, &str>> = Some(Err("failed"));
let result_err: Result<Option<i32>, &str> = opt_err.transpose();
println!("{result_err:?}"); // Err("failed")

let opt_none: Option<Result<i32, &str>> = None;
let result_none: Result<Option<i32>, &str> = opt_none.transpose();
println!("{result_none:?}"); // Ok(None)
```

---

## 完整运行示例

```rust
use std::collections::HashMap;

// 模拟数据库
fn query_db(key: &str) -> Result<Option<i32>, String> {
    let db: HashMap<&str, i32> = [("a", 1), ("b", 2), ("c", 3)].into_iter().collect();

    if key.contains('#') {
        return Err(format!("无效键: {key}"));
    }

    Ok(db.get(key).copied()) // Some(value) 或 None
}

fn process_query(key: &str) -> Result<String, String> {
    match query_db(key)? {  // ? 传播 Err
        Some(value) => Ok(format!("找到: {key} = {value}")),
        None => Ok(format!("未找到: {key}")),
    }
}

fn main() {
    println!("=== Result<Option<T>> 三态处理 ===");
    for key in ["a", "b", "z", "x#y"] {
        match process_query(key) {
            Ok(msg) => println!("  ✅ {msg}"),
            Err(e) => println!("  ❌ {e}"),
        }
    }
    println!();

    println!("=== transpose: Option<Result> ↔ Result<Option> ===");
    let cases: Vec<Option<Result<i32, &str>>> = vec![
        Some(Ok(42)),
        None,
        Some(Err("bad")),
    ];

    for c in cases {
        let transposed: Result<Option<i32>, &str> = c.transpose();
        println!("  → {transposed:?}");
    }
}
```

---

## 下一步

- 继续阅读：[6. 分层结果 Option<Result>](./6-分层结果类型：OptionResult.md)
- 回到目录：[第 12 章：错误处理](./README.md)
