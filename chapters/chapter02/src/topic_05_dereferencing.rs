//! 解引用：在“引用”和“底层值”之间显式切换。
//!
//! 这一节要建立五个直觉：
//!
//! 1) 引用本身是一个值，它“指向另一个值”
//! 2) `*r` 读取：从引用回到底层值
//! 3) `*r = ...` 写入：通过可变引用修改底层值
//! 4) 打印和方法调用场景下，Rust 会**自动解引用**，看起来像没写 `*`
//! 5) 比较两个引用和比较它们指向的值是两回事：`r1 == r2` 看值；`std::ptr::eq` 看地址
//!
//! 为什么这节短但重要？因为后面的 `Box<T>`、`Rc<T>`、`Deref` trait、方法调用的自动解引用，
//! 全都建立在“引用 vs 底层值”这条边界上。

/// 1) 基础：`*reference` 把引用“穿透一层”，拿到底层的值。
fn demonstrate_basic_deref() {
    println!("-- (1) 基础解引用 --");

    let value: i32 = 42;
    let reference = &value;

    // 看起来两行都打印了 42，但它们在类型层面完全不同：
    // - `reference` 的类型是 `&i32`
    // - `*reference` 的类型是 `i32`
    //
    // `println!` 的 `{}` 帮我们隐藏了这个区别，后面的运算会把它暴露出来。
    println!("reference (类型 &i32) = {reference}");
    println!("*reference (类型 i32) = {}", *reference);
}

/// 2) 通过可变引用“写回”底层值：`*r = new_value`。
///
/// 这里 `*` 出现在赋值左侧：表示“修改引用所指向的那个值”，
/// 而不是“把引用这个变量本身换成新值”。
fn demonstrate_write_through_mutable_reference() {
    println!("-- (2) 通过 &mut 写回底层值 --");

    let mut counter: i32 = 10;
    let r = &mut counter;

    *r += 1;
    *r *= 2;

    // 这里 `r` 已经不再被使用，NLL 判定可变借用结束，
    // 于是下面可以用不可变借用继续打印 `counter`。
    println!("counter after *r += 1; *r *= 2; => {counter}");
}

/// 3) 用解引用做运算：这是 `*` 的常见现场。
///
/// 如果你忘了写 `*`，写成 `r + 1`，编译器会抱怨：
/// `cannot add {integer} to &{integer}`。
/// 因为加法没有为“`&i32 + i32`”这种组合定义。
fn demonstrate_deref_in_arithmetic() {
    println!("-- (3) 解引用参与算术 --");

    let x = 7;
    let r = &x;

    let plus_one = *r + 1;
    let doubled = *r * 2;

    println!("*r + 1 = {plus_one}, *r * 2 = {doubled}");
    println!("提示：`*` 前缀和中缀乘法 `a * b` 同符号，但位置不同、语义不同。");
}

/// 4) 自动解引用：方法调用和格式化打印会替你补上 `*`。
///
/// 这是 Rust “用起来像有 GC”的一个重要原因——很多场景下你不需要手写 `*`。
/// 但要记住：**底层机制没有改变**，只是编译器替你生成了解引用。
fn demonstrate_auto_deref_on_method_call() {
    println!("-- (4) 方法调用时的自动解引用 --");

    let s = String::from("hello");
    let r = &s;

    // 以下两行等价：编译器会把 `r.len()` 补写成 `(*r).len()`。
    let len_via_auto = r.len();
    let len_via_manual = (*r).len();

    println!("r.len() = {len_via_auto}, (*r).len() = {len_via_manual}");

    // 另一种常见自动解引用：`&String` 被当作 `&str` 使用（deref 强转）。
    // 这也是上一节 `print_len(&String)` 能直接工作的原因。
    let as_str: &str = r; // &String -> &str
    println!("deref 强转 &String -> &str: {as_str:?}");
}

/// 5) 值相等 vs 地址相等：两个引用指向不同内存，但值相等时 `==` 仍返回 true。
///
/// 这一节想澄清一个常见误区：`==` 对引用是做**值比较**，不是地址比较。
/// 想比较“是否指向同一块内存”，用 `std::ptr::eq`。
fn demonstrate_value_vs_address_equality() {
    println!("-- (5) 值相等 vs 地址相等 --");

    let a = String::from("same text");
    let b = String::from("same text");

    let ra = &a;
    let rb = &b;

    // 默认 `ra == rb` 是比较“它们各自指向的字符串内容”，不是比较地址。
    let value_equal = ra == rb;
    let address_equal = std::ptr::eq(ra, rb);

    println!("ra == rb (值相等) => {value_equal}");
    println!("std::ptr::eq(ra, rb) (地址相等) => {address_equal}");
}

pub fn run() {
    println!("== Dereferencing ==");

    demonstrate_basic_deref();
    println!();

    demonstrate_write_through_mutable_reference();
    println!();

    demonstrate_deref_in_arithmetic();
    println!();

    demonstrate_auto_deref_on_method_call();
    println!();

    demonstrate_value_vs_address_equality();
    println!();
}
