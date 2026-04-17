// 这个文件演示 RefCell：借用规则仍然存在，但检查点从编译期延后到了运行期。
// 运行时要观察：我们能在不可变绑定下修改内部数据，这就是内部可变性。
// 同时也要注意，RefCell 不是放松规则，而是把违规从“编译错误”变成“运行时 panic”。
use std::cell::RefCell;

pub fn run() {
    println!("== RefCell ==");

    let values = RefCell::new(vec![1, 2, 3]);

    {
        let mut borrowed = values.borrow_mut();
        borrowed.push(4);
    }

    // 前面的可变借用块结束后，这里才能再次借用。
    println!("after interior mutation => {:?}", values.borrow());
    println!();
}
