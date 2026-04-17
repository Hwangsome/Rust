// 这个文件演示引用环为什么危险，以及 Weak 在哪里出场。
// 运行时要观察：children 用 Rc 表示“真正拥有子节点”，parent 用 Weak 表示“只想回看父节点，不想延长其生命周期”。
// 一旦父子都用强引用，引用计数就可能永远归零不了。
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
