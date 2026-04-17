// 这个文件演示“泛型”最核心的价值：把“结构相同、类型不同”的代码合并成一套模板。
// 运行后要观察的是：`Point<T, U>` 可以装不同类型，而且 `mixup` 会保留每个位置的类型信息。
// 这是后面 trait、迭代器和 Result/Option 泛型化设计的基础。
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    println!("== Generics ==");

    let integer_point = Point::new(3, 5);
    let mixed_point = integer_point.mixup(Point::new("left", 4.5));

    // 这里要观察的是：同一个 Point 模板，字段位置可以绑定不同具体类型。
    println!("mixed point => {:?}", mixed_point);
    println!();
}
