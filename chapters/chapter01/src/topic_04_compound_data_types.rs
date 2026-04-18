//! 复合（compound）类型：元组（tuple）与数组（array）。
//!
//! 这两种是 Rust 最原始的"把多个值打包在一起"的办法：
//! - **tuple**：成员类型可以各不相同，长度固定，按位置（`.0`、`.1`）访问
//! - **array**：成员类型完全一致，长度固定（编译期已知），按下标（`[i]`）访问
//!
//! 它们都是**固定大小**的值类型——放在栈上，默认 `Copy`（只要成员都 `Copy`）。
//! 动态可变长度的 `Vec` 会在后面章节出现，这里不展开。
//!
//! 运行后请观察：
//! 1. 元组的类型签名和访问方式
//! 2. 元组解构
//! 3. 数组的类型签名 `[T; N]` 里 N 是**类型的一部分**
//! 4. 数组越界在 **debug** 下会 panic（本节不触发，只提示）
//! 5. 如何遍历数组

/// 1) 元组基础：不同类型组合到一起。
fn demonstrate_tuple_basics() {
    let user_profile: (&str, i32, char) = ("alice", 20, 'A');

    // 按位置访问：`.0`、`.1`、`.2`。
    println!(
        "元组按位置访问: name = {}, age = {}, grade = {}",
        user_profile.0, user_profile.1, user_profile.2
    );

    // 元组类型签名是成员类型的组合：`(&str, i32, char)`。
    // 长度变了 / 类型变了，就是**不同**的元组类型。
}

/// 2) 元组解构：一次性把元素绑定到多个名字。
fn demonstrate_tuple_destructuring() {
    let rgb = (255, 128, 0);
    let (red, green, blue) = rgb;
    println!("解构元组: r = {red}, g = {green}, b = {blue}");

    // 想忽略某个字段时，用 `_`。
    let (_, only_green, _) = rgb;
    println!("只取中间: only_green = {only_green}");
}

/// 3) 嵌套元组：tuple 里也能放 tuple。
fn demonstrate_nested_tuple() {
    let point_and_label = ((3, 4), "origin");
    // 双层访问：先按位置取到内层 tuple，再继续 .0/.1。
    let (coord, label) = point_and_label;
    println!("nested tuple: coord = {:?}, label = {label}", coord);
    println!("直接链式访问: x = {}, y = {}", point_and_label.0.0, point_and_label.0.1);
}

/// 4) unit `()`：长度为 0 的元组，也是很多函数的默认返回值类型。
fn demonstrate_unit_tuple() {
    let nothing: () = ();
    println!("unit 类型 () = {:?} —— 它是‘没有信息’的值", nothing);
}

/// 5) 数组基础：`[T; N]`，N 是类型的一部分。
fn demonstrate_array_basics() {
    // 类型签名 `[i32; 5]` 里，5 属于**类型**——`[i32; 5]` 和 `[i32; 6]` 是不同类型。
    let scores: [i32; 5] = [90, 85, 88, 92, 95];

    println!("array: {scores:?}");
    println!("第一个元素 scores[0] = {}", scores[0]);
    println!("最后一个元素 scores[4] = {}", scores[4]);
    println!("长度 scores.len() = {}", scores.len());

    // 注意：`scores[10]` 在 debug 下会 panic，提示 "index out of bounds"。
    // 这里我们不触发它，因为要保持整个 chapter 可运行。
}

/// 6) 用"相同值填充"的简写：`[value; N]`。
fn demonstrate_array_fill_syntax() {
    let zeros: [i32; 5] = [0; 5];
    let hellos: [&str; 3] = ["hello"; 3];
    println!("填充写法 [0; 5] = {zeros:?}");
    println!("填充写法 [\"hello\"; 3] = {hellos:?}");
}

/// 7) 遍历数组：`for` + 引用迭代，避免把数组 move 掉。
fn demonstrate_array_iteration() {
    let fruits = ["apple", "banana", "cherry"];

    // `for x in fruits` 会消费数组（对非 Copy 元素会 move）；
    // 更通用的写法是用 `.iter()` 拿到引用迭代器。
    for fruit in fruits.iter() {
        println!("fruit = {fruit}");
    }

    // enumerate() 同时拿到下标和值。
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("idx = {idx}, fruit = {fruit}");
    }
}

/// 8) 数组切片：`&arr[..]` 得到 `&[T]`。
///
/// 这是一个重要细节：函数参数写 `&[T]` 比写 `[T; N]` 通用得多——
/// 任何长度的数组都能当切片传进去。这部分在第 2 章借用里会详细讲，这里先留个印象。
fn demonstrate_slice_preview() {
    let numbers = [10, 20, 30, 40, 50];
    let head: &[i32] = &numbers[..3];
    let tail: &[i32] = &numbers[3..];
    println!("head = {head:?}, tail = {tail:?}");
}

pub fn run() {
    demonstrate_tuple_basics();
    println!();

    demonstrate_tuple_destructuring();
    println!();

    demonstrate_nested_tuple();
    println!();

    demonstrate_unit_tuple();
    println!();

    demonstrate_array_basics();
    println!();

    demonstrate_array_fill_syntax();
    println!();

    demonstrate_array_iteration();
    println!();

    demonstrate_slice_preview();
    println!();
}
