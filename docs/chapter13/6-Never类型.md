# 6. Never 类型 `!`：永不返回的类型

> - **所属章节**：第 13 章 · Understanding Size in Rust
> - **Cargo package**：`chapter13`
> - **运行方式**：`cargo run -p chapter13`
> - **代码位置**：`chapters/chapter13/src/topic_06_zero_sized_types_never_type.rs`
> - **Lab**：`chapters/chapter13/src/lab.rs`
> - **上一篇**：[5. Unsized Coercion（非固定大小强转）](./5-unsized强转.md)
> - **下一篇**：[7. Unit 类型 `()`](./7-Unit类型.md)
> - **关键词**：`!`、never type、bottom type、diverging function、`panic!`、`loop`、`return`、类型系统底类型

---

## 这一节解决什么问题

考虑这段代码：

```rust
let result: i32 = if condition {
    42
} else {
    panic!("something went wrong!")
};
```

`if` 的两个分支类型不同：`42` 是 `i32`，`panic!` 什么都不产生。这段代码却能编译通过——为什么？

答案是：`panic!` 宏展开后的表达式类型是 `!`（念作 "never"）。`!` 是 Rust 类型系统里的**底类型（bottom type）**，可以被强制转换成任何类型，所以 `if` 的两个分支类型在统一时，`!` "退让"成了 `i32`。

这一节讲清楚：
- `!` 是什么，大小是多少
- 哪些表达式的类型是 `!`
- `!` 为什么可以"变成"任何类型（底类型理论）
- 在工程代码里 `!` 的实际用途

---

## 一分钟结论

- `!`（Never 类型）表示"这个表达式永远不会产生值"（函数永不返回）
- 大小：**0 字节**（它不可能存在，所以不占空间）
- 三类产生 `!` 类型的表达式：`panic!`、`loop {}`（不含 break）、`return`
- `!` 是**底类型（bottom type）**：可以被强制转换成任何类型 `T`
- 这让它可以"融入" `if/match` 的任意分支，而不破坏类型统一
- `->` 标注：`fn f() -> !` 表示这个函数永远不正常返回
- `!` 在 stable Rust 中作为返回类型完全可用；作为普通类型（如 `let x: ! = ...`）在 nightly 上

---

## 与其他语言对比

| 语言 | "永不返回"的表示方式 |
|-----|-----------------|
| C / C++ | `[[noreturn]] void f()` / `__attribute__((noreturn))`，不是类型系统的一部分 |
| Java | 没有对应概念，`throws` 只声明异常，不表示不返回 |
| Kotlin | `Nothing` 类型，语义相同（函数抛异常或无限循环）|
| Haskell | `Void` 类型（空类型），理论上是底类型 |
| Rust | `!` (Never type)，是类型系统一等公民，0字节，底类型 |

Kotlin 的 `Nothing` 和 Rust 的 `!` 设计思路最接近。

---

## 核心概念与心智模型

### `!` 作为底类型（Bottom Type）

类型系统理论里有一个概念叫"底类型"（bottom type）：它是所有类型的子类型。

```
类型层次（从宽到窄）：
  Any / Top type  ← 最宽泛，能接受所有类型
     ...
   i32
   String
   ...
  ! (Never) ← 最窄，是所有类型的子类型

"X 是 Y 的子类型" 意味着 "X 可以用在任何需要 Y 的地方"
因为 ! 是所有类型的子类型，所以 ! 可以用在任何需要 T 的地方
```

### 为什么 `!` 可以"变成"任何类型

这听起来奇怪，但逻辑很清晰：**如果一个表达式永远不产生值，那么讨论它的类型是什么根本没有意义**，编译器可以把它当作任何类型——反正它永远不会真的"给你一个值"。

```rust
// panic! 永远不会走完，所以这个 else 分支的类型 = !
// 编译器把 ! 当作 i32 来看待（底类型推断）
let n: i32 = if true {
    42          // i32
} else {
    panic!()    // ! → 被当成 i32
};
```

