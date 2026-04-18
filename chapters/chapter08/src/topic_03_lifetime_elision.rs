//! 生命周期省略（lifetime elision）：大多数时候你不需要手写 `'a`。
//!
//! 编译器遵循 3 条**省略规则**，按顺序尝试给每个引用参数自动标注生命周期：
//!
//! 1. **每个引用参数**各自分配一个生命周期（`fn f(x: &T) → fn f<'a>(x: &'a T)`）
//! 2. **如果只有一个输入生命周期**，输出的所有引用都复用它
//! 3. **如果有 `&self` 或 `&mut self`**，所有输出引用都复用 self 的生命周期
//!
//! 3 条规则都满足就不用写；任何一条不满足就要**手动**标注（会报 E0106）。

/// 规则 2 触发：只有一个输入引用，输出复用它。
/// 省略前：`fn first_word<'a>(text: &'a str) -> &'a str`
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// 规则 3 触发：方法里有 &self，输出复用 self 的生命周期。
struct Article {
    title: String,
}

impl Article {
    /// 省略前：`fn get_title<'a, 'b>(&'a self, _extra: &'b str) -> &'a str`
    fn get_title(&self, _extra: &str) -> &str {
        &self.title
    }
}

/// 规则不够用时：必须手写。
/// 这里两个输入引用，没有 &self，规则 2/3 都不适用——必须手写 `<'a>`。
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

pub fn run() {
    println!("== Lifetime Elision ==");

    println!("-- (1) 规则 2：单输入参数 --");
    let sentence = "borrow checker makes aliasing rules explicit";
    println!("first_word = {}", first_word(sentence));
    println!();

    println!("-- (2) 规则 3：&self 让方法自动推断 --");
    let art = Article { title: "Rust 2024".into() };
    println!("title = {}", art.get_title("ignored"));
    println!();

    println!("-- (3) 规则不够用时：手写 <'a> --");
    println!("longer(\"abc\", \"defgh\") = {}", longer("abc", "defgh"));
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        Lifetime Elision
// -------------------------------------------

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to
    all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of self is assigned to all output lifetime parameters.
*/

//Example 1:
fn main() {
    let str_1 = "some str";

    let recieved_str = return_str(&str_1);
}
// Code with Lifetime Elision
// fn return_str(s_1: &str) -> &str {
//     s_1
// }

// Code without Lifetime Elision
fn return_str<'a>(s_1: &'a str) -> &'a str {
    s_1
}

// Example 2:
/* fn main() {
    let str_1 = "some str";
    let str_2 = "other str";
    let recieved_str = return_str(&str_1, &str_2);
}

fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}
*/

/* 
--------------------------------------------------------------------------------------------------------------
Concept / Topic         | Explanation
------------------------|-------------------------------------------------------------------------------------
Lifetime Elision        | Lifetime elision allows the compiler to infer lifetimes automatically.
                        | This reduces the need for explicit lifetime annotations.
                        | If ambiguity remains after applying rules, explicit lifetimes are required.

Elision Rules           | Rule 1: Each reference parameter receives its own lifetime parameter.
                        | Rule 2: If exactly one input lifetime parameter exists, it is assigned to all outputs.
                        | Rule 3: If multiple input lifetimes exist but one is &self or &mut self, then
                        | the lifetime of self is assigned to all output lifetimes.(applies to methods)

Single Parameter Case   | In this case, Rule 1 applies. 
                        | No explicit annotation is needed.

Multiple Parameter Case | In this case, Rule 2 does not apply. 
                        | The compiler cannot infer the return lifetime and reports an error.
                        | Explicit annotations are required.
-------------------------------------------------------------------------------------------------------------- 
*/
"###;
