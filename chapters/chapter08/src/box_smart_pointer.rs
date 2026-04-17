// 这个文件演示 Box 最经典的用途：让递归类型拥有固定大小。
// 运行时要观察：如果没有 `Box<List>`，编译器就无法知道 enum 的大小。
// Box 把“下一节节点”放到堆上，当前节点本体只保留一个指针大小。
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn len(&self) -> usize {
        match self {
            Self::Cons(_, next) => 1 + next.len(),
            Self::Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            Self::Cons(value, next) => *value + next.sum(),
            Self::Nil => 0,
        }
    }
}

pub fn run() {
    println!("== Box Smart Pointer ==");

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!(
        "recursive list => {:?}, len = {}, sum = {}",
        list,
        list.len(),
        list.sum()
    );
    println!();
}
