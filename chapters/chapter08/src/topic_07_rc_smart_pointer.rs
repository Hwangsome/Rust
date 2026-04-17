// 这个文件演示 Rc：当一个值需要被多个拥有者共享时，可以把“所有权”做成引用计数。
// 运行时要观察 strong_count 的变化，这能直接看出 Rc 的共享语义。
// Rc 只解决“多个拥有者”，不解决“可变修改”。
use std::rc::Rc;

pub fn run() {
    println!("== Rc Smart Pointer ==");

    let shared_title = Rc::new(String::from("Rust patterns"));
    println!(
        "strong_count after create => {}",
        Rc::strong_count(&shared_title)
    );

    let reader_a = Rc::clone(&shared_title);
    let reader_b = Rc::clone(&shared_title);

    println!("reader_a => {}", reader_a);
    println!("reader_b => {}", reader_b);
    println!(
        "strong_count after clones => {}",
        Rc::strong_count(&shared_title)
    );
    println!();
}
