// 这一节把“struct 也能被模式匹配”说清楚。
// 一旦理解了这一点，函数参数里直接解构 struct 就不会显得突兀。
struct Point {
    x: i32,
    y: i32,
}

fn print_coord(Point { x, y }: Point) {
    // 这里函数参数不是简单写 `point: Point`，
    // 而是直接把 `Point` 解构成 `x` 和 `y` 两个绑定。
    println!("coords from destructured param = ({x}, {y})");
}

pub fn run() {
    println!("== Destructured Struct Parameters ==");

    // `match` 里既可以写具体值，也可以把字段绑定到变量上。
    let point = Point { x: 0, y: 7 };

    match point {
        Point { x: 0, y } => println!("point is on y-axis, y = {y}"),
        Point { x, y: 0 } => println!("point is on x-axis, x = {x}"),
        Point { x, y } => println!("point at ({x}, {y})"),
    }

    let point = Point { x: 5, y: 6 };
    print_coord(point);
    println!();
}
