//! 引用环（reference cycle）与 `Weak<T>` 的作用。
//!
//! Rc 是**强引用**计数：每个 `Rc<T>` 都"拥有"底层值，只有 strong_count 归 0 才释放。
//!
//! 问题：如果两个节点**互相持有 Rc**，它们的 strong_count 永远不会归 0——即使外部已经没人用了：
//!
//! ```text
//! A.next ──Rc──→ B
//! B.prev ──Rc──→ A       ← 两边都强引用 → 循环 → 都释放不了 → 内存泄漏
//! ```
//!
//! 解决：一侧用 **`Weak<T>`**：
//! - Weak **不增加** strong_count，只增加 weak_count
//! - Weak 不保证底层数据仍活着；想访问要用 `.upgrade() -> Option<Rc<T>>`
//! - 典型场景：树里 `children: Vec<Rc<Node>>`（强，父拥有子）；`parent: Weak<Node>`（弱，子不能让父多活）
//!
//! 本节构造一个 parent/child 的最小例子，打印 strong/weak 计数变化。

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Reference Cycles ==");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "leaf parent upgrade => {:?}",
        leaf.parent.borrow().upgrade().map(|node| node.value)
    );
    println!(
        "branch children => {:?}",
        branch
            .children
            .borrow()
            .iter()
            .map(|node| node.value)
            .collect::<Vec<_>>()
    );
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"

// -------------------------------------------
// 		Reference Cycles 
// -------------------------------------------
/* 
use std::cell::RefCell; 
use std::rc::{Rc, Weak}; 
#[derive(Debug)] 
struct Node {
    next: Option<Weak<RefCell<Node>>>, 
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
fn main() {    
    let a = Rc::new(RefCell::new(Node {next: None} )); 
    println!("a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 

    let b = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&a))})); 
    println!("B is created: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));  
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node {next: Some(Rc::downgrade(&b))})); 

    (*a).borrow_mut().next = Some(Rc::downgrade(&c)); 

    println!("After creating cycle: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b)); 
    println!("c strong count: {:?}, c weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c)); 

    println!("a {:?}", a);


}

*/ 


use std::borrow::Borrow; 
use std::rc::{Rc, Weak}; 
use std::cell::{RefCell, Ref}; 

#[derive(Debug)]
struct Node {
    value: i32, 
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() { 
    let leaf  = Rc::new(Node {
        value: 3, 
        parent: RefCell::new(Weak::new()), 
        children: RefCell::new(vec![]),
    }); 

    let branch = Rc::new(Node {
        value: 5, 
        parent: RefCell::new(Weak::new()), 
        children: RefCell::new(vec![Rc::clone(&leaf)]), 
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

}
"###;
