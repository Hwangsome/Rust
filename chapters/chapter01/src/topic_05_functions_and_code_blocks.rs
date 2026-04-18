//! 函数与代码块：Rust 最基础的"组织和计算"单元。
//!
//! 关键建模：
//! - **函数**是一段有名字、带签名（输入/输出）的可调用代码
//! - **代码块**（`{ ... }`）本身就是一个表达式，会产出一个值
//! - **语句**以 `;` 结尾，不产出值；**表达式**没有 `;`，会产出值
//! - 函数体就是一个代码块，所以"最后一行不加分号 = 返回值"这件事就从这来的
//!
//! 重点先认得三个直觉：
//! 1. 参数必须显式写类型；返回值类型用 `-> T`；不写返回值类型等同于返回 `()`
//! 2. 尾表达式（最后一行无分号）会作为返回值
//! 3. `return` 关键字可以提前返回，但平时写代码更倾向尾表达式

// ---------- 函数：签名决定了"我能做什么" ----------

/// 没有返回值的函数——等价于返回 unit `()`。
fn announce_topic() {
    println!("announce_topic() 执行了：这种函数负责做事，不负责返回信息。");
}

/// 最小的"加法"函数：用尾表达式返回结果。
fn add(a: i32, b: i32) -> i32 {
    // 这里故意不写 `return`，依赖 Rust 的"尾表达式即返回值"规则。
    // 注意 `a + b` 后面没有分号——加上分号它就变成"语句"，函数就变成返回 ()。
    a + b
}

/// 等价写法：用 `return` 关键字显式返回。
fn add_with_return_keyword(a: i32, b: i32) -> i32 {
    return a + b;
}

/// 多个返回值：打包进元组。Rust 没有"多返回值语法"，而是返回一个 tuple。
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

/// 提前返回：`return` 让你在函数中间跳出。
fn larger_number(left: i32, right: i32) -> i32 {
    if left > right {
        return left; // 提前返回，后面的代码不会执行
    }
    right // 尾表达式：执行到这里时返回
}

/// 分号的魔力：加不加分号，函数返回值完全不同。
///
/// Rust 会严格检查"返回值类型 = 函数尾部表达式的类型"——
/// 所以把 `;` 忘了或多打了一个，通常会得到 E0308 类型不匹配。
fn semicolon_matters() -> i32 {
    let base = 10;
    base + 5 // 尾表达式，类型是 i32，和签名匹配
    // 如果这里写成 `base + 5;`（带分号），就变成语句，整个函数的返回类型变成 ()，
    // 会触发 E0308: expected `i32`, found `()`。
}

/// 接收 `&str` 的函数：只读借用一段文本。
fn print_message(s: &str) {
    // 格式化时写 `{s}` 会直接把 s 的内容格式化出来。
    println!("print_message received: {s}");
}

// ---------- 代码块：{ ... } 本身也是一个表达式 ----------

/// 演示"代码块表达式"——块里最后一行无分号，就是块的值。
fn demonstrate_block_as_expression() {
    // 整个花括号就是一个表达式，它的值赋给 `computed`。
    let computed = {
        let base = 5;
        base + 1 // 尾表达式：这就是整个块的值
    };
    println!("code block 产出的值: computed = {computed}");

    // 如果在尾表达式后加分号，块的值就变成 `()`。
    let unit_value: () = {
        let _ = 5 + 1; // 语句，结果被丢弃
    };
    println!("带分号的块返回 unit: {unit_value:?}");
}

/// 代码块另一个用途：创建**新作用域**，内部绑定不污染外部。
fn demonstrate_block_scope_isolation() {
    let outer = 10;

    let scoped_result = {
        // 这里的 `outer` shadow 了外层 `outer`，**只在块内**生效。
        let outer = outer + 5;

        // 嵌套代码块也能返回值。
        let doubled = {
            let temporary = outer * 2;
            temporary
        };

        doubled + 1
    };

    // 回到外层：外层 `outer` 仍然是 10，没有被改动。
    println!("外层 outer 仍然 = {outer}");
    println!("scoped_result = {scoped_result}");
}

pub fn run() {
    println!("-- (1) 函数签名与返回值 --");
    announce_topic();
    println!("add(2, 3) = {}", add(2, 3));
    println!("add_with_return_keyword(4, 5) = {}", add_with_return_keyword(4, 5));

    let (sum_result, product_result) = sum_and_product(3, 4);
    println!("sum_and_product(3, 4) = (sum = {sum_result}, product = {product_result})");
    println!("larger_number(10, 6) = {}", larger_number(10, 6));
    println!("semicolon_matters() = {}", semicolon_matters());
    print_message("this is my function");
    println!();

    println!("-- (2) 代码块也是表达式 --");
    demonstrate_block_as_expression();
    println!();

    println!("-- (3) 代码块创建作用域 --");
    demonstrate_block_scope_isolation();
    println!();
}
