//! 变量：绑定、不可变、`mut`、遮蔽（shadowing）、类型推断。
//!
//! 在 Rust 里 "变量" 这个词其实有点歧义，准确说法叫 **binding（绑定）**：
//! 把一个名字绑在一个值上。默认是不可变的——这和很多默认可变的语言刚好相反。
//!
//! 运行后请重点观察 4 件事：
//! 1. 不写 `mut` 的绑定，第二次赋值会编译失败（示例里用注释标出）
//! 2. 写了 `mut` 的绑定，值可以改，但**类型不能变**
//! 3. shadowing（同名 `let` 再绑定）不是"改值"，而是"盖掉旧名字"——因此**类型可以变**
//! 4. 绝大多数时候类型是被**推断**出来的，不需要你手写

/// 1) 默认不可变：这是 Rust 最显眼的语言决定之一。
fn demonstrate_immutable_by_default() {
    let language = "Rust";
    println!("默认不可变: language = {language}");

    // 如果取消下面这行的注释，会得到 E0384：
    // cannot assign twice to immutable variable `language`
    // help: consider making this binding mutable: `mut language`
    //
    // language = "Go";
}

/// 2) `mut` 允许改值，但不允许改类型。
fn demonstrate_mut_binding() {
    let mut score = 60;
    println!("mut 改值之前: score = {score}");
    score = 80;
    println!("mut 改值之后: score = {score}");

    // 下面这行会报 E0308（类型不匹配）：
    // `mut` 只是放开"可变性"，不是放开"类型"。
    //
    // score = "high";
}

/// 3) Shadowing：同名 `let` 创建了一个新绑定，旧绑定在同一作用域内被遮蔽。
///
/// 这里要对比两件事：
/// - 遮蔽可以改变类型（例如从 `&str` 变 `usize`）
/// - 遮蔽不需要 `mut`
fn demonstrate_shadowing() {
    let spaces = "   ";
    // 第二次 `let` 创建了新绑定，盖掉了旧的 `spaces`。
    // 类型从 `&str` 变成 `usize`——`mut` 做不到这一点。
    let spaces = spaces.len();
    println!("shadowing 后 spaces = {spaces} (类型从 &str 变成了 usize)");

    // 再来一次，把 usize 变成 String。
    let spaces = format!("len-is-{spaces}");
    println!("又一次 shadowing: spaces = {spaces} (类型又变成了 String)");
}

/// 4) 类型推断：大多数时候不用手写类型。
///
/// 但有两种场景必须写：
/// - 无上下文能推断时（比如 `"42".parse()` 不知道要 parse 成 i32 还是 u64）
/// - 想让类型更窄/更宽时（比如默认会推成 `i32`，但你想要 `u8`）
fn demonstrate_type_inference_and_annotation() {
    // 推断出来是 i32（整型默认）。
    let age = 30;
    // 推断出来是 f64（浮点默认）。
    let pi = 3.14;

    // 显式写类型：把默认 i32 换成 u8。
    let byte: u8 = 255;

    // 有时必须显式写，否则编译器不知道 parse 成哪种数字。
    // 这里用 `let x: i32 = "42".parse().unwrap();` 的变体做演示。
    let parsed: i32 = "42".parse().unwrap_or(0);

    println!("age = {age} (i32), pi = {pi} (f64), byte = {byte} (u8), parsed = {parsed} (i32)");
}

/// 5) 解构赋值：`let (a, b) = pair;` 一次性绑定多个名字。
fn demonstrate_destructuring_binding() {
    let pair = (1, 2);
    let (first, second) = pair;
    println!("解构元组: first = {first}, second = {second}");

    let array = [10, 20, 30];
    let [a, b, c] = array;
    println!("解构数组: a = {a}, b = {b}, c = {c}");
}

pub fn run() {
    demonstrate_immutable_by_default();
    println!();

    demonstrate_mut_binding();
    println!();

    demonstrate_shadowing();
    println!();

    demonstrate_type_inference_and_annotation();
    println!();

    demonstrate_destructuring_binding();
    println!();
}
