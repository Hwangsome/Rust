//! Struct 基础：把相关字段打包成一个有语义的整体。
//!
//! Rust 有三种 struct：
//! 1. **具名字段**（named-field）：`struct User { name: String, age: u32 }`
//! 2. **元组结构体**（tuple struct）：`struct Point(i32, i32)`——字段只有位置，没有名字
//! 3. **单元结构体**（unit struct）：`struct Marker;`——没有字段，常用作类型层面的标记
//!
//! 本节要同时演示**怎么定义**、**怎么构造**、**怎么打印**。
//! 打印时用 `#[derive(Debug)]` + `{:?}`，这一步看似小细节，其实非常重要——
//! 没有它，自定义类型默认**不能**被 `println!` 打印。
//!
//! 另外要建立两个"构造惯用法"的直觉：
//! - **字段简写**（field init shorthand）：当变量名和字段名相同时，可以 `User { name, age }`
//! - **更新语法**（struct update syntax）：`User { active: false, ..old }` 复用其余字段

/// 1) 具名字段 struct：最常见的形式。
#[derive(Debug)]
#[allow(dead_code)] // 教学示例里有些字段只用 Debug 打印，未做读取
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

/// 2) 元组结构体：给"一组值"一个**类型名**但不给每个字段起名。
///
/// 常用来做"强类型别名"：比如 `struct Meters(f64)`，避免把"米"和"英尺"的 `f64` 相互混用。
#[derive(Debug)]
struct Rgb(u8, u8, u8);

/// 3) 单元结构体：没有字段。常用来给 trait 实现一个"纯标记类型"。
#[derive(Debug)]
struct AlwaysEqual;

fn demonstrate_named_struct_basics() {
    println!("-- (1) 具名字段 struct --");

    // 最直接的构造方式：字段名 : 值。
    let alice = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        login_count: 0,
    };

    // 点号访问字段。
    println!("username = {}", alice.username);
    println!("email    = {}", alice.email);
    println!("active   = {}", alice.active);

    // 整体用 Debug 打印——需要 `#[derive(Debug)]`，用 `{:?}`。
    println!("整体打印: {alice:?}");

    // pretty Debug 会自动换行缩进，适合嵌套结构。
    println!("pretty 打印:\n{alice:#?}");
}

fn demonstrate_field_init_shorthand() {
    println!("-- (2) 字段简写 --");

    // 当传入的变量名恰好等于字段名时，可以省略 `name: name,`，写 `name,`。
    fn build_user(username: String, email: String) -> User {
        User {
            username, // 等价于 `username: username,`
            email,    // 等价于 `email: email,`
            active: true,
            login_count: 0,
        }
    }

    let bob = build_user(String::from("bob"), String::from("bob@example.com"));
    println!("通过字段简写构造: {bob:?}");
}

fn demonstrate_struct_update_syntax() {
    println!("-- (3) 更新语法 `..old` --");

    let alice = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        login_count: 0,
    };

    // `..alice` 表示"其余字段从 alice 里取"。
    // 注意：这会**move** alice 的非 Copy 字段（如 String），所以之后 alice 不可用。
    let alice_v2 = User {
        email: String::from("alice_v2@example.com"),
        login_count: 100,
        ..alice
    };

    println!("通过更新语法构造 alice_v2: {alice_v2:?}");
    // println!("alice = {alice:?}"); // ← 此处会触发 E0382，因为 username 已被 move
}

fn demonstrate_tuple_struct() {
    println!("-- (4) 元组结构体 --");

    let red = Rgb(255, 0, 0);
    let sky_blue = Rgb(135, 206, 235);

    // 元组结构体按位置访问：`.0`、`.1`、`.2`。
    println!("red = ({}, {}, {})", red.0, red.1, red.2);
    println!("sky_blue = {sky_blue:?}");

    // 强类型别名的价值：下面这两行语法上合法，类型上却是**不同的**：
    // struct Meters(f64); struct Feet(f64);
    // let m: Meters = Meters(1.0);
    // let f: Feet = m; // ← 会报 E0308，防止单位混用
}

fn demonstrate_unit_struct() {
    println!("-- (5) 单元结构体 --");

    let marker = AlwaysEqual;
    println!("单元结构体的值只有一种形态: {marker:?}");
    // 典型用途：给 marker 实现某个 trait（比如 `Default`、`Iterator`），
    // 让它在类型系统里扮演"标签"的角色。
}

pub fn run() {
    println!("== Structs Basics ==");
    demonstrate_named_struct_basics();
    println!();
    demonstrate_field_init_shorthand();
    println!();
    demonstrate_struct_update_syntax();
    println!();
    demonstrate_tuple_struct();
    println!();
    demonstrate_unit_struct();
    println!();
}
