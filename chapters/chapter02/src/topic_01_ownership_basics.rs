//! 所有权基础：Copy / move / clone / 借用（只读）/ 作用域 / shadowing
//!
//! 这一节要回答初学者最容易卡住的一组问题：
//!
//! - `let s2 = s1` 之后，为什么有的类型可以继续用 `s1`，有的不行？
//! - “值被释放”是什么时候发生的？
//! - `let s = s;`（同名 shadowing）是不是 move？
//! - 字符串字面量 `"hi"` 和 `String::from("hi")` 在所有权层面有什么区别？
//!
//! 运行后建议按这个顺序观察：
//! 1. Copy 类型：赋值之后，原值仍可用。
//! 2. 堆分配类型（`String`、`Vec<T>`）：赋值之后默认发生 move，原绑定失效。
//! 3. 想得到“两份独立数据”——用 `clone`。
//! 4. 只想多一个只读视角——用 `&T` 借用。
//! 5. 作用域结束时，值会被 drop（释放）。
//! 6. Shadowing 和 move 的区别。
//! 7. 字符串字面量 `&'static str` 为什么不会 move。
//!
//! 和下一节的关系：本节只在变量赋值层面讨论，
//! 下一节 `topic_02_ownership_in_functions` 会把同样的规则搬到函数调用边界上。

/// 1) `Copy` 类型：赋值按位复制栈上的数据，原变量不受影响。
///
/// 注意：`Copy` 不是“编译器的恩赐”，而是一类标记——
/// - 所有整数、浮点、`bool`、`char` 都是 `Copy`
/// - 元组 `(i32, bool)` 也是 `Copy`（成员都 `Copy` 时）
/// - `String`、`Vec<T>`、`Box<T>` 这类**持有堆资源**的类型都不是 `Copy`
fn demonstrate_copy_types() {
    println!("-- (1) i32 等 Copy 类型：赋值不是 move --");

    let x: i32 = 15;
    let y = x;

    // 这里要观察的是：`x` 和 `y` 同时有效。
    // 因为 `i32` 实现了 `Copy`，`let y = x` 生成了“两份相等的 i32”。
    println!("Copy 类型赋值后原值仍可用: x = {x}, y = {y}");

    // 元组若所有成员都是 Copy，整个元组也是 Copy。
    let pair: (i32, bool) = (7, true);
    let pair_copy = pair;
    println!("Copy 元组赋值后原值仍可用: pair = {pair:?}, pair_copy = {pair_copy:?}");
}

/// 2) `String` 的默认赋值是 move：只有新所有者可用。
///
/// 这是问题 “为什么 `s1 = String::from("hello"); let s2 = s1; println!("{s1}")` 会报 E0382”
/// 的最小复刻版本。
fn demonstrate_string_move() {
    println!("-- (2) String 的默认赋值是 move --");

    let s1 = String::from("hello");
    let s2 = s1;

    // 这里要观察的是：`s2` 拥有堆上的 `"hello"`，`s1` 这个绑定已经被编译器标记为失效。
    println!("move 后仍可用的是新所有者: s2 = {s2}");

    // 若取消下面这行的注释，会触发 E0382（典型提示：
    // `use of moved value: s1` 或 `borrow of moved value: s1`）：
    //
    // 含义是：`s1` 所持有的堆缓冲区的“所有权记号”已经交给 `s2` 了。
    // Rust 为了避免同一份堆内存被两个变量误当作各自负责的资源（double free），
    // 选择让旧绑定直接失效，而不是做一次隐式浅拷贝。
    //
    // println!("s1 = {s1}");
}

/// 3) `Vec<T>` 的赋值也是 move——和 `String` 是同一条规则。
///
/// 这里要打破一个常见误区：不要觉得只有 `String` 才会 move。
/// 凡是“自己持有堆上资源”的类型，默认都走 move 语义。
fn demonstrate_vec_move() {
    println!("-- (3) Vec<T> 的赋值也是 move --");

    let v1 = vec![1, 2, 3];
    let v2 = v1;

    println!("move 后仍可用的是新所有者: v2 = {v2:?}");

    // 取消下面这行注释同样会报 E0382：
    // println!("v1 = {v1:?}");
}

/// 4) `clone`：显式深拷贝，得到两个彼此独立的所有者。
///
/// 这里要强调：`clone` 不是“免费的编译通过按钮”——
/// 它通常会做一次堆分配和内容复制，成本不小。
fn demonstrate_clone_for_heap_types() {
    println!("-- (4) 需要两份独立数据：用 clone --");

    let s1 = String::from("world");
    let s2 = s1.clone();

    // 这里要观察的是：s1 和 s2 是两份完全独立的 String，修改其中一个不会影响另一个。
    let mut s2 = s2;
    s2.push_str("!");
    println!("clone 后两个 String 互不影响: s1 = {s1}, s2 = {s2}");
}

