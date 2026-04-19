# 4. Option：显式表达"可能没有值"

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_04_option_type.rs`
> - **上一篇**：[3. Enum](./3-Enum.md)
> - **下一篇**：[5. Result](./5-Result.md)
> - **关键词**：`Option<T>`、`Some`、`None`、`unwrap`、`?`、`map`、`and_then`、空指针替代

---

## 这一节解决什么问题

Java、Python、C 里，"没有值"通常用 `null`/`None`/`0`/`-1` 表示。这很危险——你可能忘记检查，然后在运行时崩溃（NullPointerException）。

Rust 彻底消除了 `null`：用 `Option<T>` 类型级别地表达"可能没有值"——编译器强制你处理 `None` 的情况，忘记处理会编译失败。

---

## 一分钟结论

- `Option<T>` 是内置 enum：`Some(T)` 或 `None`
- 没有 `null`：编译器保证你不能在没有检查的情况下使用可能不存在的值
- 处理方式：`match`、`if let`、`?`（在返回 Option 的函数里）
- 组合子：`map`、`and_then`、`or`、`unwrap_or`、`unwrap_or_else`...
- `unwrap()` / `expect()`：只在你**确定**不会是 `None` 时使用

---

## 与其他语言对比


| 语言     | "没有值"的表示                              | 安全性                           |
| ------ | ------------------------------------- | ----------------------------- |
| Java   | `null`                                | 不安全：可能 NullPointerException   |
| Python | `None`                                | 不安全：AttributeError at runtime |
| C++    | `nullptr` / `std::optional<T>`（C++17） | 可选安全                          |
| Kotlin | `T?`（nullable type）                   | 编译期检查                         |
| Rust   | `Option<T>`                           | **完全安全：编译器强制处理**              |


---

## Option 的定义

```rust
// 标准库的 Option 定义（简化）
enum Option<T> {
    Some(T), // 有值
    None,    // 没有值
}
```

---

## 详细原理

### 1. 基础使用

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// 处理 Option
match divide(10.0, 2.0) {
    Some(result) => println!("结果: {result}"),
    None => println!("除以零！"),
}

// if let（只关心 Some 的情况）
if let Some(x) = divide(10.0, 3.0) {
    println!("有值: {x:.2}");
}
```

### 2. 组合子（链式处理）

```rust
let maybe_name: Option<String> = Some("  alice  ".to_string());

let result = maybe_name
    .map(|s| s.trim().to_string())      // 去掉空格
    .filter(|s| !s.is_empty())           // 过滤空字符串
    .map(|s| {
        let mut chars = s.chars();
        match chars.next() {
            None => s,
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }); // 首字母大写

println!("{result:?}"); // Some("Alice")
```

### 3. `and_then`：链式 Option 操作

```rust
fn find_user_by_name(name: &str) -> Option<u32> {
    match name {
        "alice" => Some(1),
        "bob" => Some(2),
        _ => None,
    }
}

fn get_email(user_id: u32) -> Option<String> {
    match user_id {
        1 => Some("alice@example.com".to_string()),
        _ => None,
    }
}

// and_then 类似于 flatMap：如果 Some，执行并返回新 Option
let email = find_user_by_name("alice")
    .and_then(|id| get_email(id));
println!("{email:?}"); // Some("alice@example.com")

let no_email = find_user_by_name("unknown")
    .and_then(|id| get_email(id));
println!("{no_email:?}"); // None（因为找不到用户）
```

### 4. 默认值

```rust
let value: Option<i32> = None;

// unwrap_or：None 时用默认值
let n1 = value.unwrap_or(42);           // 42

// unwrap_or_else：None 时调用闭包生成默认值（惰性）
let n2 = value.unwrap_or_else(|| 10 + 20); // 30

// or：None 时用另一个 Option
let n3 = value.or(Some(99));             // Some(99)

// unwrap_or_default：None 时用类型默认值
let n4: i32 = value.unwrap_or_default(); // 0（i32 的默认值）

println!("{n1}, {n2}, {n3:?}, {n4}");
```

---

## 完整运行示例

```rust
fn search(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

fn get_first_positive(nums: &[i32]) -> Option<i32> {
    nums.iter().find(|&&x| x > 0).copied()
}

fn parse_and_validate(s: &str) -> Option<u32> {
    s.parse::<u32>()
        .ok()
        .filter(|&n| n > 0 && n < 1000)
}

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("=== 基础查找 ===");
    for target in [5, 7, 9] {
        match search(&data, target) {
            Some(idx) => println!("  找到 {target} 在位置 {idx}"),
            None => println!("  找不到 {target}"),
        }
    }
    println!();

    println!("=== 链式组合子 ===");
    let result = search(&data, 9)
        .map(|idx| data[idx])
        .filter(|&n| n > 5)
        .map(|n| n * 2);
    println!("  找到 9，大于5，乘以2: {result:?}");  // Some(18)

    let no_result = search(&data, 100)
        .map(|idx| data[idx]);
    println!("  找不到100: {no_result:?}");  // None
    println!();

    println!("=== 默认值处理 ===");
    let mixed = vec![-3, -1, 0, 2, 5];
    let first_pos = get_first_positive(&mixed);
    println!("  第一个正数: {:?}", first_pos);                    // Some(2)
    println!("  不存在时用默认值: {}", first_pos.unwrap_or(0));   // 2
    println!("  没有正数时: {:?}", get_first_positive(&[-1, -2])); // None
    println!();

    println!("=== 解析验证 ===");
    for input in ["42", "0", "999", "1001", "abc"] {
        let result = parse_and_validate(input);
        println!("  {:?} → {:?}", input, result);
    }
}
```

---

## 注意点与陷阱

### 陷阱：`unwrap()` 在 None 时 panic

```rust
let x: Option<i32> = None;
// x.unwrap(); // ❌ thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'

// ✅ 处理 None 的安全方式
let val = x.unwrap_or(0);   // 或者
let val = x.unwrap_or_else(|| compute_default());
// 或者
if let Some(v) = x { use_value(v); }
```

---

## 下一步

- 继续阅读：[5. Result](./5-Result.md)
- 回到目录：[第 3 章：自定义类型](./README.md)

