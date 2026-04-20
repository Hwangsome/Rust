//! 错误传播：用 `?` 把"失败就直接返回"这件事压缩成一个符号。
//!
//! `?` 操作符的**展开语义**（大约等价于）：
//!
//! ```ignore
//! let x = expr?;
//! // 等价于：
//! let x = match expr {
//!     Ok(v)  => v,
//!     Err(e) => return Err(From::from(e)),
//! };
//! ```
//!
//! 关键三点：
//! 1. `?` 只能写在返回类型是 `Result<_, _>` / `Option<_>` / 实现 `Try` 的函数里
//! 2. 它会**自动调用 `From::from`** 把 `e` 转成当前函数的 `E` 类型——所以只要实现了 `From`，
//!    不同 crate 的错误类型能无缝接在一起
//! 3. `?` 不是"忽略错误"——恰恰相反，它强制错误向上传播，让**调用者**决定怎么处理

use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    let number = input.trim().parse::<i32>()?;
    Ok(number)
}

fn extract_username(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    email.get(..at_pos)
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        Propagating Errors
// -------------------------------------------

use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    // match input.trim().parse::<i32>() {
    //     Ok(num) => Ok(num),
    //     Err(e) => Err(e),
    // }

    /*
    let num = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    */
    let num = input.trim().parse::<i32>()?;
    Ok(num)
}

fn extract_username(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    let username = email.get(0..at_pos)?;
    Some(username)
}
fn main() {}

/* 
--------------------------------------------------------------------------------------------------------------
Concept/Topic              | Explanation
---------------------------|----------------------------------------------------------------------------------
Error Propagation          | Passing an error from a lower-level function to its caller. 
                           | This allows to avoid handling errors immediately at failure points.

Question Mark Operator (?) | A shorthand for matching on a Result or Option. 
                           | It returns early if an error (Err) or absence (None) occurs.  
                           | Otherwise it unwraps the successful value and remaining code executes normally.

? Simplifyies Code         | Replaces verbose match expressions, involving functions returning Result/Option.

Multiple Uses of ?         | The ? operator can be used multiple times within a single function. 
                           | This is useful when we want to check several fallible operations.
-------------------------------------------------------------------------------------------------------------- 
*/
"###;
