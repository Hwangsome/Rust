//! `thiserror`：**库代码**友好的"结构化错误 enum 生成器"。
//!
//! 核心理念：
//! - 让你**保留精确的错误 enum**（每个 variant 是一种失败模式）
//! - 通过 `#[error("...")]` 派生 `Display`，自动生成给人看的错误消息
//! - 通过 `#[from]` 在 variant 上自动派生 `From<InnerError>`，让 `?` 能自动转换
//!
//! 结果：既保留了按 variant 恢复的能力，又**不用手写**一堆 `impl Display` / `impl From` 样板。
//!
//! 对照 topic_07（anyhow）：
//!
//! | 对比维度      | thiserror          | anyhow                 |
//! |-----------|--------------------|-------------------------|
//! | 定位         | 精确错误 enum         | 万能错误容器            |
//! | 调用方可否按 variant match | ✅ 可以         | ❌ 不行（是 opaque）      |
//! | 适合层级      | library / 领域层       | application / CLI / script |
//! | 代码量        | 稍多（要写 enum）       | 极少                 |
//! | 能混用吗？      | 可以：库用 thiserror，应用包装层用 anyhow ||

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        thiserror
// -------------------------------------------

use core::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O Error:")]
    Io(#[from] std::io::Error),

    #[error("Prase Error:")]
    Parse(#[from] ParseIntError),
}

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Result<(), fmt::Error>
//         match self {
//             AppError::Io(e) => write!(f, "I/O Error: {}", e),
//             AppError::Parse(e) => write!(f, "Parse Error: {}", e),
//         }
//     }
// }
// impl From<Error> for AppError {
//     fn from(err: Error) -> AppError {
//         AppError::Io(err)
//     }
// }

// impl From<ParseIntError> for AppError {
//     fn from(err: ParseIntError) -> AppError {
//         AppError::Parse(err)
//     }
// }

fn read_and_parse_number(file_path: String) -> Result<i32, AppError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn main() {
    let file = String::from("number.txt");
    match read_and_parse_number(file) {
        Ok(number) => println!("File contains number: {}", number),
        Err(e) => {
            println!("{:?}", e);
            match e {
                AppError::Io(err) => std::process::exit(1),
                AppError::Parse(err) => std::process::exit(2),
            }
        }
    }
}

/*
--------------------------------------------------------------------------------------
Concept/Topic               | Explanation
--------------------------------------------------------------------------------------
Purpose of thiserror        | - Simplifies implementing the standard Error trait.
                            | - Automates Display, Debug, and Error implementations.
                            | - Designed for well-typed, structured errors.

Using thiserror Attributes  | - #[error("...")] defines display messages per variant.
                            | - #[from] enables automatic error conversions.
                            | - Reduces repetitive trait implementations.

Error Type Preservation     | - Error variants remain concrete and distinguishable.
                            | - Enables matching on specific error cases.
                            | - Supports fine-grained error handling logic.
--------------------------------------------------------------------------------------
*/

/*
Comparison
-------------------------------------------------------------------------------------------
anyhow                                          | thiserror
-------------------------------------------------------------------------------------------
Application-level error handling                | Library-level error definitions

Single catch-all (anyhow::Error)                | Custom typed errors (usually enum)

Can not match on specific types after wrapping  | Matching in specific error types are possible

provides context(), with_context() to add       | attributes attachment, #[error("...")]
information dynamically at runtime              |
-------------------------------------------------------------------------------------------

*/
"###;
