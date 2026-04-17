// 这一节演示最常见的基础标量类型。
// 这里故意把例子写得很直白，让读者先建立“一个值对应一个类型”的感觉，
// 后面再进入更复杂的类型系统主题。
pub fn run() {

    // `i32` 是最常见的有符号整数类型，也是很多整数值的默认推断结果。
    let integer_default = 32;
    let integer_value: i32 = -42;
    let unsigned_value: u8 = 255;
    let float_value: f64 = 3.1415;
    let is_rust_fun: bool = true;

    // `char` 表示单个 Unicode 标量值，不只是 ASCII。
    let chinese_char: char = '中';

    // 使用 `as` 做一个最小的数值类型转换示例。
    // 这里的目标不是推荐到处强转，而是先让读者知道 Rust 的转换通常要显式写出来。
    let type_conversion = integer_value as f64;

    println!("integer_default = {integer_default}");
    println!("integer_value = {integer_value}");
    println!("unsigned_value = {unsigned_value}");
    println!("float_value = {float_value}");
    println!("is_rust_fun = {is_rust_fun}");
    println!("chinese_char = {chinese_char}");
    println!("type_conversion = {type_conversion}");
    println!();
}
