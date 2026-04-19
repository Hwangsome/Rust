# 13. Rust 中的运算符

> - **所属章节**：第 1 章 · Quick Startup
> - **Cargo package**：`chapter01`
> - **运行方式**：`cargo run -p chapter01`
> - **代码位置**：`chapters/chapter01/src/topic_12_operators_in_rust.rs`
> - **上一篇**：[12. 错误信息与错误码](./12-错误信息与错误码.md)
> - **下一篇**：[14. 结合性与运算符重载](./14-结合性与运算符重载.md)
> - **关键词**：算术运算符、比较运算符、逻辑运算符、位运算、复合赋值、范围运算符

---

## 这一节解决什么问题

Rust 的运算符大多和 C/Java 一样，但有几个重要的区别：

1. **整数除法**截断（`7 / 2 == 3`，不是 `3.5`）
2. **不支持隐式类型提升**（`i32 + i64` 会报错）
3. 有独特的**范围运算符**（`0..5`、`0..=5`）
4. **`%` 取余的符号**跟随被除数（`-7 % 3 == -1`，不是 `2`）

---

## 完整速查表

### 算术运算符

```rust
let a = 10_i32;
let b = 3_i32;

println!("加: {}", a + b);  // 13
println!("减: {}", a - b);  // 7
println!("乘: {}", a * b);  // 30
println!("除: {}", a / b);  // 3（整数截断！）
println!("余: {}", a % b);  // 1

// 浮点除法
let x = 10.0_f64;
let y = 3.0_f64;
println!("浮点除: {}", x / y);  // 3.333...

// ⚠️ 取余的符号跟随被除数
println!("-7 % 3 = {}", -7_i32 % 3);  // -1（不是 2！）
println!("7 % -3 = {}", 7_i32 % -3);  // 1
```

### 比较运算符（都返回 bool）

```rust
println!("== : {}", 5 == 5);  // true
println!("!= : {}", 5 != 3);  // true
println!("<  : {}", 3 < 5);   // true
println!(">  : {}", 5 > 3);   // true
println!("<= : {}", 5 <= 5);  // true
println!(">= : {}", 3 >= 3);  // true
```

### 逻辑运算符

```rust
println!("&&: {}", true && false);  // false（短路求值）
println!("||: {}", false || true);  // true（短路求值）
println!("! : {}", !true);          // false

// 短路求值：左边确定结果时，右边不执行
fn has_side_effect() -> bool {
    println!("side effect!");
    true
}
let _ = false && has_side_effect();  // "side effect!" 不会打印
let _ = true  || has_side_effect();  // "side effect!" 不会打印
```

### 位运算

```rust
let a: u8 = 0b1010_1010;  // 170
let b: u8 = 0b1100_1100;  // 204

println!("AND:  {:08b} = {}", a & b, a & b);   // 10001000 = 136
println!("OR:   {:08b} = {}", a | b, a | b);   // 11101110 = 238
println!("XOR:  {:08b} = {}", a ^ b, a ^ b);   // 01100110 = 102
println!("NOT:  {:08b} = {}", !a, !a);          // 01010101 = 85
println!("<<2:  {:08b} = {}", a << 2, a << 2); // 10101000 = 168（左移2位）
println!(">>1:  {:08b} = {}", a >> 1, a >> 1); // 01010101 = 85（右移1位）
```

### 复合赋值运算符

```rust
let mut x = 10;
x += 5;   println!("+=5: {x}");   // 15
x -= 3;   println!("-=3: {x}");   // 12
x *= 2;   println!("*=2: {x}");   // 24
x /= 4;   println!("/=4: {x}");   // 6
x %= 4;   println!("%=4: {x}");   // 2
x <<= 1;  println!("<<=1: {x}");  // 4
x >>= 1;  println!(">>=1: {x}");  // 2
```

### 范围运算符（Rust 独有）

```rust
// 半开区间（不含结尾）
let r1 = 0..5;       // 0, 1, 2, 3, 4
let r2 = 'a'..'f';   // a, b, c, d, e

// 闭区间（含结尾）
let r3 = 0..=5;      // 0, 1, 2, 3, 4, 5

// 使用范围
let sum: i32 = (1..=100).sum();  // 5050（高斯求和）
println!("1到100的和: {sum}");

for c in 'a'..='e' {
    print!("{c} ");  // a b c d e
}
println!();
```

---

## 完整运行示例

```rust
fn main() {
    println!("=== 整数除法截断 ===");
    println!("7 / 2 = {}   (整数截断，不是 3.5)", 7_i32 / 2);
    println!("-7 / 2 = {}  (向零截断)", -7_i32 / 2);
    println!("7.0 / 2.0 = {} (浮点)", 7.0_f64 / 2.0);
    println!();

    println!("=== 短路求值 ===");
    let mut count = 0;
    let always_true = || { count += 1; true };
    let _ = false && always_true(); // always_true 不会执行
    println!("false && f(): 调用了 {} 次 always_true", count); // 0

    count = 0;
    let _ = true || always_true(); // always_true 不会执行
    println!("true  || f(): 调用了 {} 次 always_true", count); // 0
    println!();

    println!("=== 位运算实际应用 ===");
    // 权限系统：用 bit 标记权限
    const READ: u8 = 0b001;
    const WRITE: u8 = 0b010;
    const EXEC: u8 = 0b100;

    let user_perms = READ | WRITE;  // 0b011
    println!("用户权限: {:03b}", user_perms);
    println!("有读权限: {}", user_perms & READ != 0);
    println!("有写权限: {}", user_perms & WRITE != 0);
    println!("有执行权限: {}", user_perms & EXEC != 0);
    println!();

    println!("=== 范围运算符 ===");
    let sum: i32 = (1..=100).sum();
    println!("高斯求和 1+2+...+100 = {sum}");

    let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
    println!("0-19 的偶数: {evens:?}");

    let alphabet: String = ('a'..='z').collect();
    println!("字母表: {alphabet}");
}
```

---

## 注意点与陷阱

### 陷阱 1：整数和浮点不能直接混合运算

```rust
let x: i32 = 5;
let y: f64 = 2.5;
// let z = x + y; // ❌ 类型不匹配
let z = x as f64 + y; // ✅ 显式转换
```

### 陷阱 2：取余的符号

```rust
// Rust（和 C/Java）：符号跟随被除数
println!("{}", -7 % 3);  // -1（不是 2！）
// 数学上的模运算（总是非负）：
fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}
println!("{}", modulo(-7, 3));  // 2
```

---

## 下一步

- 继续阅读：[14. 结合性与运算符重载](./14-结合性与运算符重载.md)
- 回到目录：[第 1 章：Quick Startup](./README.md)
