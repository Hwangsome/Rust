//! 闭包（closure）：既像函数又能捕获环境的匿名函数。
//!
//! 闭包的 3 种捕获方式**由编译器自动推断**，对应 3 个 trait：
//!
//! | 捕获方式      | 对应 trait  | 调用次数           |
//! |-------------|-----------|-----------------|
//! | 不可变借用 `&T` | `Fn`      | 任意次           |
//! | 可变借用 `&mut T` | `FnMut`   | 任意次（但需 `&mut` 环境） |
//! | 按值捕获（取所有权）| `FnOnce`  | **只能 1 次**          |
//!
//! 关系：`Fn` ⊂ `FnMut` ⊂ `FnOnce`（更强的能满足更弱的 bound）。
//!
//! 本节演示：
//! 1. 闭包捕获外部不可变变量（`Fn`）
//! 2. 闭包捕获外部可变变量（`FnMut`）
//! 3. `move` 强制按值捕获（`FnOnce`，但具体 trait 取决于用法）
//! 4. 闭包作参数：用 `impl Fn`/`impl FnMut`/`impl FnOnce` 做 bound
//! 5. 闭包作返回值：`-> impl Fn(...)`

fn validate_user<F: Fn(&str) -> bool>(name: &str, validator: F) -> bool {
    validator(name)
}

/// 接收一个可变调用多次的闭包（FnMut）。
fn repeat_with<F: FnMut()>(times: u32, mut f: F) {
    for _ in 0..times {
        f();
    }
}

/// 返回一个闭包：`impl Fn(i32) -> i32`。
fn make_adder(delta: i32) -> impl Fn(i32) -> i32 {
    move |x| x + delta // move 把 `delta` 的所有权搬进闭包
}

pub fn run() {
    println!("== Closures ==");

    println!("-- (1) Fn：不可变捕获外部变量 --");
    let min_length = 4;
    // 闭包捕获了 `min_length`（不可变借用）。因为只读，它满足 `Fn`。
    let is_valid_user =
        |name: &str| name.len() >= min_length && name.chars().all(char::is_alphabetic);
    println!("Alice valid => {}", validate_user("Alice", is_valid_user));
    println!("Bo    valid => {}", validate_user("Bo", is_valid_user));
    // 仍然能继续用原变量——闭包是借用。
    println!("min_length 仍可用 = {min_length}");
    println!();

    println!("-- (2) FnMut：可变捕获外部变量 --");
    let mut count = 0;
    let mut tick = || {
        count += 1; // 捕获 &mut count → FnMut
        println!("  tick #{count}");
    };
    repeat_with(3, &mut tick);
    println!("count 最终 = {count}");
    println!();

    println!("-- (3) move：强制按值捕获（常用于线程、跨作用域返回） --");
    let adder5 = make_adder(5);
    let adder10 = make_adder(10);
    println!("adder5(3)  = {}", adder5(3));
    println!("adder10(3) = {}", adder10(3));
    println!();

    println!("-- (4) FnOnce：闭包只能调一次（因为消费了捕获的值） --");
    let name = String::from("Rust");
    let consume = move || {
        // 这里的 drop 把 name 消费掉了 → 这个闭包是 FnOnce。
        drop(name);
    };
    consume();
    // consume(); // ← 第二次会 E0382：closure已经被 move 过了
    println!("FnOnce 闭包只能调 1 次（drop 掉了内部捕获的 String）");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Closures
// -------------------------------------------

struct User {
    name: String,
    age: u8,
    salary: u32,
}

// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: FnOnce(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}
fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };

    let mut banned_user = String::from("banned user");
    let validate_user_simple = move |name: &str| {
        let banned_user_name = &banned_user;
        name.len() != 0 && name != banned_user_name
    };
    //println!("{banned_user}");

    let validate_user_advance = |age: u8| age >= 30;
    println!(
        "User validity {}",
        is_valid_user(
            &person_1.name,
            person_1.age,
            validate_user_simple,
            validate_user_advance
        )
    );
}
/*
------------------------------------------------------------------------------------------------------
Concept/Topic             | Explanation
--------------------------|---------------------------------------------------------------------------
Closures Basics           | They are anonymous functions with no names. 
                          | They can be stored in variables or passed as arguments.
                          | Rust infers parameter and return types in closures when possible. 
                          | They can captured the variables from their enviroments.
                          | Each closure has a unique concrete type.
                          | They can be passed to function using generics with trait bounds.

Environment Capture Modes | Closures capture external variables in three ways.
                          | Immutable borrow in which case it implements Fn trait.
                          | Mutable borrow in which case it implements FnMut trait.
                          | Ownership move in which case it implements FnOnce trait.

move Keyword              | The move keyword forces captured variables to be moved into the closure.
                          | Even references become owned inside the closure.
                          | This prevents later access to moved variables.
------------------------------------------------------------------------------------------------------
*/
"###;
