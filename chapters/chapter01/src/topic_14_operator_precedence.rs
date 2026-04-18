//! 运算符优先级：不同运算符之间，谁先算？
//!
//! 这一节的目标不是背完整优先级表，而是：
//!
//! - 建立"乘除优先于加减"这种最基本的直觉
//! - 记住**位运算比比较运算优先级低**（C/Java 里一样有这个坑）
//! - 知道**逻辑运算 `&&` 优先于 `||`**
//! - 当表达式变复杂时，**不要赌优先级，加括号更明显**
//!
//! 优先级参考完整列表见 Rust Reference 的 "Expression precedence" 章节。

/// 1) 乘除 vs 加减：最经典的一对。
fn demonstrate_mul_div_vs_add_sub() {
    println!("2 + 3 * 4     = {} (先算 3 * 4)", 2 + 3 * 4);
    println!("(2 + 3) * 4   = {} (括号改变分组)", (2 + 3) * 4);
    println!("10 - 6 / 2    = {} (先算 6 / 2)", 10 - 6 / 2);
}

/// 2) 比较 vs 位运算：**位运算优先级更低**，很多人想当然写错。
///
/// 下面的例子如果写成 `x & 1 == 0`，它其实被解释成 `x & (1 == 0)` —— 完全不是你想的意思。
/// 所以涉及位运算做判断时，**一定要加括号**。
fn demonstrate_bitwise_vs_comparison() {
    let x: u32 = 42;

    // 正确写法：把位运算用括号包起来，再拿来比较。
    let is_even_via_bit = (x & 1) == 0;
    println!("(x & 1) == 0   = {is_even_via_bit}  ←（推荐写法）");

    // 反面教材：没有括号，`x & 1 == 0` 会被 Rust 解释成 `x & (1 == 0)`——
    // 但 `x & bool` 类型不匹配，所以这种误写在 Rust 里**会编译失败**（反倒救了你）。
    // 但在 C/Java 里这会编译通过并得到错误结果，所以这个坑值得牢记。
    //
    // let surprise = x & 1 == 0; // 会报 E0277
}

/// 3) 逻辑运算：`&&` 优先于 `||`。
fn demonstrate_logical_precedence() {
    // `true || false && false` 相当于 `true || (false && false)` → `true || false` → `true`
    let value = true || false && false;
    println!("true || false && false = {value} (相当于 true || (false && false))");

    // 加括号让意图更清晰。
    let forced = (true || false) && false;
    println!("(true || false) && false = {forced}");
}

/// 4) 一元负号 vs 算术：`-x * 2` 相当于 `(-x) * 2`。
fn demonstrate_unary_minus() {
    let x = 3;
    println!("-x * 2         = {} (相当于 (-x) * 2)", -x * 2);
    println!("-(x * 2)       = {}", -(x * 2));
}

/// 5) 实战总结：别赌，直接加括号。
fn demonstrate_real_world_advice() {
    let score = 85;
    let bonus = true;

    // 看起来"聪明"的写法——读者每次都得停下来算优先级。
    let eligible = score >= 80 && score < 100 || bonus;

    // 推荐写法：给读者一个清晰的分组。
    let eligible_clear = ((score >= 80) && (score < 100)) || bonus;

    println!("紧凑写法结果 = {eligible}, 括号写法结果 = {eligible_clear}");
    println!("实战建议：表达式超过 3 个运算符，就用括号画出分组。");
}

pub fn run() {
    println!("-- (1) 乘除 vs 加减 --");
    demonstrate_mul_div_vs_add_sub();
    println!();

    println!("-- (2) 比较 vs 位运算 --");
    demonstrate_bitwise_vs_comparison();
    println!();

    println!("-- (3) 逻辑运算优先级 --");
    demonstrate_logical_precedence();
    println!();

    println!("-- (4) 一元负号 --");
    demonstrate_unary_minus();
    println!();

    println!("-- (5) 实战建议: 加括号 --");
    demonstrate_real_world_advice();
    println!();
}
