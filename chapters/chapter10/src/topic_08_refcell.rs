//! `RefCell<T>`：**内部可变性**——借用规则仍在，只是检查点从**编译期**推迟到**运行期**。
//!
//! 为什么需要？
//! - Rust 默认要求"可变访问必须独占"是在**编译期**检查的——有时这对某些合法场景太严
//! - `RefCell` 把这个规则搬到运行期：仍然强制"同时只有一个 &mut"，但是**运行时**才检查
//! - 违规不再是编译失败，而是 **panic**（"already borrowed: BorrowMutError"）
//!
//! API：
//! - `.borrow()` → `Ref<T>`（只读借用，可以有多个）
//! - `.borrow_mut()` → `RefMut<T>`（独占可变借用，只能有一个）
//! - 借用在 guard 离开作用域时自动释放
//!
//! 适用场景：
//! - 外部接口要求 `&self`，但内部逻辑需要改状态（比如缓存、计数器、观察者）
//! - 常与 `Rc` 组合成 `Rc<RefCell<T>>` 实现"多所有者 + 可变共享"（见下一节）
//!
//! 多线程场景要换成 `Mutex<T>` / `RwLock<T>`。

use std::cell::RefCell;

/// 典型场景：外部是 `&self`，内部需要修改缓存。
struct LazyCache {
    computed: RefCell<Option<u64>>,
}

impl LazyCache {
    fn new() -> Self {
        Self { computed: RefCell::new(None) }
    }

    /// 注意签名是 `&self`（不是 `&mut self`）——这就是"内部可变性"。
    fn get_or_compute(&self, input: u64) -> u64 {
        let mut guard = self.computed.borrow_mut();
        if let Some(cached) = *guard {
            println!("  [cache] 命中: {cached}");
            cached
        } else {
            let v = input * input;
            *guard = Some(v);
            println!("  [cache] 计算并缓存: {v}");
            v
        }
    }
}

pub fn run() {
    println!("== RefCell ==");

    println!("-- (1) 在不可变绑定下修改内部 Vec --");
    let values = RefCell::new(vec![1, 2, 3]);
    // `values` 本身没有 `mut`，但它内部的数据可以通过 borrow_mut 修改。
    {
        let mut borrowed = values.borrow_mut();
        borrowed.push(4);
    }
    println!("values = {:?}", values.borrow());
    println!();

    println!("-- (2) 多个 .borrow() 可以并存 --");
    let a = values.borrow();
    let b = values.borrow();
    println!("a = {:?}, b = {:?}", *a, *b);
    drop(a); drop(b);
    println!();

    println!("-- (3) borrow_mut 与其他借用冲突时 panic --");
    // let _a = values.borrow();
    // let _b = values.borrow_mut(); // ← 会运行时 panic
    println!("  （见代码注释：违规会运行时 panic，不是编译错误）");
    println!();

    println!("-- (4) 真实场景: 带缓存的 LazyCache --");
    let cache = LazyCache::new();
    // 注意：方法签名是 &self，完全看不出它"改了什么"——但内部修改了 computed
    let _ = cache.get_or_compute(7);
    let _ = cache.get_or_compute(99); // 第二次走缓存
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		RefCell Smart Pointer
// -------------------------------------------

use std::{cell::RefCell, rc::Rc};

fn main() {
    // Example 1: Borrowing rules at run time
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("{} {} ", x1, x2);

    let a = RefCell::new(10);

    //{ // add the scope later on
    let b = a.borrow();
    let c = a.borrow();
    //}

    drop(b); // add later on, remove after adding the scope above
    drop(c); // add later on
    let d = a.borrow_mut();
    // drop(d) // add later on
    //println!("{} {}", b, c); // later on delete this
    //println!("Value of a is : {:?}", a); // add later on


    // Example 2: Interior mutability
    // let x = 32;
    // let x1 = &mut x;

    let a = RefCell::new(10);
    //let c = *a; // add later on
    let mut b = a.borrow_mut();
    *b = 15;

    drop(b); // add later on
    println!("{:?}", a);

    // Example 3 
    let a = Rc::new(RefCell::new(String::from("c++")));
    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("rust");
    println!("{:?}", a);
}

/* 
--------------------------------------------------------------------------------------------------
Concept / Topic           | Explanation
--------------------------|-----------------------------------------------------------------------
RefCell Smart Pointer     | It enforces borrowing rules at runtime instead of compile time.                       
                          | The borrow() creates immutable runtime borrows.
                          | borrow_mut() creates a mutable runtime borrow.
                          | Simultaneous mutable and immutable borrows cause a panic.
                          | RefCell borrows are tied to scope and do not use non-lexical lifetimes.

Interior Mutability       | RefCell allows mutation through an immutable binding.
                          | The outer variable can be immutable while the inner value changes.
                          | This is known as interior mutability.

Explicit Drop Behavior    | Runtime borrows remain active until they are dropped.
                          | Calling drop() or ending a scope releases the borrow.
                          | Mutable and immutable borrows must not overlap at runtime.

Rc<RefCell<T>> Combination | Rc<T> enables multiple ownership.
                           | RefCell<T> enables interior mutability.
                           | Rc<RefCell<T>> allows multiple owners to mutate shared data.
                           | Commonly used in graph structures and doubly linked lists.
--------------------------------------------------------------------------------------------------

 */
"###;
