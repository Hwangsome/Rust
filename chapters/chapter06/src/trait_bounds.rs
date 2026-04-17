// 这个文件把“trait 是能力”继续推进到“trait bound 是函数的入场条件”。
// 运行后要观察：只有实现了指定 trait 的类型，才能进入泛型函数。
// 这让泛型不是“什么都能收”，而是“在抽象里仍保留明确约束”。
use std::fmt::Debug;

trait Shape {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn shape_summary<T>(shape: &T)
where
    T: Shape + Debug,
{
    println!("shape summary => {:?}, area = {}", shape, shape.area());
}

fn build_square(side: u32) -> impl Shape {
    struct Square {
        side: u32,
    }

    impl Shape for Square {
        fn area(&self) -> u32 {
            self.side * self.side
        }
    }

    Square { side }
}

pub fn run() {
    println!("== Trait Bounds ==");

    let rectangle = Rectangle {
        width: 8,
        height: 3,
    };
    shape_summary(&rectangle);

    let hidden_square = build_square(4);
    println!("impl Shape return => area = {}", hidden_square.area());
    println!();
}
