// 这一节补充变量命名约定，以及 `const` / `static` 的最小差异。
// 读者要观察的重点不是“背语法”，而是知道这些约定为什么能让代码更容易读。
static WELCOME: &str = "Welcome to Rust";
const PI: f32 = 3.14;

pub fn run() {

    // 前导下划线告诉编译器：这个变量暂时不用，不要给未使用告警。
    let _unused_number = 45;

    // 数字中加下划线只是为了可读性，不影响数值本身。
    let large_value = 40_000;
    let radius = 2.0_f32;

    println!("snake_case variable = {large_value}");

    // `const` 更像“编译期常量”，这里把它当作普通值使用。
    println!("circle area ≈ {}", PI * radius * radius);

    // `static` 有固定存储位置，通常表示全局共享的固定数据。
    println!("WELCOME = {WELCOME}");
    println!("未使用变量通常以下划线开头，常量和静态量通常用 SCREAMING_SNAKE_CASE。");
    println!();
}
