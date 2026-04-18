//! Rust 的错误处理哲学：**把错误当成普通的返回值**。
//!
//! Rust 没有"异常"。错误分两类：
//!
//! | 类型           | 机制          | 什么时候用                        |
//! |--------------|-------------|-------------------------------|
//! | **不可恢复**     | `panic!`    | bug、不变式被破坏、继续执行毫无意义        |
//! | **可恢复**       | `Result<T, E>` | 业务/环境因素导致的失败（文件不存在、解析失败、网络断开） |
//!
//! 关键事实：
//! - 可恢复错误**写在类型里**——调用方必须在编译期决定如何处理
//! - 没有"被偷偷 throw 然后调用栈某处 catch"的隐式控制流
//! - `Result<T, E>` 是个 enum：`Ok(T)` 或 `Err(E)`
//! - `Option<T>` 和 Result 共用相似的组合子，但 Option 用于"值可能不存在"，Result 用于"操作可能失败"

use std::{fs::File, num::ParseIntError};

fn parse_input_to_int(input: &str) -> Result<i32, ParseIntError> {
    input.trim().parse::<i32>()
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        Error Handling
// -------------------------------------------
// Error Types
// 1. Unrecoverable Errors
// 2. Recoverable Errors

use std::fs::File;

// Examples of unrecoverable errors
// Example 1: Unimplemented code
fn unimplemented_feature() {
    panic!("This feature is not implemented yet");
}

// Example 2: Invalid values
fn value_processing(value: i32) {
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => panic!("unexpected value: {}", value),
    }
}

// Example 3: Critical conditions of tests
fn must_be_positive(n: i32) {
    assert!(n > 0, "Value must be positive, got {}", n);
}
fn main() {
    // Example 4: Index out of bound
    // let v = vec![1, 2, 3];
    // println!("{}", v[5]);

    // Examples of recoverable errors
    // Example 1: File opening
    let file = File::open("missing.txt");
    match file {
        Ok(f) => println!("File opened successfully: {:?}", f),
        Err(e) => println!("Failed to open file: {}", e),
    }

    // Example 2: Parsing integer
    let user_input = "42a";
    match parse_input_to_int(user_input) {
        Ok(n) => println!("Prased number: {}", n),
        Err(e) => println!("Invalid input: {}", e),
    }
}

fn parse_input_to_int(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse::<i32>()
}


/* 
-----------------------------------------------------------------------------------
Concept/Topic          | Explanation 
-----------------------|-----------------------------------------------------------
Rust Error Handling    | Rust handles errors as part of normal program flow.
(Explicit Approach)    | Unlike exceptions, control flow remains visible and predictable.

Error Categories       | Rust categorizes errors into unrecoverable and recoverable. 

Unrecoverable Errors   | Indicate serious problems and program must stop.
                       | Handled using the `panic!` macro.
                       | Often signals a bug or invalid program state.

Common Unrecoverable   | Accessing unimplemented code during incremental development.
Scenarios              | Reaching an invalid state/value that cannot be handled.
                       | Inputs not conforming to required formats or ranges.
                       | Critical test failures using indicated using `assert!` (intentional panic)
                       | Out-of-bounds indexing errors.

Recoverable Errors     | Errors that can be handled gracefully without stopping execution.
(Result)               | Typically represented using the `Result` enum.
                       | Useful for expected failures like missing files or invalid input.

Handling Recoverable   | Result can be handled using match.
Errors with match      | Ok branch: perform success actions (e.g., read/write file)
                       | Err branch: print or return error, guide user with next steps
----------------------------------------------------------------------------------
*/
"###;
