//! 分层结果（part 2）：`Option<Result<T, E>>` —— "操作可能不发生；发生了再分成/败"。
//!
//! 与上一节相反——这里的外层 `Option` 是在问："我**要不要**做这个操作？"
//!
//! | 形状         | 语义                  |
//! |-----------|---------------------|
//! | `None`          | 根本没尝试（输入为空 / 条件不满足 / 用户没填）|
//! | `Some(Ok(t))`   | 尝试了，成功了               |
//! | `Some(Err(e))`  | 尝试了，失败了               |
//!
//! 典型场景：**可选表单字段**（不填就 None，填了再做校验）、**条件解析**（某个 flag 开才去 parse）。
//!
//! 对比 part 1：
//! - `Result<Option<T>, E>` —— 操作**一定会发生**，结果可能没有数据
//! - `Option<Result<T, E>>` —— 操作**可能不发生**，发生了再看成/败

use std::num::ParseIntError;

fn handle_user_registration(
    name: &str,
    age_input: Option<&str>,
) -> Option<Result<u32, ParseIntError>> {
    println!("registering user: {name}");
    age_input.map(|value| value.parse::<u32>())
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Layered Outcomes: Option<Result<T, E>> ==");

    println!(
        "age provided and valid => {:?}",
        handle_user_registration("Alice", Some("25"))
    );
    println!(
        "age provided but invalid => {:?}",
        handle_user_registration("Bob", Some("twenty"))
    );
    println!(
        "age not provided => {:?}",
        handle_user_registration("Carol", None)
    );
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// --------------------------------------------------------
// 	    Layering Result and Option Together (Part 2)
// --------------------------------------------------------

// Result<Option<T>, E>
// Option<Result<T, E>>

use std::num::ParseIntError;

fn handle_user_registration(
    name: &str,
    age_input: Option<&str>,
) -> Option<Result<u32, ParseIntError>> {
    println!("Registering user: {}", name);
    age_input.map(|s| s.parse::<u32>())

    // None -> None                             // Operation not attempted
    // Some(value) -> Some(Ok(valid_age))       // Operation attempted and
    // successful Some(age) -> Some(Err(ParseIntError))    // Operation
    // attempted but failed
}
fn main() {
    match handle_user_registration("Alice", Some("25")) {
        None => println!("Age not provided, continuing without age"),
        Some(Ok(age)) => println!("User age is valid: {}", age),
        Some(Err(e)) => println!("Invalid age input: {}", e),
    }
}

/*
Summary
----------------------------------------------------------------------------------------------
Concept/Topic                 | Explanation
----------------------------------------------------------------------------------------------
Option<Result<T, E>> Pattern  | Represents an optional operation.
                              | Outer Option indicates whether the operation was attempted.
                              | Inner Result indicates success or failure if attempted.

Two-Stage Semantics           | Stage one answers whether execution occurred
                              | Stage two reports success or failure
                              | Cleanly separates intent from outcome

 Use Cases                    | Suitable when an operation may be skipped
                              | Form validation, Invalid input
                              | Common in user-driven or conditional logic
----------------------------------------------------------------------------------------------
*/

/*
Comparison
-------------------------------------------------------------------------------------------
Aspect         | Result<Option<T>, E>                 | Option<Result<T, E>>
-------------------------------------------------------------------------------------------
Operatin type  | The operation must run               | The operation is optional.

Outcomes       | Ok(Some(t)) —> success with a value  | None — operation not run
               | Ok(None) —> success but no value     | Some(Ok(t)) — tried and successful
               | Err(e) — actual error                | Some(Err(e)) — tried but failed

Use Case       | APIs, DB lookup                      | Form field validation
-------------------------------------------------------------------------------------------
 */
"###;
