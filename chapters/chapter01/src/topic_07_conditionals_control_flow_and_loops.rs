//! 条件、控制流、循环：`if` / `loop` / `while` / `for`。
//!
//! Rust 的控制流比 C/Java 看起来差不多，但有几个关键差异需要一次性记住：
//!
//! - `if` 的条件**必须**是 `bool`——整数不会被自动当真假
//! - `if` 是**表达式**，所以可以 `let x = if cond { a } else { b };`
//! - `loop` 可以**返回值**：`break value;` 会让整个 `loop` 产出这个值
//! - `while let` 和 `if let` 是非常常用的简写（本章不展开，后面章节会讲）
//! - `for` 最常搭配 range（`1..5`、`1..=5`）或迭代器
//! - 带标签的 break/continue（`'outer: loop { ... break 'outer; }`）用来跳出嵌套循环

/// 1) `if` 是表达式，可以直接参与赋值。
fn demonstrate_if_as_expression() {
    let temperature = 28;

    let label = if temperature >= 30 {
        "hot"
    } else if temperature >= 20 {
        "warm"
    } else {
        "cool"
    };

    println!("temperature = {temperature} => {label}");

    // 注意：各分支的类型必须一致，否则会 E0308。
    // let bad = if temperature > 0 { 1 } else { "zero" }; // 不同类型，编译失败
}

/// 2) `loop` 可以带返回值：用 `break value;` 把结果送出去。
///
/// 这在"找到就停"这类场景非常自然，比到处写 `flag` 变量清爽。
fn demonstrate_loop_with_break_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter * counter >= 50 {
            // 这个 `break` 带了值 `counter`，于是整个 `loop` 表达式产出这个值。
            break counter;
        }
    };

    println!("第一个平方 >= 50 的自然数是 {result}");
}

/// 3) `while`：适合"条件成立就继续"的场景。
fn demonstrate_while() {
    let mut index = 0;
    while index < 3 {
        println!("while: index = {index}");
        index += 1;
    }
}

/// 4) `for` + range：最常用的顺序遍历写法。
///
/// - `a..b`：左闭右开（不含 b）
/// - `a..=b`：闭区间（含 b）
/// - `.step_by(n)`：每 n 步取一次
/// - `.rev()`：反向
fn demonstrate_for_with_ranges() {
    print!("1..4 => ");
    for i in 1..4 {
        print!("{i} ");
    }
    println!();

    print!("1..=4 => ");
    for i in 1..=4 {
        print!("{i} ");
    }
    println!();

    print!("(1..=10).step_by(2) => ");
    for i in (1..=10).step_by(2) {
        print!("{i} ");
    }
    println!();

    print!("(1..=5).rev() => ");
    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!();
}

/// 5) `for` 遍历集合：直接迭代元素，或通过 `enumerate` 拿到下标。
fn demonstrate_for_over_collections() {
    let vector = vec![10, 20, 30];

    for value in &vector {
        println!("for-in-ref: value = {value}");
    }

    for (i, value) in vector.iter().enumerate() {
        println!("enumerate: i = {i}, value = {value}");
    }

    // 解构元组元素
    let pairs = [(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in pairs.iter() {
        println!("pair: num = {num}, word = {word}");
    }
}

/// 6) 带标签的 break/continue：在嵌套循环里精确跳出。
fn demonstrate_labeled_break() {
    let mut hit = None;

    // `'outer:` 是给外层循环起的**标签**。内层可以通过它精确跳出指定循环。
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x * y >= 6 {
                hit = Some((x, y));
                break 'outer; // 直接跳出外层循环，而不只是内层
            }
        }
    }

    println!("labeled break 找到的第一个 (x, y): {hit:?}");
}

pub fn run() {
    println!("-- (1) if 是表达式 --");
    demonstrate_if_as_expression();
    println!();

    println!("-- (2) loop 带返回值 --");
    demonstrate_loop_with_break_value();
    println!();

    println!("-- (3) while 条件循环 --");
    demonstrate_while();
    println!();

    println!("-- (4) for + range --");
    demonstrate_for_with_ranges();
    println!();

    println!("-- (5) for 遍历集合 --");
    demonstrate_for_over_collections();
    println!();

    println!("-- (6) 带标签的 break --");
    demonstrate_labeled_break();
    println!();
}
