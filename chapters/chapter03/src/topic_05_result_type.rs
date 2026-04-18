//! `Result<T, E>`：成功拿 `Ok(T)`，失败拿 `Err(E)`。
//!
//! Rust 没有"异常"，错误就是**一个普通的返回值**：
//!
//! ```ignore
//! enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```
//!
//! 这让"错误"彻底显式化——调用方必须在类型层面决定如何处理错误。
//!
//! 本节演示：
//! 1. `match` 两路处理
//! 2. `?` 操作符传播错误（和函数签名之间的关系）
//! 3. 组合子：`map` / `map_err` / `and_then` / `unwrap_or_else`
//! 4. 自定义错误 enum（最小骨架，第 11+ 章还会继续深入）

use std::num::ParseIntError;

/// 自定义错误：把"可能出错的地方"枚举出来。
///
/// 入门阶段做成这样即可，后面章节会引入 `thiserror`、`anyhow`、`impl std::error::Error`。
#[derive(Debug)]
#[allow(dead_code)] // `ParseFailure` 内层字段仅用于 Debug 打印
enum CalcError {
    ParseFailure(ParseIntError),
    DivideByZero,
}

/// 把"可能产生不同错误"的函数组合起来：每一步用 `?` 传播。
fn parse_and_halve(input: &str) -> Result<i32, CalcError> {
    // str::parse 返回 `Result<i32, ParseIntError>`，和签名的 `CalcError` 不完全一致。
    // 所以我们需要先把 `ParseIntError` 映射成 `CalcError`。
    let n: i32 = input.parse().map_err(CalcError::ParseFailure)?;

    // 对 0 特殊处理：直接返回自己定义的错误。
    if n == 0 {
        return Err(CalcError::DivideByZero);
    }

    Ok(100 / n)
}

pub fn run() {
    println!("== Result ==");

    println!("-- (1) match 两路处理 --");
    let parsed: Result<i32, _> = "42".parse();
    match parsed {
        Ok(value) => println!("Ok: {value}"),
        Err(error) => println!("Err: {error}"),
    }
    let parse_err: Result<i32, _> = "not-a-number".parse();
    match parse_err {
        Ok(value) => println!("Ok: {value}"),
        Err(error) => println!("Err: {error}"),
    }
    println!();

    println!("-- (2) 组合子: map / map_err / unwrap_or_else --");
    let ok: Result<i32, &str> = Ok(10);
    let mapped = ok.map(|x| x * 2); // Ok(10) -> Ok(20)
    println!("map: {mapped:?}");

    let err_case: Result<i32, &str> = Err("bad");
    let normalized: Result<i32, String> = err_case.map_err(|e| format!("归一化: {e}"));
    println!("map_err: {normalized:?}");

    let fallback = normalized.unwrap_or_else(|e| {
        println!("  (触发 unwrap_or_else，错误: {e})");
        -1
    });
    println!("fallback value = {fallback}");
    println!();

    println!("-- (3) ? 操作符 + 自定义错误 --");
    for input in ["5", "0", "abc"] {
        println!("parse_and_halve({input:?}) => {:?}", parse_and_halve(input));
    }
    println!();

    println!("-- (4) Result 链式 and_then --");
    let pipeline = "20"
        .parse::<i32>()
        .map_err(|e| format!("parse: {e}"))
        .and_then(|n| if n > 0 { Ok(n * 3) } else { Err(String::from("not positive")) });
    println!("pipeline = {pipeline:?}");
    println!();
}
