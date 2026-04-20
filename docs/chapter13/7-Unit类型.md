# 7. Unit 类型 `()`：零大小的"空值"

> - **所属章节**：第 13 章 · Understanding Size in Rust
> - **Cargo package**：`chapter13`
> - **运行方式**：`cargo run -p chapter13`
> - **代码位置**：`chapters/chapter13/src/topic_07_zero_sized_types_unit_type.rs`
> - **Lab**：`chapters/chapter13/src/lab.rs`
> - **上一篇**：[6. Never 类型 `!`](./6-Never类型.md)
> - **下一篇**：[8. Unit 结构体与类型状态机](./8-Unit结构体.md)
> - **关键词**：`()`、unit type、ZST、`HashSet`、`Result<(), E>`、副作用函数、零大小类型

---

## 这一节解决什么问题

很多语言里，"没有返回值的函数"和"返回了 `void`" 的概念是模糊的。Java 有 `void`，但你不能把 `void` 当类型用，不能存进泛型容器里。

Rust 用 `()` 来统一这个概念：没有有意义的返回值的函数返回 `()`，带分号的代码块的值也是 `()`。更重要的是，`()` 是一个**正经的类型**，可以存进 `Vec<()>`，可以是 `Result<(), E>` 的成功值，可以是 `HashMap<String, ()>` 的 value（这就是 `HashSet` 的实现方式）。

这一节要讲清楚：
- `()` 是什么，为什么大小是 0
- 分号的语义：如何产生 `()`
- `()` 在标准库中的工程用途（HashSet、Result、Future）
- `()` 和 `!` 的根本区别

---

## 一分钟结论

- `()` 是 Rust 里的 **Unit 类型**，只有一个值：`()`（读作 "unit"）
- 大小：**0 字节**（值唯一，不需要存储任何信息）
- 没有返回值的函数实际上返回 `()`（等价于 `fn f() -> ()`）
- 带分号的语句块最后一个表达式被"丢弃"，整个块产出 `()`
- `HashSet<T>` 内部实际上是 `HashMap<T, ()>`——用 `()` 做 value 零开销
- `Result<(), E>` 表示"操作成功但不产生数据"（如写文件）
- `()` 和 `!` 的区别：`()` 是"有值，但信息量为零"；`!` 是"根本不会有值"

---

## 与其他语言对比

| 语言 | "没有有意义的返回值" | 能否作为类型使用 |
|-----|-----------------|--------------|
| C / C++ | `void`，不是类型，不能放进容器 | ❌ |
| Java | `void`（primitives）/ `Void`（boxed），`Void` 可以作为类型但很笨重 | 勉强 |
| Kotlin | `Unit`，完全是一个类型，可以放进泛型 | ✅ |
| Haskell | `()`（unit type），和 Rust 完全一样 | ✅ |
| Rust | `()`，0字节，完全的类型 | ✅ |

Kotlin 的 `Unit` 和 Rust 的 `()` 设计非常相似，都是真正的类型，大小为零（或 boxed 时为 0 字节值）。

---

## 核心概念与心智模型

### `()` 的唯一性：值只有一个

```
布尔型（bool）：两个值
  true
  false

Unit 类型（()）：一个值
  ()

Never 类型（!）：零个值（不可能存在）
  （空集）
```

正因为 `()` 只有一个可能的值，你永远不需要存储任何信息就能"知道"它的值是什么——所以它是 0 字节。

### 分号如何产生 `()`

```rust
let a = { 5 };    // 代码块最后是 5（表达式），a: i32 = 5
let b = { 5; };   // 代码块最后是 5;（语句），b: () = ()
//                  ↑ 分号把 5 变成了"被丢弃的语句"，整个块产出 ()
```

```
  表达式（expression）：有值
     ↓ 加分号
  语句（statement）：没有值（产出 ()）
```

---

## 详细原理

### 1. 函数返回 `()` 的多种写法

```rust
// 这四种写法完全等价：
fn do_work_1() { println!("working..."); }
fn do_work_2() -> () { println!("working..."); }
fn do_work_3() -> () { println!("working..."); () }
fn do_work_4() -> () { println!("working..."); return (); }
```

