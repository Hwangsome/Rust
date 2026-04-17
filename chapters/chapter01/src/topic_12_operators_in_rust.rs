// 这一节把最常见的运算符放在一起，帮助初学者建立“不同运算符解决不同问题”的印象。
pub fn run() {

    // `%` 取余，常用来判断整除或周期性规律。
    println!("17 % 5 = {}", 17 % 5);

    let a = 10;
    let b = 10;

    // 这里把常见比较运算一次性打印出来，方便观察返回值都是 `bool`。
    println!(
        "== {} != {} < {} > {} <= {} >= {}",
        a == b,
        a != b,
        a < b,
        a > b,
        a <= b,
        a >= b
    );

    let left = 10;
    let right = 20;

    // 逻辑运算通常作用于布尔表达式。
    println!("logical && result = {}", (left > 5) && (right < 25));

    let mut x = 5;

    // 复合赋值运算是“先计算，再写回原变量”的简写。
    x += 5;
    x *= 2;
    println!("assignment operators result = {x}");

    let bits: u8 = 4;

    // 位运算会直接作用在二进制位上，这里先只保留最小例子。
    println!("bitwise and = {}", bits & bits);
    println!("left shift = {}", bits << 1);
    println!();
}
