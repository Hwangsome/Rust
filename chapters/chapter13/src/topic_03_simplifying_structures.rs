//! 结构拆分：把大 struct 拆成几个子结构，缓解借用冲突 + 改善建模。
//!
//! Rust 的借用检查器是**字段级**敏感的：
//!
//! ```ignore
//! let (a, b) = (&state.meta, &mut state.stats);  // ✅ 两个字段不冲突
//! let (a, b) = (&state.stats, &mut state.stats); // ❌ 同一字段，E0502
//! ```
//!
//! 把"总是一起变的字段"打包到一个子 struct 里，"很少一起动的字段"拆出来，能带来两重好处：
//!
//! 1. **编译层面**：借用冲突常常自然消失——因为每次借用范围更窄
//! 2. **建模层面**：每个子 struct 有独立名字和内聚的职责，单独测试、单独传参都变容易
//!
//! 本节通过把 `DocumentState` 拆成 `Meta` + `Stats`，同时演示"读 Meta + 改 Stats"完全合法
//! （而把它们合在一个扁平 struct 里若要"同时读 title 又改 likes"会被借用检查器限制）。

#[derive(Debug)]
struct DocumentState {
    meta: Meta,
    stats: Stats,
}

#[derive(Debug)]
struct Meta {
    title: String,
}

#[derive(Debug)]
struct Stats {
    reads: u32,
    likes: u32,
}

fn read_title(meta: &Meta) -> &str {
    &meta.title
}

fn bump_likes(stats: &mut Stats) {
    stats.likes += 1;
}

fn total_engagement(stats: &Stats) -> u32 {
    stats.reads + stats.likes
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Simplifying Structures ==");

    let mut doc = DocumentState {
        meta: Meta {
            title: "Rust Notes".to_string(),
        },
        stats: Stats {
            reads: 10,
            likes: 2,
        },
    };

    let title = read_title(&doc.meta);
    bump_likes(&mut doc.stats);
    let engagement = total_engagement(&doc.stats);

    println!("title = {title}, engagement = {engagement}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//           	- Simplifying structures
// -------------------------------------------

// The problem
// struct A {
//     f1: u32,
//     f2: u32,
//     f3: u32,
// }

// fn fn1(a: &mut A) -> &u32 {
//     &a.f2
// }
// fn fn2(a: &mut A) -> u32 {
//     a.f1 + a.f3
// }

// fn fn3(a: &mut A) {
//     let x = fn1(a);
//     let y = fn2(a);
//     println!("{}", x);
// }
// --------- Problem Ends ------

// ---------- Solution --------
struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

fn fn1(b: &mut B) -> &u32 {
    &b.f2
}
fn fn2(c: &mut C) -> u32 {
    c.f1 + c.f3
}

fn fn3(a: &mut A) {
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);
    println!("{}", x);
}

fn main() {}

/* 
----------------------------------------------------------------------------------------------------
Concept / Topic        | Explanation
-----------------------|----------------------------------------------------------------------------
Borrowing Whole Struct | Borrowing a struct field mutably, resulting in borrows the entire struct.
                       | This prevents additional mutable borrows of the same struct.

Struct Decomposition   | Splitting the struct into smaller sub-structures can solve this problem.
                       | Each smaller struct can then be borrowed independently.

Borrowing Flexibility  | After decomposition, functions can borrow only the required sub-structure.
                       | The borrow checker can recognize that different fields are independent.
                       | This allows simultaneous mutable borrows of different parts of the data.

Design Benefit         | Decomposing large structures often improves program design.
                       | Smaller structs are easier to understand and manage.
-----------------------------------------------------------------------------------------------------
*/
"###;
