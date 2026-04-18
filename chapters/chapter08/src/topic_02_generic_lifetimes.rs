//! 泛型生命周期参数 `<'a>`：**描述多个引用之间的关系**，不是延长寿命。
//!
//! 最典型的例子：`fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`
//!
//! 含义："调用方给我两个引用，它们都至少活 `'a` 这么久；我承诺返回的引用也活 `'a`。"
//! - 编译器会推出 `'a` = 两个输入里**较短**的那个的生命周期
//! - 所以返回值最多活到"两个输入中先死的那个"为止
//!
//! 特殊生命周期 `'static`：
//! - 活**整个程序运行期**
//! - 字符串字面量 `"hello"` 的类型就是 `&'static str`
//!
//! 本节演示 4 个子场景：
//! 1. `longer` 函数的基本用法
//! 2. 同作用域内调用：生命周期充裕
//! 3. 跨作用域：被限制为较短那一个
//! 4. `&'static` 引用的特殊地位

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}

/// 只有一个输入的例子：`'a` 被推断为输入的生命周期。
fn first_char<'a>(s: &'a str) -> &'a str {
    &s[..1]
}

pub fn run() {
    println!("== Generic Lifetimes ==");

    println!("-- (1) longer 基本用法 --");
    let title = String::from("Rust");
    let topic = "ownership";
    let chosen = longer(title.as_str(), topic);
    println!("longer = {chosen}");
    println!();

    println!("-- (2) 跨作用域：返回值寿命等于较短输入 --");
    let long_lived = String::from("long_lived_string");
    {
        let short_lived = String::from("short");
        let picked = longer(&long_lived, &short_lived);
        println!("在内层作用域里 picked = {picked}");
        // picked 的生命周期 = 较短的 short_lived
        // 不能把 picked 带出这个作用域。
    }
    println!("内层作用域结束后 long_lived 仍可用 = {long_lived}");
    println!();

    println!("-- (3) 单参数 --");
    let s = String::from("abc");
    println!("first_char(&s) = {}", first_char(&s));
    println!();

    println!("-- (4) &'static：活整个程序 --");
    let literal: &'static str = "string literal lives forever";
    println!("literal = {literal}");
    println!("所有字符串字面量默认就是 &'static str");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Generic Lifetimes
// -------------------------------------------

// Example 1:
/* fn main() {
    let int1 = 5;
    let int2 = 10;
    let picked_value = picking_int(&int1, &int2);
    println!("{picked_value}");
}

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}
*/

// Example 2:
/*
fn main(){
let int1 = 5;
    {
        let int2 = 10;
        let picked_value = picking_int(&int1, &int2);
        println!("{picked_value}");
    }
}

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}
*/

// Example 3:
/* fn main() {
    let int1 = 5;
    let picked_value;
    {
        let int2 = 10;
        picked_value = picking_int(&int1, &int2);
    }
    //println!("{picked_value}");
}

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}
*/

// Example 4:
fn main() {
    let int1 = 5;
    let picked_value;
    {
        let int2 = 10;
        picked_value = picking_int(&int1, &int2);
    }
    println!("{picked_value}");
}

fn picking_int(i: &i32, j: &i32) -> &'static i32 {
    let y: &'static i32 = &6;
    y
}

/*
-----------------------------------------------------------------------------------------------------------
Concept / Topic        | Explanation
-----------------------|-----------------------------------------------------------------------------------
Generic Lifetime       | They describe relationships between concrete reference lifetimes.
                       | They are written using a tick followed by a name, such as 'a.
                       | They connect input lifetimes with the return lifetime.

Shortest Lifetime Rule | When multiple parameters share a lifetime parameter, the return lifetime
                       | is the shortests of the input lifetimes.

Some key points        | A function may specify that the return lifetime matches only one parameter.
                       | In such cases, only a single (and not all) parameter needs lifetime annotation.
                       | Returning a reference to a function local variable is invalid (dangling).

Static lifetimes       | A 'static lifetime means the reference lives for the entire program.
                       | A 'static reference satisfies any shorter lifetime requirement.
-----------------------------------------------------------------------------------------------------------
*/
"###;
