// 这个文件演示 super trait：一个 trait 可以要求“想实现我，先实现别的 trait”。
// 运行后要观察：`ShapeCard` 不需要重复声明所有方法，因为它复用了上层 trait 的能力。
// 这常用于把多个基础能力组合成一个更高层的接口。
trait Draw {
    fn draw(&self) -> String;
}

trait Named {
    fn name(&self) -> &'static str;
}

trait ShapeCard: Draw + Named {
    fn summary(&self) -> String {
        format!("{} => {}", self.name(), self.draw())
    }
}

struct CircleCard;

impl Draw for CircleCard {
    fn draw(&self) -> String {
        "draw circle".to_string()
    }
}

impl Named for CircleCard {
    fn name(&self) -> &'static str {
        "CircleCard"
    }
}

impl ShapeCard for CircleCard {}

pub fn run() {
    println!("== Super Traits ==");

    let card = CircleCard;
    println!("{}", card.summary());
    println!();
}
