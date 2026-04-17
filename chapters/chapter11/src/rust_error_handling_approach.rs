// 这个文件先把 Rust 的错误处理基调讲清楚：不可恢复错误通常用 panic，预期失败用 Result。
// 为了让整章示例能继续跑下去，这里只演示 recoverable error，不主动触发 panic。
// 重点观察：错误不会被隐藏在异常机制里，而是直接出现在类型和 match 分支上。
use std::{fs::File, num::ParseIntError};

fn parse_input_to_int(input: &str) -> Result<i32, ParseIntError> {
    input.trim().parse::<i32>()
}

pub fn run() {
    println!("== Rust Error Handling Approach ==");

    match File::open("missing.txt") {
        Ok(file) => println!("file opened successfully: {:?}", file),
        Err(error) => println!("recoverable file error => {}", error),
    }

    match parse_input_to_int("42a") {
        Ok(number) => println!("parsed number => {}", number),
        Err(error) => println!("recoverable parse error => {}", error),
    }

    println!("panic! / assert! are reserved for unrecoverable situations or violated assumptions");
    println!();
}
