//! 分层结果（part 1）：`Result<Option<T>, E>` —— "一定会执行，可能找到 / 找不到 / 失败"。
//!
//! 三种结局清晰分开：
//!
//! | 形状         | 语义               | 典型处理                  |
//! |-----------|------------------|-----------------------|
//! | `Ok(Some(t))`   | 查询成功，有结果         | 正常走业务               |
//! | `Ok(None)`      | 查询合法完成，但没数据      | 产品未找到、缓存 miss       |
//! | `Err(e)`        | 执行失败（网络/DB 故障）   | 记录、重试、向上抛         |
//!
//! **关键区分**：`Ok(None)` 不是错误！不要把 "找不到用户" 当成 Err，否则调用方会把
//! 业务上的 "空查询结果" 和 "系统故障" 混在同一个 match 里。
//!
//! 典型场景：数据库查询、缓存读取、配置查找。

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum DbError {
    ConnectionFailed,
}

fn db_connection(available: bool) -> Result<(), DbError> {
    if available {
        Ok(())
    } else {
        Err(DbError::ConnectionFailed)
    }
}

fn find_product(id: u32, connection_available: bool) -> Result<Option<Product>, DbError> {
    db_connection(connection_available)?;

    match id {
        0..=100 => Ok(Some(Product {
            id,
            name: "Laptop".to_string(),
        })),
        _ => Ok(None),
    }
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Layered Outcomes: Result<Option<T>, E> ==");

    match find_product(10, true) {
        Ok(Some(product)) => println!(
            "existing product => id = {}, name = {}",
            product.id, product.name
        ),
        Ok(None) => println!("existing product => none"),
        Err(error) => println!("existing product => error: {:?}", error),
    }
    println!("missing product => {:?}", find_product(999, true));
    println!("connection failure => {:?}", find_product(10, false));
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// --------------------------------------------------------
// 	    Layering Result and Option Together (Part 1)
// --------------------------------------------------------

// Result<Option<T>, E>
// Three-way Outcome -> Ok(Some(val))   for success with value,
//                   -> Ok(None)        for success but no value,
//                   -> Err(e)          for failure.

// Option<Result<T, E>>
use rand::Rng;
struct Product {
    id: u32,
    name: String,
}

enum DBError {
    ConnectionFailed,
}

fn db_connection() -> Result<(), DBError> {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < 0.1 {
        Err(DBError::ConnectionFailed)
    } else {
        Ok(())
    }
}
fn find_product(id: u32) -> Result<Option<Product>, DBError> {
    // The query failed
    // The query succeeded with a valid product
    // The query succeeded but no valid product
    db_connection()?;
    match id {
        0..=100 => Ok(Some(Product {
            id,
            name: "Laptop".to_string(),
        })),
        _ => Ok(None),
    }
}
fn main() {}

/*
-----------------------------------------------------------------------------------
Concept/Topic                | Explanation
-----------------------------|-----------------------------------------------------
Option and Result Outcomes   | Option models presence or absence of a value.
                             | Result models success or failure with an error.
                             | Real-world logic is often complex, requiring combining both.

Layered Outcome Motivation   | Single binary outcomes are often insufficient.
                             | Operations may succeed without producing meaningful data.
                             | Absence of a value may be valid, not an error.

Result<Option<T>, E> Pattern | Represents a fallible operation.
                             | Success is captured by Ok, containing Some value or None.
                             | Failure is represented by the Err.

Three-Way Semantic           | Operation failed due to an error.
                             | Operation succeeded with a valid value.
                             | Operation succeeded but no value exists.

Database Query Use Case      | Database connection itself may fail.
                             | Successful query may or may not find data.
                             | Product lookup naturally maps to Result<Option<T>, E>.

 Error vs Absence separation | Errors indicate exceptional failure conditions.
                             | None represents expected absence of data.
                             | Clear distinction improves correctness and clarity.
-----------------------------------------------------------------------------------

 */
"###;
