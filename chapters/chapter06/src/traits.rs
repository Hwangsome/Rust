// 这个文件演示 trait 的核心职责：描述“一个类型承诺提供哪些能力”。
// 运行时要观察：`Rectangle` 和 `Square` 的具体实现不同，但都能以 `Shape` 身份被统一调用。
// 这和泛型组合后，就能写出“面向行为而不是面向具体类型”的函数。
trait Shape {
    fn area(&self) -> u32;

    fn perimeter(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    side: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        self.side * 4
    }
}

fn print_shape_details(name: &str, shape: &impl Shape) {
    println!(
        "{name} => area = {}, perimeter = {}",
        shape.area(),
        shape.perimeter()
    );
}

pub fn run() {
    println!("== Traits ==");

    let rectangle = Rectangle {
        width: 6,
        height: 4,
    };
    let square = Square { side: 5 };

    print_shape_details("rectangle", &rectangle);
    print_shape_details("square", &square);
    println!();
}
