//! `anyhow`：**应用层**友好的"万能错误"类型。
//!
//! 核心理念：
//! - `anyhow::Error` 可以由**任何**实现了 `std::error::Error` 的类型转换而来
//! - `.context("...")` / `.with_context(|| format!("..."))` 给错误链**堆上"在哪一步失败"的上下文**
//! - 支持类似回溯的错误链打印（`{:?}` / `{:#}`）
//!
//! 适合谁？
//! - **应用代码 / 脚本 / CLI**：调用方只关心"能不能跑成功"，不需要按错误类型做精细恢复
//! - 快速原型：不想每加一种失败就写一个 enum variant
//!
//! 不适合谁？
//! - **library / API 边界**：调用方需要按错误类型分支恢复 → 请用 thiserror + 精确 enum
//!
//! 对照 topic_08（thiserror）：本节"宽松"，下一节"严谨"。两者**可以在同一项目里共存**——
//! library crate 用 thiserror，顶层应用用 anyhow 包装。

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        Anyhow
// -------------------------------------------

use anyhow::{Context, Result};
use std::{fs::File, io::Error, io::Read, num::ParseIntError};

// #[derive(Debug)]
// enum AppError {
//     Io(Error),
//     Parse(ParseIntError),
// }

// impl From<Error> for AppError {
//     fn from(value: Error) -> Self {
//         Self::Io(value)
//     }
// }

// impl From<ParseIntError> for AppError {
//     fn from(value: ParseIntError) -> Self {
//         Self::Parse(value)
//     }
// }

// Result<i32> = Result<i32, anyhow::Error>
fn read_and_parse_number(file_path: String) -> Result<i32> {
    let mut file = File::open(file_path).context("Failed to read file contents")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number = contents
        .trim()
        .parse::<i32>()
        .with_context(|| format!("Failed to parse integer from contents: {}", contents.trim()))?;
    Ok(number)
}
fn main() {
    let file = String::from("number.txt");
    match read_and_parse_number(file) {
        Ok(number) => println!("File contains number: {}", number),
        Err(e) => println!("Cannot process: {:?}", e),
    }
}

/*
------------------------------------------------------------------------------------
Concept/Topic              | Explanation
------------------------------------------------------------------------------------
Motivation for anyhow      | Handling multiple error types can make Rust code verbose.
                           | Fine-grained error enums may not be always required.
                           | anyhow provides a flexible, unified error type.

Multiple Error Type        | File opening and reading may produce I/O errors.
                           | Parsing file contents may produce parsing errors.
                           | A single function may naturally encounter different error kinds.

Custom Error Enum Approach | Requires defining a dedicated error enum.
                           | Needs manual conversion between error types.
                           | Often implemented using the From trait or map_err.

Simplification with anyhow | Eliminates the need for custom error enums.
                           | Removes manual error conversions.
                           | Uses anyhow::Result with a dynamic error type.

Adding Context to Errors   | context adds simple, static messages to errors.
                           | with_context supports dynamically generated messages.
------------------------------------------------------------------------------------
*/

/*
------------------------------------------------------------------------------------
Code without anyhow                 | Code with anyhow
------------------------------------------------------------------------------------
For functions returning multiple    | Eliminates the need for Custom
error type with error propagation,  | error types not required
custom Error types are required

Requires manual conversion          | No need for manual conversion
using `From` trait

Return type `Result<i32, Customtype>| Result<i32>

Simple Error propagation            | Error propagation enriched with context
------------------------------------------------------------------------------------
*/
s
"###;
