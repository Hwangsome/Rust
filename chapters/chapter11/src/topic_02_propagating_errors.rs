// 这个文件演示“传播错误”而不是“就地吞掉错误”。
// 运行时要观察：`?` 不是忽略错误，而是把当前函数变成一层干净的传递通道。
// 只要返回类型兼容，调用者就能继续决定怎么处理失败。
use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    let number = input.trim().parse::<i32>()?;
    Ok(number)
}

fn extract_username(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    email.get(..at_pos)
}

pub fn run() {
    println!("== Propagating Errors ==");

    println!("read_number(\"15\") => {:?}", read_number("15"));
    println!(
        "extract_username => {:?}",
        extract_username("alice@example.com")
    );
    println!(
        "extract_username on invalid email => {:?}",
        extract_username("invalid-email")
    );
    println!();
}
