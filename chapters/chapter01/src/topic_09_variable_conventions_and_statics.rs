//! 变量命名约定 & `const` / `static` 的差异。
//!
//! 本节涵盖：
//! - Rust 命名约定（`snake_case`、`SCREAMING_SNAKE_CASE`、`UpperCamelCase`）
//! - 数字字面量里的 `_` 分隔符（只是给人看，不影响值）
//! - `let` vs `const` vs `static` 的**三条关键差异**
//! - `_` 前缀告诉编译器"这是故意不用的变量"
//!
//! 运行后请重点留意：`PI` 和 `WELCOME` 都是在模块顶层定义的——
//! 这是因为 `const` / `static` 可以放在**任何 item 位置**，不一定要进函数体。

// ---------- 模块顶层的常量与静态量 ----------

/// `const`：**编译期常量**。
/// - 必须显式写类型
/// - 可以直接放在模块顶层，也可以放在函数内
/// - 没有固定内存地址，使用时**内联**进每个使用点
/// - 适合"纯值"（数字、字符串字面量、被编译期能算出的表达式）
const PI: f32 = 3.14159;
const MAX_RETRIES: u32 = 5;

/// `static`：**程序生命周期内存在的静态变量**。
/// - 有固定内存地址
/// - 可以是 `'static` 的引用（比如 `&str`）
/// - 一般用于"需要被引用/共享的全局数据"
/// - 可变的 `static mut` 需要 `unsafe`，**不要在入门阶段使用**
static WELCOME: &str = "Welcome to Rust";

/// 命名约定：
/// - 变量 / 函数 / 模块：`snake_case`
/// - 常量 / 静态量：`SCREAMING_SNAKE_CASE`
/// - 类型 / trait / enum variant：`UpperCamelCase`
fn demonstrate_naming_conventions() {
    // 变量名应当描述**是什么**，而不是**类型是什么**。
    let player_name = "Alice";
    let player_score: u32 = 42;

    // 前导下划线：明确告诉编译器"我知道我没用它，别警告"。
    let _unused_profile_id = 100;

    // 数字里的 `_` 是可读性分隔符，不影响值。
    let large_value = 1_000_000;
    let readable_hex = 0xFF_FF_FF;
    let readable_bin = 0b_1010_1010;

    println!("player_name = {player_name}, player_score = {player_score}");
    println!("large_value = {large_value}, readable_hex = 0x{readable_hex:X}, bin = 0b{readable_bin:b}");
}

/// `const` 的使用：更像"常量表达式"，在编译期替换。
fn demonstrate_const_usage() {
    let radius: f32 = 2.0;

    // PI 会在这里被内联使用——编译后相当于直接写了 3.14159。
    let area = PI * radius * radius;
    println!("circle area ≈ {area}");

    println!("最大重试次数 MAX_RETRIES = {MAX_RETRIES}");

    // 函数内也可以声明 const，作用域局限于函数内
    const DEFAULT_TIMEOUT_SECS: u64 = 30;
    println!("DEFAULT_TIMEOUT_SECS = {DEFAULT_TIMEOUT_SECS}");
}

/// `static` 的使用：有固定内存地址，适合被多处**引用**。
fn demonstrate_static_usage() {
    println!("WELCOME = {WELCOME}");

    // 可以取它的地址——因为 `static` 有固定存储位置（`const` 不行，`const` 没有地址）。
    let addr = std::ptr::addr_of!(WELCOME);
    println!("WELCOME 的地址: {addr:p}");
}

/// `const` vs `static` vs `let` 简明对照表。
fn demonstrate_difference_summary() {
    println!("三者差异速记:");
    println!("  let   —— 运行期绑定；默认不可变；局部作用域；可被 drop");
    println!("  const —— 编译期常量；必须显式写类型；使用时内联；不占固定地址");
    println!("  static—— 程序生命周期内存在的全局值；有固定地址；可被借用");
}

pub fn run() {
    println!("-- (1) 命名约定 --");
    demonstrate_naming_conventions();
    println!();

    println!("-- (2) const 使用 --");
    demonstrate_const_usage();
    println!();

    println!("-- (3) static 使用 --");
    demonstrate_static_usage();
    println!();

    println!("-- (4) let / const / static 差异速记 --");
    demonstrate_difference_summary();
    println!();
}
