//! 具体生命周期（concrete lifetime）：每个值**实际存活**的时间段。
//!
//! 关键事实：
//! - 值的生命周期 = 从创建到被 drop（或被 move 走）
//! - 引用的生命周期 **必须** 短于或等于所指向值的生命周期
//! - **NLL（Non-Lexical Lifetime）**：借用只活到**最后一次使用**为止，不是到块结束
//! - 如果一个引用可能"悬垂"（指向已被释放的内存），编译器就报 **E0597**
//!
//! 本节演示：
//! 1. NLL 让"借用后再 push"变得合法
//! 2. 作用域结束值被 drop，引用不能逃出作用域（悬垂防护）
//! 3. 值被 move 后，之前的借用也跟着失效

pub fn run() {
    println!("== Concrete Lifetimes ==");

    println!("-- (1) NLL：借用活到最后一次使用为止 --");
    let mut values = vec![10, 20, 30];
    let first = &values[0];
    println!("first = {first}");
    // first 在上一行之后不再被使用 → NLL 判定借用已结束 → 下面的 push 合法
    values.push(40);
    println!("values after push = {values:?}");
    println!();

    println!("-- (2) 防止悬垂引用：引用不能活得比值长 --");
    // 下面这段如果取消注释会报 E0597：
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // x 在这里被 drop，r 就变悬垂了
    // println!("{r}");
    println!("  见代码注释：编译器用借用检查直接拦截悬垂引用");
    println!();

    println!("-- (3) move 让原借用失效 --");
    let s1 = String::from("hello");
    let _r = &s1; // 只要 _r 还活着，下面就不能 move s1
    drop(_r);      // 这里显式结束 _r
    let s2 = s1;   // 现在可以 move
    println!("move 后新所有者 s2 = {s2}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//         - Concrete Lifetimes
// -------------------------------------------

fn main() {
    // Example 1:
    let i = 5;
    let j = i;
    println!("{i}");

    // Example 2:
    let str_1 = String::from("abc");
    let str_2 = str_1;
    //println!("str_1: {str_1}");

    // Example 3:
    let str_1 = String::from("abc");
    str_fn(str_1);
    //let str_2 = str_1;

    // Example 4:
    let i;
    {
        let j = 5;
        i = &j;
        println!("i: {i}");
    }

    // Example 5:
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1;
    println!("ref 1: {:?}", ref_1);
    let ref_2 = &mut vec_1;
    ref_2.push(3);
    println!("ref 2: {:?}", ref_2);
}

fn str_fn(s: String) {
    println!("s: {s}");
}

/*
----------------------------------------------------------------------------------------------------------
Concept / Topic       | Explanation
----------------------|-----------------------------------------------------------------------------------
Lifetimes             | They can classified as concrete and generic lifetimes.

Concrete Lifetime     | It is the duration for which a value exists in memory.
                      | It starts when a value is created and ends when it is dropped or moved.
                      | Ownership transfer end a value’s lifetime in a given binding.
                      | When a value is passed by ownership, the lifetime of binding ends at function call.

Dangling References   | A dangling reference points to a value that has already been dropped.
                      | Rust prevents this by ensuring references do not outlive their data.
                      | The borrow checker enforces that borrowed values live long enough.

Non-Lexical Lifetimes | They are determined by actual usage rather than scope.
                      | A reference’s lifetime ends at its last use.
                      | If &mut and & lifetimes do not overlap, borrowing rules are satisfied.
------------------------------------------------------------------------------------------------------------
*/
"###;
