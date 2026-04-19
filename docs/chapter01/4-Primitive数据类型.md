# 4. Primitive 数据类型

> - **所属章节**：第 1 章 · Quick Startup
> - **Cargo package**：`chapter01`
> - **运行方式**：`cargo run -p chapter01`
> - **代码位置**：`chapters/chapter01/src/topic_03_primitive_data_types.rs`
> - **上一篇**：[3. 变量](./3-变量.md)
> - **下一篇**：[5. Compound 数据类型](./5-Compound数据类型.md)
> - **关键词**：`i32`、`u8`、`f64`、`bool`、`char`、类型推断、溢出、`as` 转换

---

## 这一节解决什么问题

Rust 是**静态强类型**语言——每个值都有一个在编译期已知的类型，不同类型之间不会隐式转换。

从 Python/JavaScript 来的开发者会发现：

```rust
let x: i32 = 5;
let y: i64 = 10;
// let z = x + y; // ❌ 不能直接相加！类型不同
let z = x as i64 + y;  // ✅ 显式转换
```

这节讲清楚 Rust 的基础标量类型：整数、浮点、布尔、字符。

---

## 一分钟结论

- **整数**：`i8/16/32/64/128`（有符号）、`u8/16/32/64/128`（无符号）、`isize/usize`（平台相关）
- **浮点**：`f32`（单精度）、`f64`（双精度，默认）
- **布尔**：`bool`，只有 `true`/`false`（不是 0/1！）
- **字符**：`char`，Unicode 标量值，4 字节（不是 ASCII 的 1 字节）
- 整数**默认**是 `i32`；浮点**默认**是 `f64`
- 类型不匹配不会隐式转换，必须用 `as` 显式转换
- Debug 模式下整数溢出会 panic；Release 模式下回绕

---

## 与其他语言对比

| 类型 | Java | Python | C++ | Rust |
|-----|------|--------|-----|------|
| 整数溢出 | 静默回绕 | 不溢出（大整数）| 未定义行为 | Debug panic / Release 回绕 |
| 隐式类型转换 | 有（int→long）| 有 | 有（危险）| **无**（必须显式 `as`）|
| char | 2字节（UTF-16）| 1字节 Unicode 代码点 | 1字节（ASCII）| **4字节**（Unicode 标量）|
| 整数类型 | `int`（32位）等 | 动态大小 | `int`（平台相关）| **明确指定位数** |

---

## 详细原理

### 1. 整数类型

```
有符号（i）：可以是负数
  i8:    -128 ~ 127
  i16:   -32768 ~ 32767
  i32:   -2^31 ~ 2^31-1  ← 最常用，整数默认
  i64:   -2^63 ~ 2^63-1
  i128:  -2^127 ~ 2^127-1
  isize: 平台相关（32位系统=i32，64位=i64）

无符号（u）：只有非负数
  u8:    0 ~ 255      ← 常用于字节操作
  u16:   0 ~ 65535
  u32:   0 ~ 2^32-1
  u64:   0 ~ 2^64-1
  usize: 平台相关，常用于数组下标和长度
```

```rust
let a: i8 = 127;         // i8 最大值
let b: u8 = 255;         // u8 最大值
let c: i32 = -2_147_483_648; // i32 最小值（下划线只是可读性）
let d: u64 = 18_446_744_073_709_551_615; // u64 最大值

// 字面量后缀
let e = 42i32;   // 明确指定类型
let f = 0xFF_u8; // 十六进制 255，类型 u8
let g = 0b1111_0000_u8; // 二进制，类型 u8
```

### 2. 浮点类型

```rust
let x = 3.14;        // 默认 f64
let y: f32 = 3.14;  // 单精度，精度较低
let z: f64 = 3.14;  // 双精度，推荐

// 特殊值
let inf = f64::INFINITY;
let neg_inf = f64::NEG_INFINITY;
let nan = f64::NAN;

println!("{}", nan == nan);  // false（NaN 不等于自身！）
println!("{}", nan.is_nan()); // true
```

### 3. `bool`：严格的真假

```rust
let t: bool = true;
let f: bool = false;

// ⚠️ 不能把整数当 bool 用
// if 1 { } // ❌ 必须是 bool

// 比较运算返回 bool
let x = 5;
let is_big = x > 3;    // bool
let is_ten = x == 10;  // bool

println!("{t} && {f} = {}", t && f);  // false
println!("{t} || {f} = {}", t || f);  // true
println!("!{t} = {}", !t);            // false
```

### 4. `char`：Unicode 标量值（4字节！）

```rust
let a = 'a';       // ASCII，但存为 Unicode
let emoji = '🦀';  // 完全合法！Rust char 支持 Unicode
let chinese = '中'; // 汉字
let newline = '\n'; // 转义字符

println!("size of char: {}", std::mem::size_of::<char>());  // 4

// char 不等于 u8（ASCII 字节）！
let byte: u8 = b'a';   // b'a' 是字节字面量，97
let ch: char = 'a';    // char 'a'，也是 97，但是 4 字节存储
```

