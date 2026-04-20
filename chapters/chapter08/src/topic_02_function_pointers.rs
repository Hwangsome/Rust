//! 函数指针（`fn(...) -> ...`）：最窄的"可调用"类型。
//!
//! `fn(i32) -> i32` 是一个**具体类型**，只能装：
//! 1. 真正的函数项（由 `fn` 关键字定义的）
//! 2. **零捕获**闭包（不捕获任何环境的那种）
//!
//! 凡是捕获了环境的闭包，都不能退化为 `fn`——那种情况必须用 `impl Fn`/`Box<dyn Fn>` 接收。
//!
//! 本节演示 4 个对照：
//! 1. 传入普通函数 → 合法
//! 2. 传入零捕获闭包 → 合法
//! 3. 传入捕获闭包 → 必须改用 `impl Fn` 参数
//! 4. `fn` 指针天然实现 `Fn + FnMut + FnOnce`——反向完全兼容

fn add_one(value: i32) -> i32 {
    value + 1
}

/// 只接 **函数指针**——签名最窄，但不能接捕获闭包。
fn apply_twice_fnptr(value: i32, op: fn(i32) -> i32) -> i32 {
    op(op(value))
}

/// 接收任意可调用对象（包括捕获闭包）——更通用。
fn apply_twice_generic<F: Fn(i32) -> i32>(value: i32, op: F) -> i32 {
    op(op(value))
}

pub fn run() {
    println!("== Function Pointers ==");

    println!("-- (1) 传普通函数 --");
    println!("apply_twice_fnptr(3, add_one) = {}", apply_twice_fnptr(3, add_one));
    println!();

    println!("-- (2) 传零捕获闭包 --");
    let double = |x: i32| x * 2; // 没有捕获外部变量
    println!("apply_twice_fnptr(3, double)  = {}", apply_twice_fnptr(3, double));
    println!();

    println!("-- (3) 捕获闭包传不进 fn 指针参数 --");
    let offset = 5;
    let add_offset = |x: i32| x + offset; // 捕获了 offset

    // apply_twice_fnptr(3, add_offset); // ← 这行会 E0308：捕获闭包不能当 fn 指针
    // 改用 impl Fn 参数：
    println!("apply_twice_generic(3, add_offset) = {}", apply_twice_generic(3, add_offset));
    println!();

    println!("-- (4) fn 指针满足 Fn / FnMut / FnOnce 三个 trait 的 bound --");
    // 所以 fn 指针能当作任意闭包 trait 的实参传进泛型函数。
    println!("apply_twice_generic(3, add_one) = {}", apply_twice_generic(3, add_one));
    println!();

    println!("选型建议：");
    println!("  优先用 `impl Fn(...) -> ...` 写参数——同时接受函数和闭包");
    println!("  只在跨 FFI / 存储到固定大小数组 / 只需零捕获场景时才写 `fn(...)`");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Function Pointers
// -------------------------------------------
struct User {
    name: String,
    age: u8,
    salary: u32,
}

fn is_valid_user(
    name: &str,
    banned_user_name: &str,
    age: u8,
    simple_validator: fn(&str, &str) -> bool,
    advance_validator: fn(u8) -> bool,
) -> bool {
    simple_validator(name, banned_user_name) && advance_validator(age)
}

fn validate_user_simple(name: &str, banned_user_name: &str) -> bool {
    name.len() != 0 && name != banned_user_name
}

fn validate_user_advance(age: u8) -> bool {
    age >= 30
}
fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };
    let banned_user = "banned user";

    // let validate_user_simple = |name: &str| name.len() != 0;
    // let validate_user_advance = |age: u8| age >= 30;

    println!(
        "User validity {}",
        is_valid_user(
            &person_1.name,
            banned_user,
            person_1.age,
            validate_user_simple,
            validate_user_advance
        )
    );
}

/* 
---------------------------------------------------------------------------------------------------------
Concept/Topic              | Explanation
---------------------------|-----------------------------------------------------------------------------
Function Pointers          | Function pointers are pointers to regular functions.
                           | They do not capture variables from their environment.
                           | They can be passed where closures are expected.

Closures                   | Closures may capture environment variables whereas function pointers cannot.
vs Function Pointers       | Function pointers are concrete types written as fn(arg) -> Return.
                           | They implement all three closure traits automatically.

Passing to function        | Generics aren’t needed for passing function pointer to function.
                           | The parameter type can be written directly as fn(args) -> Return.

Conversion of Closures to  | A closure can be coerced to a function pointer only if it captures nothing.
Function Pointer           

Workaround for Conversion  | Workaround: Captured variables are passed as arguments to functions. 
---------------------------------------------------------------------------------------------------------
 */
"###;
