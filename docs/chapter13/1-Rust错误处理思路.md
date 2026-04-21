# 1. Rust 的错误处理思路

> - **所属章节**：第 12 章 · Error Handling
> - **Cargo package**：`chapter13`
> - **运行方式**：`cargo run -p chapter13`
> - **代码位置**：`chapters/chapter13/src/topic_01_rust_error_handling_approach.rs`
> - **上一篇**：本章第一篇
> - **下一篇**：[2. 传播错误](./2-传播错误.md)
> - **关键词**：`panic!`、`Result`、`Option`、不可恢复错误、可恢复错误、错误处理哲学

---

## 这一节解决什么问题

其他语言（Java、Python、C#）用**异常（exception）**处理错误：抛出一个异常，某个 `catch` 块接住，中间的调用栈可能完全不知道异常在传播。

Rust 不同：**错误就是普通的返回值**。

这个设计选择带来了：
- 调用方**必须**处理错误（类型系统强制）
- 错误传播路径**完全显式**（没有隐式的调用栈展开）
- **零成本**：没有异常运行时开销

---

## 一分钟结论

Rust 的错误分两类：

| 类型 | 机制 | 何时使用 |
|-----|-----|---------|
| **不可恢复** | `panic!` | bug、不变量被破坏、程序无法继续 |
| **可恢复** | `Result<T, E>` | 预期中的失败（文件不存在、网络超时、解析失败）|

额外：
- `Option<T>`：值可能不存在（不是"错误"，而是"空"）
- `unwrap()`/`expect()`：临时开发时可以；生产代码应该处理所有情况

---

## 与其他语言对比

| 语言 | 错误机制 | 调用方是否强制处理 |
|-----|---------|---------------|
| Java | 异常（checked/unchecked）| checked 是强制的，但常被 `throws` 逃掉 |
| Python | 异常 | 不强制 |
| Go | 返回 `(value, error)` | 惯例强制，但编译器不强制 |
| Rust | `Result<T, E>` 或 `panic!` | **编译器强制**（`#[must_use]` + 穷尽匹配）|

---

## 详细原理

### 1. `panic!`：不可恢复错误

```rust
// 以下情况应该 panic（不是真正的错误，是 bug）：
fn array_element(arr: &[i32], i: usize) -> i32 {
    if i >= arr.len() {
        panic!("索引越界：i={}, len={}", i, arr.len());
    }
    arr[i]
}

// 常见的会自动 panic 的情况：
let v = vec![1, 2, 3];
// v[10]; // index out of bounds: panic!
// let n: i32 = "abc".parse().unwrap(); // called `Result::unwrap()` on `Err`
```

`panic!` 默认展开调用栈（打印 backtrace）；可以通过 `RUST_BACKTRACE=1` 看完整栈。

### 2. `Result<T, E>`：可恢复错误

```rust
use std::num::ParseIntError;

fn parse_port(s: &str) -> Result<u16, ParseIntError> {
    s.trim().parse::<u16>()
}

match parse_port("8080") {
    Ok(port) => println!("端口: {port}"),
    Err(e)   => println!("解析失败: {e}"),
}

// 链式处理
let port = parse_port("  9090  ")
    .unwrap_or(80); // 失败时用默认值

println!("使用端口: {port}");
```

### 3. `Option<T>`：值可能不存在

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,  // 不是错误，只是没找到
    }
}

// Option 的方法
let name = find_user(1).unwrap_or("匿名".to_string());
let upper = find_user(2).map(|n| n.to_uppercase());
let len = find_user(3).as_deref().map(str::len);

println!("{name}, {upper:?}, {len:?}");
```

### 4. `unwrap` vs `expect`

```rust
// unwrap()：失败时 panic，没有上下文
let n: i32 = "42".parse().unwrap();

// expect()：失败时 panic，但有更好的错误信息
let port: u16 = "8080".parse().expect("端口必须是有效的 u16 数字");

// 适用场景：
// - 测试代码：可以用 unwrap（失败直接 panic 出测试错误）
// - 原型代码：可以用 unwrap（先让代码跑起来）
// - 生产代码：应该用 ? 或 match 正确处理
```

---

## 完整运行示例

```rust
use std::{fs, num::ParseIntError};

fn demonstrate_panic() {
    println!("=== panic! 演示（注意：不要在生产代码里无谓 panic）===");

    // 安全的：提前检查
    fn safe_divide(a: i32, b: i32) -> i32 {
        if b == 0 { panic!("除数不能为零"); }
        a / b
    }

    println!("10 / 2 = {}", safe_divide(10, 2));
    // safe_divide(10, 0); // ← 取消注释会 panic
    println!();
}

fn demonstrate_result() {
    println!("=== Result<T, E> 演示 ===");

    fn parse_int(s: &str) -> Result<i32, ParseIntError> {
        s.trim().parse()
    }

    let inputs = ["42", "abc", " -10 ", "999999999999"];
    for input in inputs {
        match parse_int(input) {
            Ok(n) => println!("  ✅ {:?} → {n}", input),
            Err(e) => println!("  ❌ {:?} → {e}", input),
        }
    }
    println!();
}

fn demonstrate_option() {
    println!("=== Option<T> 演示 ===");

    let numbers = vec![1, 3, 5, 7, 9, 10];

    println!("  第一个偶数: {:?}", numbers.iter().find(|&&x| x % 2 == 0));
    println!("  最大值: {:?}", numbers.iter().max());
    println!("  找 99: {:?}", numbers.iter().find(|&&x| x == 99));

    // Option 组合
    let result = numbers.iter()
        .find(|&&x| x > 6)
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0);
    println!("  大于6的数*2且能被3整除: {result:?}");
    println!();
}

fn main() {
    demonstrate_panic();
    demonstrate_result();
    demonstrate_option();

    println!("=== 原则总结 ===");
    println!("  panic! → 不变量被破坏、不可能的情况、bug");
    println!("  Result → 预期中的失败（IO、解析、网络）");
    println!("  Option → 值可能不存在（不是错误）");
    println!("  unwrap → 只在你确定不会失败或测试代码中使用");
}
```

---

## 下一步

- 继续阅读：[2. 传播错误](./2-传播错误.md)
- 回到目录：[第 12 章：错误处理](./README.md)
- 官方参考：[The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