### 5. 类型转换（必须显式）

```rust
let x: i32 = 42;
let y: f64 = x as f64;  // i32 → f64
let z: i32 = 3.99_f64 as i32;  // f64 → i32：截断（不是四舍五入！）

println!("3.99 as i32 = {z}");  // 3，不是 4！

// 窄化转换会截断
let big: i32 = 256;
let small: u8 = big as u8;  // 256 mod 256 = 0（溢出截断）
println!("256 as u8 = {small}");  // 0
```

### 6. 整数溢出

```rust
// u8 的范围是 0-255
let max: u8 = u8::MAX;  // 255

// Debug 模式（cargo build 或 cargo run）：溢出会 panic
// let overflow = max + 1; // thread 'main' panicked: arithmetic operation overflowed

// 显式处理溢出的方法
let wrapping = max.wrapping_add(1);   // 256 → 0（回绕）
let checked = max.checked_add(1);     // None（溢出）
let saturated = max.saturating_add(1); // 255（饱和，不超过最大值）
let overflowing = max.overflowing_add(1); // (0, true)（返回结果和是否溢出）

println!("wrapping_add(1) = {wrapping}");    // 0
println!("checked_add(1) = {checked:?}");    // None
println!("saturating_add(1) = {saturated}"); // 255
println!("overflowing_add(1) = {overflowing:?}"); // (0, true)
```

---

## 完整运行示例

```rust
use std::mem::size_of;

fn main() {
    println!("=== 整数类型大小 ===");
    println!("i8:    {} bytes, 范围 [{}, {}]", size_of::<i8>(), i8::MIN, i8::MAX);
    println!("u8:    {} bytes, 范围 [{}, {}]", size_of::<u8>(), u8::MIN, u8::MAX);
    println!("i32:   {} bytes, 范围 [{}, {}]", size_of::<i32>(), i32::MIN, i32::MAX);
    println!("u64:   {} bytes, 范围 [0, {}]", size_of::<u64>(), u64::MAX);
    println!("usize: {} bytes (平台相关)", size_of::<usize>());
    println!();

    println!("=== 浮点 ===");
    let pi_f32: f32 = std::f32::consts::PI;
    let pi_f64: f64 = std::f64::consts::PI;
    println!("f32 π = {pi_f32:.10}");  // 精度较低
    println!("f64 π = {pi_f64:.10}");  // 精度更高
    println!();

    println!("=== bool ===");
    let nums = [1, 2, 3, 4, 5];
    let all_positive = nums.iter().all(|&n| n > 0);
    let has_five = nums.iter().any(|&n| n == 5);
    println!("全部正数: {all_positive}");
    println!("包含5: {has_five}");
    println!();

    println!("=== char（Unicode）===");
    let chars = ['A', '中', '🦀', '\u{1F600}'];  // U+1F600 = 😀
    for c in chars {
        println!("  '{}' (U+{:04X}, {} bytes)", c, c as u32, c.len_utf8());
    }
    println!();

    println!("=== 类型转换 ===");
    let i: i32 = 300;
    let f: f64 = i as f64;
    let u: u8 = i as u8;  // 截断！300 % 256 = 44
    println!("i32 {} as f64 = {f}", i);
    println!("i32 {} as u8 = {} (截断)", i, u);

    let pi = 3.99_f64;
    let truncated = pi as i32;
    println!("f64 {pi} as i32 = {truncated} (截断，不是四舍五入)");
    println!();

    println!("=== 溢出处理 ===");
    let max: u8 = 250;
    println!("saturating_add(10) = {}", max.saturating_add(10)); // 255（不超255）
    println!("wrapping_add(10) = {}", max.wrapping_add(10));      // 4（回绕）
    println!("checked_add(10) = {:?}", max.checked_add(10));      // None
}
```

---

## 实际工程场景

### 何时选择哪种整数类型

```
u8：字节操作、颜色值(0-255)、ASCII码
u16：端口号(0-65535)、某些协议字段
i32：通用整数（默认）、小到中等计数
u64/i64：大数计数、文件大小、时间戳（毫秒）
usize：数组下标、Vec 长度、内存偏移
```

---

## 注意点与陷阱

### 陷阱 1：`as` 截断不是四舍五入

```rust
let x = 3.9_f64;
let y = x as i32;  // 3（截断），不是 4（四舍五入）
let rounded = x.round() as i32;  // 4（先四舍五入再转）
```

### 陷阱 2：`char` 长度与 `str.len()` 的区别

```rust
let s = "hello中文";
println!("s.len() = {}", s.len());            // 11（字节数）
println!("chars = {}", s.chars().count());   // 7（字符数）
```

---

## 下一步

- 继续阅读：[5. Compound 数据类型](./5-Compound数据类型.md)
- 回到目录：[第 1 章：Quick Startup](./README.md)
