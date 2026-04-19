# 5. Result<T, E>：显式的错误处理

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_05_result_type.rs`
> - **上一篇**：[4. Option](./4-Option.md)
> - **下一篇**：[6. HashMap](./6-HashMap.md)
> - **关键词**：`Result<T, E>`、`Ok`、`Err`、`?`、`map_err`、`unwrap`、错误传播

---

## 这一节解决什么问题

在 Java 里，方法可能抛出异常——但你看函数签名根本看不出来！你必须查文档或试错。

Rust 的 `Result<T, E>` 把"可能失败"写进类型签名：

```rust
fn read_file(path: &str) -> Result<String, io::Error>
//                          ^^^^^^^^^^^^^^^^^^^^^^^^
//                          一眼看出：成功时是 String，失败时是 io::Error
```

调用方**必须**处理错误，否则编译器给出 `unused Result` 警告（`#[must_use]`）。

---

## 一分钟结论

- `Result<T, E>` 是内置 enum：`Ok(T)` 或 `Err(E)`
- `?` 操作符：在函数里传播错误（遇到 `Err` 立刻返回）
- 组合子：`map`、`map_err`、`and_then`、`or_else`、`unwrap_or`...
- `?` 要求函数返回 `Result` 或 `Option`
- 与 `Option` 的关系：`result.ok()` → `Option`；`option.ok_or(err)` → `Result`

---

## Result 的定义

```rust
enum Result<T, E> {
    Ok(T),   // 成功，携带值
    Err(E),  // 失败，携带错误
}
```

---

## 详细原理

### 1. 返回 Result

```rust
use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<u8, ParseIntError> {
    s.trim().parse::<u8>()
}

// 处理
match parse_age("30") {
    Ok(age) => println!("年龄: {age}"),
    Err(e)  => println!("解析错误: {e}"),
}
```

### 2. `?` 操作符（最重要！）

```rust
fn load_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.trim().parse()?; // 失败时提前返回 Err
    Ok(n * 2)
}
```

`?` 等价于：

```rust
let n: i32 = match s.trim().parse() {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
};
```

### 3. 链式处理

```rust
let result = "42"
    .parse::<i32>()           // Result<i32, ParseIntError>
    .map(|n| n * 2)            // Ok(84) 或传播 Err
    .map_err(|e| e.to_string()); // 把错误类型转换成 String

println!("{result:?}"); // Ok(84)
```

---

## 完整运行示例

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    TooLarge(i32),
    TooSmall(i32),
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::ParseError(e) }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "解析失败: {e}"),
            AppError::TooLarge(n) => write!(f, "{n} 太大（最大 100）"),
            AppError::TooSmall(n) => write!(f, "{n} 太小（最小 0）"),
        }
    }
}

fn parse_score(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.trim().parse()?;  // ParseIntError → AppError 通过 From
    if n > 100 { return Err(AppError::TooLarge(n)); }
    if n < 0   { return Err(AppError::TooSmall(n)); }
    Ok(n)
}

fn main() {
    println!("=== Result 基础 ===");
    let inputs = ["85", "abc", "120", "-5", "100"];

    for input in inputs {
        match parse_score(input) {
            Ok(score) => println!("  ✅ {:?} → 成绩: {score}", input),
            Err(e) => println!("  ❌ {:?} → {e}", input),
        }
    }
    println!();

    println!("=== Result 组合子 ===");
    let double_score = parse_score("40")
        .map(|n| n * 2)
        .unwrap_or(0);
    println!("40 分 × 2 = {double_score}");

    // and_then：只有成功时才继续
    let result = parse_score("75")
        .and_then(|n| {
            if n >= 60 { Ok("及格") } else { Err(AppError::TooSmall(n)) }
        });
    println!("75 分是否及格: {:?}", result);
    println!();

    println!("=== ? 操作符 ===");
    fn process(input: &str) -> Result<String, AppError> {
        let score = parse_score(input)?;  // 失败立刻返回
        let grade = match score {
            90..=100 => "A",
            80..=89  => "B",
            70..=79  => "C",
            60..=69  => "D",
            _        => "F",
        };
        Ok(format!("分数 {score} → 等级 {grade}"))
    }

    for input in ["95", "abc", "55"] {
        match process(input) {
            Ok(msg) => println!("  ✅ {msg}"),
            Err(e) => println!("  ❌ {e}"),
        }
    }
}
```

---

## Option vs Result 的选择


| 场景            | 用 Option                      | 用 Result                    |
| ------------- | ----------------------------- | --------------------------- |
| 值可能不存在（查找、索引） | ✅                             |                             |
| 操作可能失败（IO、解析） |                               | ✅                           |
| 转换：ok()       | `Option<T>` → `Result<T, ()>` |                             |
| 转换：ok_or(err) | `Option<T>` → `Result<T, E>`  |                             |
| 转换：err()      |                               | `Result<T,E>` → `Option<E>` |
| 转换：ok()       |                               | `Result<T,E>` → `Option<T>` |


---

## 下一步

- 继续阅读：[6. HashMap](./6-HashMap.md)
- 回到目录：[第 3 章：自定义类型](./README.md)

