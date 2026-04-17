// 这个文件演示 thiserror 更适合“我要保留结构化错误类型”的场景。
// 运行时要观察：错误仍然是一个明确的 enum，所以调用方可以按变体做分支处理。
// 这在库代码、领域层和需要精细恢复策略的代码里很重要。
use std::{fs, io, num::ParseIntError, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseIntError),
}

fn read_and_parse_number(file_path: &Path) -> Result<i32, AppError> {
    let contents = fs::read_to_string(file_path)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

pub fn run() {
    println!("== thiserror ==");

    let file_path = std::env::temp_dir().join("rust-learning-thiserror-demo.txt");
    fs::write(&file_path, "not-a-number\n")
        .expect("temp file should be writable for thiserror demo");

    match read_and_parse_number(&file_path) {
        Ok(number) => println!("thiserror success => {}", number),
        Err(error) => {
            println!("typed error => {}", error);
            match error {
                AppError::Io(_) => println!("caller can handle I/O separately"),
                AppError::Parse(_) => println!("caller can handle parse failure separately"),
            }
        }
    }

    let _ = fs::remove_file(&file_path);
    println!();
}
