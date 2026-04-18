//! 原生（primitive）类型：整数、浮点、布尔、字符。
//!
//! Rust 的原生类型组成一张对比强烈的表：
//! - **整数**：`i8 i16 i32 i64 i128 isize` 和它们的无符号版本 `u8 ... usize`
//! - **浮点**：`f32 f64`
//! - **布尔**：`bool`（只占 1 字节，值只能是 `true`/`false`，不允许 `0/1` 当布尔用）
//! - **字符**：`char` 是一个 **Unicode 标量值**，占 4 字节，不等同于 C 里的 `char`
//!
//! 运行后建议重点观察：
//! 1. 字面量默认类型推断（整型默认 `i32`，浮点默认 `f64`）
//! 2. 数字后缀写法（`2.0_f32`、`1u8`、`1_000_000i64`）
//! 3. 各种类型的 `MIN` / `MAX` 边界
//! 4. 整数溢出在 debug 与 release 下行为不同（这里只展示"用 wrapping_add 显式处理"）
//! 5. `char` 的真实宽度

/// 1) 默认推断 + 常用显式标注。
fn demonstrate_integer_defaults_and_annotations() {
    let default_int = 32; // 默认 i32
    let small: i8 = -8;
    let big: i64 = 9_000_000_000;
    let byte: u8 = 255;
    let word: u16 = 65_535;

    println!(
        "默认 i32: {default_int}, i8: {small}, i64: {big}, u8: {byte}, u16: {word}"
    );

    // 后缀写法：数字直接贴上类型名，可读性比冒号标注更好。
    let literal_i64 = 1_000_000i64;
    let literal_u8 = 200u8;
    println!("后缀写法: literal_i64 = {literal_i64}, literal_u8 = {literal_u8}");
}

/// 2) 浮点数：默认 `f64`，精度更高。
fn demonstrate_float_types() {
    let default_float = 3.14; // 默认 f64
    let single: f32 = 3.14_f32;

    // 浮点数运算结果也是浮点，记住除法在整数/浮点下行为不同。
    let int_div = 7 / 2; // 整数除法，结果是 3
    let float_div = 7.0 / 2.0; // 浮点除法，结果是 3.5

    println!("default_float = {default_float} (f64), single = {single} (f32)");
    println!("整数除法 7 / 2 = {int_div}，浮点除法 7.0 / 2.0 = {float_div}");
}

/// 3) 整数边界：每种类型都有固定范围。
///
/// `i8` 的范围是 -128..=127；`u8` 的范围是 0..=255。
/// 知道这一点，后面再看溢出才能不迷糊。
fn demonstrate_integer_bounds() {
    println!("i8  范围: [{}, {}]", i8::MIN, i8::MAX);
    println!("u8  范围: [{}, {}]", u8::MIN, u8::MAX);
    println!("i32 范围: [{}, {}]", i32::MIN, i32::MAX);
    println!("u32 范围: [{}, {}]", u32::MIN, u32::MAX);
}

/// 4) 整数溢出：Rust 把这件事放在明面上。
///
/// - **Debug 构建**：溢出会 panic，让 bug 当场暴露
/// - **Release 构建**：按"二进制补码回绕"处理（不会 panic，但结果可能出乎意料）
/// - **显式选择**：用 `wrapping_*` / `checked_*` / `saturating_*` / `overflowing_*`
///   明确告诉读者你要什么行为
fn demonstrate_overflow_explicitly() {
    let near_max: u8 = 250;

    // wrapping_add: 回绕（wrap），250 + 10 = 260 超了 u8 最大值 255，回绕到 4。
    let wrapped = near_max.wrapping_add(10);

    // checked_add: 溢出时返回 None，不溢出时返回 Some(结果)。
    let checked = near_max.checked_add(10);

    // saturating_add: 饱和，到达边界后就停在 MAX。
    let saturated = near_max.saturating_add(10);

    println!(
        "250 + 10 (u8): wrapping = {wrapped}, checked = {checked:?}, saturating = {saturated}"
    );
}

/// 5) `bool`：只能是 `true` / `false`，不能当整数。
fn demonstrate_bool_is_not_integer() {
    let is_rust_fun = true;
    let is_tired: bool = false;

    println!("is_rust_fun = {is_rust_fun}, is_tired = {is_tired}");

    // 在 C / Python 里 0 当 false 的那套在 Rust 里不成立：
    // if 0 { ... } 会报 E0308，类型必须显式是 bool。
    //
    // if 0 { println!("never compiles"); }
}

/// 6) `char`：一个 **Unicode 标量值**，4 字节。
///
/// 这和 C 的 `char`（1 字节）、Java 的 `char`（2 字节）都不同。
/// 所以一个 `char` 可以放 `'中'`、`'🦀'`、`'A'`。
fn demonstrate_char_is_unicode_scalar() {
    let ascii: char = 'A';
    let chinese: char = '中';
    let emoji: char = '🦀';

    println!(
        "char 都占 {} 字节 —— ascii = {ascii}, chinese = {chinese}, emoji = {emoji}",
        std::mem::size_of::<char>()
    );
}

/// 7) `as` 显式数值转换：Rust 不做隐式转换。
///
/// 这和 C 的默认 promote 语义截然不同：`i64 + i32` 在 Rust 里不会自动通融，你得显式转。
fn demonstrate_explicit_cast_with_as() {
    let int_value: i32 = -42;
    let as_float = int_value as f64;

    // as 做窄转时会**截断**，不是四舍五入——这是常见坑。
    let rounded = 3.7_f64 as i32;

    println!("int -> float: {int_value} as f64 = {as_float}");
    println!("float -> int 会截断: 3.7 as i32 = {rounded} (不是 4!)");
}

pub fn run() {
    demonstrate_integer_defaults_and_annotations();
    println!();

    demonstrate_float_types();
    println!();

    demonstrate_integer_bounds();
    println!();

    demonstrate_overflow_explicitly();
    println!();

    demonstrate_bool_is_not_integer();
    println!();

    demonstrate_char_is_unicode_scalar();
    println!();

    demonstrate_explicit_cast_with_as();
    println!();
}