```
if 分支的类型统一过程：
  分支 1：i32
  分支 2：! (never)
     ↓
  统一后：i32（! 被"提升"为 i32）
```

---

## 详细原理

### 1. 产生 `!` 类型的表达式

```rust
// 这些表达式的类型是 !：

// 1. panic! 和相关宏
panic!("message");
unreachable!();
todo!();
unimplemented!();

// 2. 永不退出的 loop
loop {
    // 没有 break
}

// 3. 函数内的 return
return 42; // 类型是 !（控制流离开了当前表达式位置）

// 4. 函数内的 continue / break（在 loop 上下文里）
continue;
break;

// 5. std::process::exit
std::process::exit(1);
```

### 2. `-> !` 函数的用途

```rust
// 恐慌（panic）
fn crash(msg: &str) -> ! {
    panic!("{}", msg);
}

// 无限服务循环（不应该退出）
fn run_server() -> ! {
    loop {
        // 处理请求...
    }
}

// 进程退出
fn exit_with_code(code: i32) -> ! {
    std::process::exit(code);
}

// 错误处理中的 abort
fn abort_on_oom() -> ! {
    eprintln!("Out of memory!");
    std::process::abort();
}
```

### 3. `!` 在 `match` 中的应用

```rust
fn parse_positive(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(n) if n > 0 => n,
        Ok(n) => panic!("expected positive, got {}", n),  // ! → i32
        Err(e) => panic!("parse error: {}", e),          // ! → i32
    }
}
// 整个 match 的类型：i32（! 被"提升"成 i32）
```

### 4. `Result::unwrap_or_else` 中的 `!`

```rust
// unwrap_or_else 的签名：
// fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T

// 当 op 的返回类型是 ! 时，这个也合法（因为 ! 是 T 的子类型）：
let n = some_result.unwrap_or_else(|e| {
    panic!("fatal: {}", e)  // 返回类型 !，满足 F: FnOnce(E) -> T
});
```

---

## 完整运行示例

```rust
use std::fmt;

// 1. 基本的 -> ! 函数
fn fail(msg: &str) -> ! {
    panic!("Fatal error: {}", msg);
}

// 2. ! 在 if/else 中统一类型
fn get_value(condition: bool) -> i32 {
    if condition {
        42
    } else {
        // panic! 的类型是 !，被统一为 i32
        fail("condition was false")
    }
}

// 3. ! 在 match 中
#[derive(Debug)]
enum Command { Quit, Run(i32) }

fn execute(cmd: Command) -> i32 {
    match cmd {
        Command::Run(n) => n * 2,
        Command::Quit => {
            println!("Quitting...");
            std::process::exit(0); // 类型是 !，match 类型统一为 i32
        }
    }
}

// 4. loop {} 的类型是 !
fn countdown() -> i32 {
    let mut n = 0;
    loop {
        n += 1;
        if n >= 5 {
            break n; // break 带值时，loop 的类型是 break 后的值的类型（i32）
        }
    }
    // break 前的 loop 内容是一系列 !，break 值是 i32，整体 loop 是 i32
}

// 5. ! 的大小真的是 0 字节
fn size_demo() {
    // stable Rust 不能直接用 size_of::<!>()，用枚举验证
    enum Void {}  // 无成员枚举，类似 !
    println!("size_of::<Void>() = {}", std::mem::size_of::<Void>()); // 0
    println!("(理论上 size_of::<!>() 也应该是 0)");
}

fn main() {
    // 2. if/else 类型统一
    println!("get_value(true)  = {}", get_value(true));
    // get_value(false) 会 panic，先不调用

    // 3. match 类型统一
    println!("execute(Run(10)) = {}", execute(Command::Run(10)));

    // 4. loop break 值
    println!("countdown()      = {}", countdown()); // 5

    // 5. 大小
    size_demo();

    println!("\n--- ! 作为 unwrap_or_else 的闭包返回类型 ---");
    let s = "42";
    let n: i32 = s.parse().unwrap_or_else(|e| {
        // 这里返回 !，编译器把它当成 i32
        panic!("parse error: {}", e)
    });
    println!("parsed: {}", n);
}
```

