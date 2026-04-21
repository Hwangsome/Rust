//! Trait 必须在作用域内，才能用**方法调用语法**使用其 trait 方法（参考手册中的 *trait in scope*）。
//!
//! 以 **`std::ops::Add`** 为例：`i32 + i32` 为什么能直接写 `a + b`？
//! 因为编译器对 `+` 这类**操作符**会自行查找对应的 `Add` 实现，**不要求**你把 `Add` import 进来。
//! 但一旦改成**方法语法** `a.add(b)`（调用 trait 定义的 `fn add`），`Add` 就必须在作用域内。
//!
//! ```rust,compile_fail
//! // 没有 `use std::ops::Add;`
//! let s = 3_i32.add(5_i32); // ← E0599: no method named `add` found for type `i32`
//! // help: trait `Add` which provides `add` is implemented but not in scope;
//! //       consider importing it: `use std::ops::Add;`
//! ```
//!
//! 两种解决方式：
//!
//! 1. **`use std::ops::Add;`** 把 trait 名引入作用域，之后 `a.add(b)` 就能用。
//! 2. **完全限定语法（UFCS）** `std::ops::Add::add(a, b)`——路径里显式写了 trait，所以**不依赖 `use`**。
//!
//! 对比：`Clone::clone`、`ToString::to_string` 等之所以"不用 `use`"，是因为 `Clone` / `ToString`
//! 已经在 **prelude** 里预导入；`Add` 不在 prelude，所以需要手动引入。

pub fn run() {
    println!("== Trait must be in scope (std::ops::Add) ==");

    let a: i32 = 3;
    let b: i32 = 5;

    println!("-- (1) 操作符 `+`：编译器自行查找 Add 实现，**不需要** `use Add` --");
    let sum_operator = a + b;
    println!("  {a} + {b} = {sum_operator}");
    println!();

    println!("-- (2) 方法语法 `a.add(b)`：需要 `use std::ops::Add` --");
    // 把 `use` 限制在块作用域里，读者能看清"只有这里 Add 在作用域"。
    {
        use std::ops::Add;
        let sum_method = a.add(b);
        println!("  a.add(b) = {sum_method} （块内 `use std::ops::Add`）");
    }
    // 离开块后再写 `a.add(b)` 会 E0599：
    // let s = a.add(b);
    println!();

    println!("-- (3) 不 `use` 也行：完全限定路径（UFCS）--");
    let sum_ufcs = std::ops::Add::add(a, b);
    println!("  std::ops::Add::add(a, b) = {sum_ufcs}");

    let sum_ufcs_type = <i32 as std::ops::Add>::add(a, b);
    println!("  <i32 as std::ops::Add>::add(a, b) = {sum_ufcs_type}");
    println!();

    println!("-- (4) 结论 --");
    println!("  · 操作符 `+` / `-` / `*` / `==` 走 trait，但编译器帮你找，免 `use`");
    println!("  · trait 方法语法 `x.trait_method()` 必须 **trait in scope**，否则 E0599");
    println!("  · 写全路径 `Trait::method(x)` 可绕过，但日常仍推荐 `use Trait`");
    println!();
}
