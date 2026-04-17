// 这一节把最常见的控制流放在一起对比：
// `if` 负责分支，`loop`/`while`/`for` 负责重复执行。
pub fn run() {

    let temperature = 28;

    // `if` 的条件必须是布尔值，Rust 不会把整数自动当成 true/false。
    if temperature >= 30 {
        println!("天气偏热");
    } else if temperature >= 20 {
        println!("天气适中");
    } else {
        println!("天气偏凉");
    }

    // `loop` 是最原始的循环形式，通常要自己决定何时 `break`。
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("loop 在 count = {count} 时退出");
            break;
        }
    }

    // `while` 更适合“条件成立就继续”的循环。
    let mut index = 0;
    while index < 3 {
        println!("while index = {index}");
        index += 1;
    }

    // `for` + range 是入门阶段最清晰的遍历方式之一。
    for number in 1..=3 {
        println!("for number = {number}");
    }
    println!();
}
