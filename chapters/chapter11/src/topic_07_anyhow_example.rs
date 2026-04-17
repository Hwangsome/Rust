// 这个文件演示 anyhow 更适合应用层：我只想把失败向上传，并补充上下文，不想手写很多错误枚举。
// 运行时要观察：`context` / `with_context` 能把“在哪一步失败”补进错误链里。
// 当调用方只需要打印、记录和向上返回错误时，这种写法非常省心。
use anyhow::{Context, Result};
use std::{fs, path::Path};

fn read_and_parse_number(file_path: &Path) -> Result<i32> {
    let contents = fs::read_to_string(file_path)
        .with_context(|| format!("failed to read file: {}", file_path.display()))?;

    let number = contents
        .trim()
        .parse::<i32>()
        .with_context(|| format!("failed to parse integer from: {}", contents.trim()))?;

    Ok(number)
}

pub fn run() {
    println!("== anyhow ==");

    let file_path = std::env::temp_dir().join("rust-learning-anyhow-demo.txt");
    fs::write(&file_path, "42\n").expect("temp file should be writable for anyhow demo");

    match read_and_parse_number(&file_path) {
        Ok(number) => println!("anyhow success => {}", number),
        Err(error) => println!("anyhow error => {:?}", error),
    }

    let _ = fs::remove_file(&file_path);
    println!();
}
