//! Rust 的运算符：算术、比较、逻辑、位运算、赋值、范围、解引用/取引用。
//!
//! 本节把**每类运算符**都走一遍最小示例。重点不是"记住每个符号"，
//! 而是建立几件**容易踩坑**的认知：
//!
//! - 整数除法 `7 / 2` 结果是 `3`（向零截断），不是 `3.5`
//! - 整数求余 `%` 的符号跟随被除数：`-7 % 3 == -1`
//! - `&&` / `||` 是**短路求值**
//! - 位移 `<<` / `>>` 不是乘除 —— 它们直接动二进制位
//! - `&x` 是取引用，`*r` 是解引用 —— 这组符号在后面借用/智能指针章节反复用
//! - `..` / `..=` 是**范围运算符**，既是语法，也是类型（后面迭代器会看到）

/// 1) 算术运算：整数除法会截断。
fn demonstrate_arithmetic() {
    println!("7 + 2 = {}", 7 + 2);
    println!("7 - 2 = {}", 7 - 2);
    println!("7 * 2 = {}", 7 * 2);
    println!("7 / 2 = {} (整数除法截断)", 7 / 2);
    println!("7.0 / 2.0 = {} (浮点除法)", 7.0 / 2.0);
    println!("17 % 5 = {} (求余)", 17 % 5);
    println!("-7 % 3 = {} (符号跟随被除数)", -7 % 3);
}

/// 2) 比较运算：结果都是 `bool`。
fn demonstrate_comparison() {
    let a = 10;
    let b = 20;
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a <  b: {}", a < b);
    println!("a <= b: {}", a <= b);
    println!("a >  b: {}", a > b);
    println!("a >= b: {}", a >= b);
}

/// 3) 逻辑运算：`&&` / `||` 是短路求值；`!` 是取反。
fn demonstrate_logical() {
    let truthy = true;
    let falsy = false;
    println!("true && false = {}", truthy && falsy);
    println!("true || false = {}", truthy || falsy);
    println!("!true          = {}", !truthy);

    // 短路：左边是 `false`，右边的表达式**根本不会求值**。
    let never_called = || {
        println!("  (这条副作用不应该打印)");
        true
    };
    let _ = false && never_called();
    println!("短路求值: && 左边为 false 时，右边不会被调用");
}

/// 4) 位运算：直接操作二进制位。
fn demonstrate_bitwise() {
    let a: u8 = 0b1100_0011;
    let b: u8 = 0b1010_1010;

    println!("a       = 0b{a:08b} ({a})");
    println!("b       = 0b{b:08b} ({b})");
    println!("a & b   = 0b{:08b} ({})", a & b, a & b);
    println!("a | b   = 0b{:08b} ({})", a | b, a | b);
    println!("a ^ b   = 0b{:08b} ({})", a ^ b, a ^ b);
    println!("!a      = 0b{:08b} ({})", !a, !a);
    println!("a << 1  = 0b{:08b} ({})", a << 1, a << 1);
    println!("a >> 2  = 0b{:08b} ({})", a >> 2, a >> 2);
}

/// 5) 复合赋值：`+=` / `-=` / `*=` / `/=` / `%=` / `&=` / `|=` / `^=` / `<<=` / `>>=`。
fn demonstrate_compound_assignment() {
    let mut x = 5;
    x += 5; // x = 10
    x *= 2; // x = 20
    x -= 3; // x = 17
    println!("after +=5, *=2, -=3: x = {x}");
}

/// 6) 范围运算符：`..` 左闭右开；`..=` 全闭。它们既是语法，也是 `Range` / `RangeInclusive` 类型。
fn demonstrate_range_operators() {
    let a: std::ops::Range<i32> = 0..5;
    let b: std::ops::RangeInclusive<i32> = 0..=5;
    println!("0..5   (类型 Range)           包含: {:?}", a.collect::<Vec<_>>());
    println!("0..=5  (类型 RangeInclusive)  包含: {:?}", b.collect::<Vec<_>>());
}

/// 7) 引用与解引用：`&x` 取引用，`*r` 解引用。
fn demonstrate_reference_and_deref() {
    let value = 42;
    let reference = &value; // &i32
    println!("reference      = {reference}");
    println!("*reference     = {}", *reference);
    println!("*reference + 1 = {}", *reference + 1);
}

pub fn run() {
    println!("-- (1) 算术运算 --");
    demonstrate_arithmetic();
    println!();

    println!("-- (2) 比较运算 --");
    demonstrate_comparison();
    println!();

    println!("-- (3) 逻辑运算 --");
    demonstrate_logical();
    println!();

    println!("-- (4) 位运算 --");
    demonstrate_bitwise();
    println!();

    println!("-- (5) 复合赋值 --");
    demonstrate_compound_assignment();
    println!();

    println!("-- (6) 范围运算符 --");
    demonstrate_range_operators();
    println!();

    println!("-- (7) 引用与解引用 --");
    demonstrate_reference_and_deref();
    println!();
}
