# 6. 分层结果：`Option<Result<T, E>>`

> - **所属章节**：第 11 章 · Error Handling
> - **Cargo package**：`chapter11`
> - **运行方式**：`cargo run -p chapter11`
> - **代码位置**：`chapters/chapter11/src/topic_06_layered_outcomes_result_option_part2.rs`
> - **上一篇**：[5. 分层结果 Result<Option>](./5-分层结果类型：ResultOption.md)
> - **下一篇**：[7. anyhow](./7-anyhow.md)
> - **关键词**：`Option<Result<T, E>>`、可选操作、条件执行、`filter_map`

---

## 这一节解决什么问题

与上一篇相反——有时候**操作本身是可选的**：

- 表单字段是可选的：用户可以不填
- 某个配置项不存在时跳过处理
- 批量操作中某些元素不需要处理

`Option<Result<T, E>>` 表示：
- `None`：根本不需要执行这个操作
- `Some(Ok(v))`：执行了，成功了
- `Some(Err(e))`：执行了，失败了

---

## 详细原理

```rust
fn maybe_parse(input: Option<&str>) -> Option<Result<i32, String>> {
    input.map(|s| {
        s.trim().parse::<i32>()
            .map_err(|e| format!("解析失败: {e}"))
    })
}

// None → 什么都不做
// Some("42") → Some(Ok(42))
// Some("abc") → Some(Err("解析失败: ..."))
for input in [None, Some("42"), Some("bad")] {
    match maybe_parse(input) {
        None          => println!("  跳过（没有输入）"),
        Some(Ok(n))   => println!("  成功: {n}"),
        Some(Err(e))  => println!("  失败: {e}"),
    }
}
```

---

## `transpose()` 方向

```rust
// Option<Result<T, E>> → Result<Option<T>, E>
let opt_res: Option<Result<i32, &str>> = Some(Ok(5));
let transposed: Result<Option<i32>, &str> = opt_res.transpose();
// Ok(Some(5)) → 便于在 ? 链里使用
```

---

## 完整运行示例

```rust
fn main() {
    // 模拟表单处理：某些字段是可选的
    let forms = vec![
        // (name, optional_age)
        ("Alice", Some("25")),
        ("Bob", None),        // 没有填年龄
        ("Carol", Some("abc")), // 填了但格式错误
    ];

    for (name, age_input) in &forms {
        let age_result: Option<Result<u32, _>> = age_input.map(|s| {
            s.trim().parse::<u32>()
                .map_err(|_| format!("'{s}' 不是有效年龄"))
        });

        match age_result {
            None             => println!("{name}: 年龄未填写"),
            Some(Ok(age))    => println!("{name}: 年龄 {age}"),
            Some(Err(e))     => println!("{name}: 年龄格式错误 - {e}"),
        }
    }
    println!();

    // 使用 filter_map 只收集成功的可选值
    let optional_values: Vec<Option<&str>> = vec![
        Some("1"), None, Some("bad"), Some("3"), None, Some("5")
    ];

    let valid_numbers: Vec<i32> = optional_values.iter()
        .filter_map(|opt| {
            opt.and_then(|s| s.parse().ok())
        })
        .collect();

    println!("有效数字（跳过 None 和解析失败）: {valid_numbers:?}");
}
```

---

## Result<Option> vs Option<Result> 对比

| | `Result<Option<T>, E>` | `Option<Result<T, E>>` |
|-|----------------------|----------------------|
| **操作是否执行** | 一定执行 | 可能不执行 |
| **None 含义** | 查询成功但无结果 | 操作根本不触发 |
| **典型场景** | 数据库查询、搜索 | 可选表单字段、条件处理 |

---

## 下一步

- 继续阅读：[7. anyhow](./7-anyhow.md)
- 回到目录：[第 11 章：错误处理](./README.md)