### 2. 分号和代码块的交互

```rust
let x1 = {
    let a = 1;
    let b = 2;
    a + b       // 没有分号，返回 3
};
println!("x1: {x1}"); // x1 = 3，类型 i32

let x2 = {
    let a = 1;
    let b = 2;
    a + b;      // 有分号，值被丢弃
};
println!("x2: {:?}", x2); // x2 = ()，类型 ()

// if 没有 else 时，整个 if 返回 ()（因为 else 分支隐式是 ()）
let x3 = if true { 1 } else { 0 };    // i32
let x4: () = if true { /* 什么都不写 */ }; // () - 两个分支都是 ()
```

### 3. `()` 的零大小特性

```rust
use std::mem::size_of;

println!("{}", size_of::<()>());         // 0
println!("{}", size_of::<((), ())>());   // 0（两个 () 组成的元组也是 0）
println!("{}", size_of::<[(); 1000]>()); // 0！1000个 () 的数组也是 0
```

这个特性被标准库大量利用。

### 4. `HashSet<T>` 就是 `HashMap<T, ()>`

```rust
// 标准库 HashSet 的（简化）定义：
pub struct HashSet<T, S = RandomState> {
    map: HashMap<T, (), S>,
}
// 存的 value 是 ()，大小 0，不浪费任何空间
// 只是用来"标记这个 key 存在"

// 对比：
let mut set: std::collections::HashSet<String> = HashSet::new();
set.insert("rust".to_string()); // 内部 map.insert("rust", ())
```

---

## 完整运行示例

```rust
use std::collections::{HashSet, HashMap};
use std::mem::size_of;
use std::io::Write;

fn greet(name: &str) {         // 隐式 -> ()
    println!("Hello, {name}!");
}

fn explicit_unit() -> () {     // 显式 -> ()
    println!("I return ()");
}

fn save_data(data: &[u8]) -> Result<(), String> {
    if data.is_empty() {
        return Err("data cannot be empty".to_string());
    }
    // 假装写入成功
    println!("Saved {} bytes", data.len());
    Ok(()) // 明确的"成功，但没有有用数据"
}

fn main() {
    println!("=== () 的大小 ===");
    println!("size_of::<()>()             = {}", size_of::<()>());          // 0
    println!("size_of::<((), ())>()       = {}", size_of::<((), ())>());    // 0
    println!("size_of::<[(); 1000]>()     = {}", size_of::<[(); 1000]>()); // 0
    println!();

    println!("=== 函数返回 () ===");
    let r1: () = greet("World");          // 接住返回的 ()
    let r2: () = explicit_unit();
    println!("greet 返回: {:?}", r1);    // ()
    println!("explicit 返回: {:?}", r2);
    println!();

    println!("=== 代码块与分号 ===");
    let a = { 5 };      // i32
    let b = { 5; };     // ()
    let c: () = {};     // ()
    println!("{{5}} = {a}");
    println!("{{5;}} = {:?}", b);
    println!("{{}} = {:?}", c);
    println!();

    println!("=== Result<(), E> 表示无数据的成功/失败 ===");
    match save_data(b"hello") {
        Ok(()) => println!("保存成功（无返回数据）"),
        Err(e) => println!("保存失败: {e}"),
    }
    match save_data(b"") {
        Ok(()) => println!("保存成功"),
        Err(e) => println!("保存失败: {e}"),
    }
    println!();

    println!("=== HashSet<T> = HashMap<T, ()> ===");
    let mut hs: HashSet<&str> = HashSet::new();
    hs.insert("rust");
    hs.insert("go");
    hs.insert("rust"); // 重复，不影响

    // 等价的手动实现：
    let mut hm: HashMap<&str, ()> = HashMap::new();
    hm.insert("rust", ());
    hm.insert("go", ());

    println!("HashSet: {:?}", hs);
    println!("等价 HashMap: {:?}", hm);
    println!();

    println!("=== Vec<()>：只关心长度，不关心内容 ===");
    // 有时候需要一个"只计数不存值"的集合
    let mut events: Vec<()> = Vec::new();
    events.push(());
    events.push(());
    events.push(());
    println!("事件发生了 {} 次（不存内容，零额外内存）", events.len());
}
```