/// 5) 只读借用 `&T`：不转移所有权，也能让另一个名字“看到同一份数据”。
///
/// 这是“避免 clone”最常见的办法：当你只想读的时候，就借一下，别真的复制。
fn demonstrate_shared_immutable_borrow() {
    println!("-- (5) 只想共享只读视图：用 &T --");

    let greeting = String::from("hello");
    let view1 = &greeting;
    let view2 = &greeting;

    // 这里要观察的是：`greeting` 仍是唯一所有者；`view1`、`view2` 只是借用引用。
    // 借用不会转移所有权，也不会复制数据，只是多了一个“只读入口”。
    println!("借用不改变所有者: greeting = {greeting}, view1 = {view1}, view2 = {view2}");
}

/// 6) 作用域与 drop：值在其所有者离开作用域时被释放。
///
/// Rust 没有 GC，资源回收靠“作用域 + 所有者”这一组静态规则。
/// 这个例子通过一对大括号构造一个内部作用域，演示：
/// - 在内部作用域里创建的 `String`，会在 `}` 时被自动释放
/// - 所以离开这个作用域后，就再也没法访问它
fn demonstrate_scope_and_drop() {
    println!("-- (6) 作用域结束时值会被 drop --");

    {
        let inner = String::from("temp");
        println!("进入内部作用域，inner = {inner}");
        // 这一行之后，内部作用域结束，`inner` 会被释放。
        // Rust 不需要你手动写 free / delete，编译期就能知道什么时候释放。
    }

    // 此时如果尝试使用 `inner`，会报“cannot find value `inner` in this scope”，
    // 因为作用域内的绑定名也一起消失了。
    println!("离开内部作用域后，inner 已经不存在了。");
}

/// 7) Shadowing（遮蔽）不是 move，而是“用同名新变量覆盖旧绑定”。
///
/// 这一节想明确区分：
/// - `let s = s;`（左侧没有新信息，只是重绑定）——对 `String` 来说这就是 move
/// - `let s = s.to_uppercase();`（左侧是新值）——创建新 `String`，旧 `String` 被 drop
/// - `let x = 5; let x = x + 1;`——对 Copy 类型而言是 shadowing，不是 move
fn demonstrate_shadowing_vs_move() {
    println!("-- (7) Shadowing 与 move 的对比 --");

    // 7a. Copy 类型的 shadowing：只是“换一个同名绑定”，类型甚至可以改变。
    let x = 5;
    let x = x + 1; // shadowing：新的 x 遮蔽旧的 x
    let x = format!("x-as-string-{x}"); // 类型从 i32 变成 String，也合法
    println!("Copy 类型 shadowing 可以连类型都换掉: x = {x}");

    // 7b. 持堆类型的 shadowing：旧值在新绑定建立后会被 drop。
    let s = String::from("lower");
    let s = s.to_uppercase(); // 旧 s 被消费（move 进 to_uppercase），返回新 String 覆盖同名
    println!("String 的 shadowing: s = {s}");
}

/// 8) 字符串字面量 `&'static str` 为什么不会 move。
///
/// 这里想解决一个新手常见疑惑：
/// - `String::from("hi")` 得到的是 `String`，会 move
/// - 但 `"hi"` 本身是 `&str`（准确说是 `&'static str`），是“借用”
///
/// 借用（引用）默认实现 `Copy`，赋值时按位复制“这个引用本身”。
/// 底层的字符串数据存储在只读段里，永远不需要谁“负责释放”，所以不需要 move。
fn demonstrate_str_literal_is_not_string() {
    println!("-- (8) &str 字面量不会 move --");

    let literal: &str = "hi"; // &'static str
    let another = literal; // 复制的是引用本身，不是底层字节
    println!("两个 &str 都可用: literal = {literal}, another = {another}");

    // 对比：同样的字面量放进 String::from 得到的是“拥有型”的 String，行为就完全不同了。
    let owned = String::from(literal);
    let moved = owned;
    println!("String 版本 move 后新所有者: moved = {moved}");
    // println!("owned = {owned}"); // ← 如果取消注释，会触发 E0382
}

pub fn run() {
    println!("== Ownership Basics ==");

    // 顺序经过精心设计：先建立直觉（Copy），再看反直觉（move），
    // 然后给出两条常见解法（clone / &T），再补上作用域与 shadowing 这两条背景知识。
    demonstrate_copy_types();
    println!();

    demonstrate_string_move();
    println!();

    demonstrate_vec_move();
    println!();

    demonstrate_clone_for_heap_types();
    println!();

    demonstrate_shared_immutable_borrow();
    println!();

    demonstrate_scope_and_drop();
    println!();

    demonstrate_shadowing_vs_move();
    println!();

    demonstrate_str_literal_is_not_string();
    println!();
}