**预期输出**：

```text
get_value(true)  = 42
execute(Run(10)) = 20
countdown()      = 5
size_of::<Void>() = 0
(理论上 size_of::<!>() 也应该是 0)

--- ! 作为 unwrap_or_else 的闭包返回类型 ---
parsed: 42
```

---

## 编译器错误分析

### ❌ 非 diverging 函数标注了 `-> !`

```rust
fn bad() -> ! {
    42 // ❌ 这里正常返回了一个值
}
```

```text
error[E0308]: mismatched types
  |
3 |     42
  |     ^^
  |     expected `!`, found integer
  = note: expected type `!`
           found type `{integer}`
```

**修复**：函数体必须真的不返回（panic / loop / exit）

### ❌ `let x: !` 在 stable Rust 里不能用

```rust
let x: ! = panic!(); // ❌ 在 stable Rust 里这样的绑定有限制
// 需要 nightly + #![feature(never_type)] 才能作为普通类型变量
```

---

## 实际工程场景

### 1. 错误处理的 "abort path"

```rust
fn require<T>(opt: Option<T>, msg: &str) -> T {
    opt.unwrap_or_else(|| {
        eprintln!("FATAL: {msg}");
        std::process::exit(1); // -> !
    })
}
```

### 2. 测试中的 unreachable 分支

```rust
fn should_be_even(n: i32) -> i32 {
    match n % 2 {
        0 => n,
        _ => unreachable!("expected even number, got {}", n), // !
    }
}
```

### 3. 区分"没有值"（`Option`）和"不可能的值"（`!`）

```rust
// Option<T>：可能有值，也可能没有（None 是合法的空状态）
// !：根本不可能到达这里（不是空，是"不存在"）

// 在类型系统里用 ! 表达"这个分支不可能存在"
type Infallible = std::convert::Infallible; // = !
// impl From<Infallible> for AnyType 的实现体可以是 unreachable!()
```

---

## 性能影响

`!` 类型的代码路径会让编译器知道**这段代码不可达**，从而：
- 死代码消除（DCE）更彻底
- `match` 的 arm 如果只有 `!` 分支，不产生任何代码
- 函数标注 `-> !` 让调用方的编译器优化器知道不需要为这个函数的"返回"做准备（例如不需要保存返回地址附近的寄存器）

---

## 注意点与陷阱

### 陷阱 1：`loop` 含 break 时类型不再是 `!`

```rust
// 这个 loop 类型是 !（永不 break）
let _ = loop { };

// 这个 loop 类型是 i32（break 带了值）
let x: i32 = loop {
    break 42;
};

// 这个 loop 类型是 ()（break 没有值）
let (): () = loop {
    break;
};
```

### 陷阱 2：`todo!()` 和 `unimplemented!()` 是 `!`，但不代表"不需要实现"

```rust
fn work_in_progress(x: i32) -> String {
    todo!() // 编译通过，类型 ! → String，但运行时会 panic！
}
// 这只是暂时占位，不能忘记实现
```

---

## 我的理解与记忆方法

**核心直觉**：

> `!` 是"死路"类型。到了死路，后面什么都没有，所以它"兼容"任何类型——反正你到不了那里去取值。

**决策树**：

```
看到 panic! / unreachable! / todo! / return / loop(无break) → 类型是 !
                                                              ↓
                                         编译器在类型统一时把它当 T 看待
```

**与 `Option::None` 的区别**：

```
None → "这里可以没有值，这是合法的空状态"
!    → "这里根本到不了，类型系统层面的'不可能'"
```

---

## 下一步

下一篇讲 Unit 类型 `()`——Rust 里"什么也不表示"的类型，和 `!` 的对比很有意思。

- 继续阅读：[7. Unit 类型 `()`](./7-Unit类型.md)
- 回到目录：[第 13 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Reference - Never type](https://doc.rust-lang.org/reference/types/never.html)
- RFC 1216：[! Type (Never Type RFC)](https://github.com/rust-lang/rfcs/blob/master/text/1216-bang-type.md)
