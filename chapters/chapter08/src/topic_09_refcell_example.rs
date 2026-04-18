//! `Rc<RefCell<T>>` 组合：**共享所有权 + 内部可变性**。
//!
//! 两者解决的问题互补：
//! - `Rc<T>` → 让多个变量都拥有同一份数据（解决"所有权唯一"限制）
//! - `RefCell<T>` → 让不可变绑定下也能修改内部（解决"可变性对共享的要求"）
//!
//! 合在一起就是教科书级的"**多所有者 + 可变共享**"，常用于：
//! - GUI / 场景树：多个父节点指向同一个子节点，子节点状态可变
//! - 图数据结构：多个邻接表共享节点数据
//! - 观察者模式：多个观察者读写同一份状态
//!
//! 再多一层要求：**跨线程**——那要换成 `Arc<Mutex<T>>` / `Arc<RwLock<T>>`。

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct FileState {
    name: String,
    opened_by: Vec<&'static str>,
}

/// 接收 `&Rc<RefCell<FileState>>`：借用 Rc（不增加计数），再用 borrow_mut 改内部。
fn open_file(file: &Rc<RefCell<FileState>>, user: &'static str) {
    file.borrow_mut().opened_by.push(user);
}

pub fn run() {
    println!("== RefCell Example ==");

    let shared_file = Rc::new(RefCell::new(FileState {
        name: "notes.txt".into(),
        opened_by: Vec::new(),
    }));

    println!("-- (1) 多个 `客户端` 共享可变状态 --");
    // 把 Rc 传给不同模块/函数，每个都能改底层数据
    open_file(&shared_file, "Alice");
    open_file(&shared_file, "Bob");

    // strong_count 显示当前 Rc 的引用数
    println!("strong_count = {}", Rc::strong_count(&shared_file));
    let snap = shared_file.borrow();
    println!("{} opened by {:?}", snap.name, snap.opened_by);
    drop(snap);
    println!();

    println!("-- (2) 两个 Rc 克隆指向同一份数据 --");
    let view1 = Rc::clone(&shared_file);
    let view2 = Rc::clone(&shared_file);
    // view1 改，view2 也立刻看到
    view1.borrow_mut().opened_by.push("Charlie");
    println!("view2 看到 opened_by = {:?}", view2.borrow().opened_by);
    println!("strong_count = {}", Rc::strong_count(&shared_file));
    println!();

    println!("-- (3) 多线程场景：Rc<RefCell<T>> 不行，换成 Arc<Mutex<T>> --");
    println!("  Rc 不是 Send；RefCell 不是 Sync");
    println!("  多线程要用：std::sync::Arc + std::sync::Mutex / RwLock");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		RefCell Example
// -------------------------------------------
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File {
    active_user: u32,
}

#[derive(Debug)]
struct User {
    file: Rc<RefCell<File>>,
}

fn main() {
    let mut txt_file = Rc::new(RefCell::new((File { active_user: 0 })));

    let mut user_1 = User {
        file: Rc::clone(&txt_file),
    };
    user_1.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file.borrow().active_user);

    let mut user_2 = User {
        file: Rc::clone(&txt_file),
    };
    user_2.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file.borrow().active_user);
}


/* 
-----------------------------------------------------------------------------------------------------
Concept / Topic               | Explanation
------------------------------|----------------------------------------------------------------------
Immutability Limitation of Rc | Rc<T> only provides immutable access to shared data.

Combining Rc with RefCell     | RefCell<T> enables interior mutability with runtime borrow checking.
                              | Wrapping Refcell by an Rc allows mutation with shared owners.
                                
Shared State Visibility       | All Rc clones point to the same underlying heap allocation.
                              | Updates through one owner are visible to all owners.
                              | This pattern is common for shared resource management.
---------------------------------------------------------------------------------------------------- 
*/
"###;