---

## 编译器错误分析

### ❌ 把 `()` 当有值的类型使用

```rust
fn bad() -> i32 {
    println!("hello"); // println! 返回 ()
    // 这里隐式返回了 ()，但函数签名是 i32
}
```

```text
error[E0308]: mismatched types
  |
  | fn bad() -> i32 {
  |             ---  expected `i32` because of return type
  |     println!("hello");
  |     ^^^^^^^^^^^^^^^^^ expected `i32`, found `()`
```

**修复**：显式返回 i32，或修改签名为 `-> ()`

### ❌ if 没有 else 但类型不是 `()`

```rust
let x: i32 = if condition { 42 }; // ❌ 缺少 else，整体是 ()，不是 i32
```

```text
error[E0317]: `if` may be missing an `else` clause
```

**修复**：加 else 分支，或把 let 改为 `let x = ...`（类型推断为 `()`）

---

## 实际工程场景

### 1. `Result<(), E>` 是 I/O 函数的标准返回类型

```rust
use std::io;

fn write_config(path: &str, content: &str) -> io::Result<()> {
    let mut f = std::fs::File::create(path)?;
    f.write_all(content.as_bytes())?;
    Ok(()) // 成功写入，没有有用的返回数据
}
```

### 2. 错误处理链中的 `Ok(())` 检查

```rust
// 调用方不关心返回值，只关心是否成功
write_config("config.toml", "[server]\nport = 8080")?;

// 或者用 if let / match 来区分成/败：
if let Err(e) = write_config("config.toml", content) {
    eprintln!("配置写入失败: {}", e);
    return Err(e);
}
```

### 3. 异步编程：`Future<Output = ()>`

```rust
// async 函数没有返回值时，Future::Output = ()
async fn handle_request(req: Request) {
    println!("handling {}", req.path);
    // 不需要返回什么有意义的值
}
// handle_request: async fn() -> () → Future<Output = ()>
```

### 4. 泛型代码里的占位类型

```rust
// 在泛型状态机里，没有关联数据的状态用 () 表示
struct StateMachine<Data> {
    data: Data,
}
type EmptyState = StateMachine<()>; // 没有额外数据的状态
```

---

## `()` vs `!`：相似但截然不同

| 特征 | `()` (Unit) | `!` (Never) |
|-----|------------|------------|
| 可能的值 | 1 个（只有 `()`） | 0 个（不存在） |
| 大小 | 0 字节 | 0 字节 |
| 含义 | "操作完成，无数据" | "永远不会到达这里" |
| 能赋值给 `T` | 否 | 是（底类型） |
| 用在函数返回 | 副作用函数 | 永不返回的函数 |
| 用在 Result | `Ok(())` → 成功无数据 | N/A |
| 例子 | `println!()` 的返回值 | `panic!()` 的返回值 |

---

## 注意点与陷阱

### 陷阱 1：`println!` 返回 `()`，不是无类型

```rust
let r = println!("hello"); // r 的类型是 ()，不是"没有值"
println!("{:?}", r);       // 打印: ()
```

### 陷阱 2：条件赋值时 else 分支不能省略（除非两个分支都是 `()`）

```rust
let x = if cond { 42 };     // ❌ 没有 else，类型不统一
let x: () = if cond { };    // ✅ else 隐式是 ()，与 if 分支的 () 统一
```

---

## 我的理解与记忆方法

**核心直觉**：

> `()` 就像一个空的快递箱——箱子送到了（操作完成），但里面什么都没有（无数据）。
> `!` 就像快递车从来没出发——根本不会有"到达"这个事件。

**三者对比速记**：

```
值的数量：0     1          无穷多
对应类型：!  →  ()  →  i32、String、...
含义    ：不存在  空   有意义的数据
```

---

## 下一步

下一篇讲 Unit 结构体（`struct Marker;`）和类型状态机——`()` 的进阶版，用于在类型级别编码状态。

- 继续阅读：[8. Unit 结构体与类型状态机](./8-Unit结构体.md)
- 回到目录：[第 13 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Reference - Unit Struct](https://doc.rust-lang.org/reference/types/tuple.html#unit-type)
