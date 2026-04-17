// 这个文件把 Rc 和 RefCell 组合起来，演示“共享所有权 + 运行时可变性”。
// 运行时要观察：两个不同调用者都在修改同一个底层状态。
// 这类组合常见于 GUI 树、图结构和需要共享可变状态的教学示例。
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct FileState {
    name: String,
    opened_by: Vec<&'static str>,
}

fn open_file(file: &Rc<RefCell<FileState>>, user: &'static str) {
    file.borrow_mut().opened_by.push(user);
}

pub fn run() {
    println!("== RefCell Example ==");

    let shared_file = Rc::new(RefCell::new(FileState {
        name: "notes.txt".to_string(),
        opened_by: Vec::new(),
    }));

    open_file(&shared_file, "Alice");
    open_file(&shared_file, "Bob");

    let snapshot = shared_file.borrow();
    println!("{} opened by {:?}", snapshot.name, snapshot.opened_by);
    println!();
}
