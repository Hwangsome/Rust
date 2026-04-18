//! `Rc<T>`：单线程引用计数——**多个所有者**共享同一份只读数据。
//!
//! Rc 做什么：
//! - 允许**多个变量同时拥有同一个值**（对 Rust 默认"单一所有者"规则的补充）
//! - 每次 `Rc::clone(&rc)` 把引用计数 +1，drop 时 -1，降到 0 释放
//! - **只读**：Rc 本身不提供可变访问（想改需要配合 `RefCell`）
//!
//! Rc ≠ Arc：
//! - Rc 只能在**单线程**用（引用计数不是原子操作，快但不安全多线程）
//! - Arc 是原子引用计数，可以跨线程。两者 API 几乎完全一样，按需替换
//!
//! 本节演示：
//! 1. `Rc::new` / `Rc::clone` / `strong_count`
//! 2. 离开作用域后计数自动下降
//! 3. 共享场景：两个"读者"各自保留自己的 Rc

use std::rc::Rc;

#[derive(Debug)]
struct Document {
    title: String,
}

pub fn run() {
    println!("== Rc Smart Pointer ==");

    println!("-- (1) 创建与 clone 计数变化 --");
    let shared = Rc::new(Document { title: "Rust patterns".into() });
    println!("创建后 strong_count = {}", Rc::strong_count(&shared));

    let reader_a = Rc::clone(&shared);
    let reader_b = Rc::clone(&shared);
    println!("两次 clone 后 strong_count = {}", Rc::strong_count(&shared));

    println!("reader_a.title = {}", reader_a.title);
    println!("reader_b.title = {}", reader_b.title);
    println!();

    println!("-- (2) 离开作用域后计数自动下降 --");
    {
        let temp = Rc::clone(&shared);
        println!("内层作用域内 strong_count = {}", Rc::strong_count(&shared));
        drop(temp); // 显式 drop，或者自然离开作用域
        println!("drop temp 后 strong_count = {}", Rc::strong_count(&shared));
    }
    println!();

    println!("-- (3) Rc 是只读 --");
    // shared.title = "X".into(); // ← 不行，Rc 没给你 &mut 访问
    println!("  想改 Rc 里的值，必须配合 RefCell<T>（见 topic_08）");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Reference Counting Smart Pointer
// -------------------------------------------

use std::rc::Rc;
enum List {
    Cons(i32, Option<Rc<List>>),
}
fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        println!("Reference count after b: {}", Rc::strong_count(&a));

        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
}

/* 
----------------------------------------------------------------------------------------------
Concept / Topic       | Explanation
----------------------|-----------------------------------------------------------------------
Rc Smart Pointer      | It enables multiple ownership in single-threaded contexts.
                      | Rc::new creates a new reference-counted value on the heap.
                      | Rc::clone creates a new owner and it does not copy the inner data.

Strong Count Behavior | Rc::strong_count returns the number of active owners.
                      | Each clone increases the strong count.
                      | When an owner goes out of scope, the count decreases automatically.
                      | The heap data is freed only when the count reaches zero.
----------------------------------------------------------------------------------------------
*/
"###;
