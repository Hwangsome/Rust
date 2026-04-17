// enum 适合表达“一个值只能处于若干状态之一”。
// 这里用交通灯做例子，是因为状态之间天然互斥，便于初学者理解。
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn run() {
    println!("== Enums ==");

    // 当前值只能是某一个分支，而不是三个分支同时存在。
    let light = TrafficLight::Green;

    // `match` 会要求你显式处理每个分支，这也是 enum 非常适合建模状态的原因之一。
    match light {
        TrafficLight::Red => println!("当前状态: 停止"),
        TrafficLight::Yellow => println!("当前状态: 准备"),
        TrafficLight::Green => println!("当前状态: 通行"),
    }

    // 再额外构造两个值，避免读者误以为 enum 只能“声明一个当前值”。
    let _red = TrafficLight::Red;
    let _yellow = TrafficLight::Yellow;
    println!();
}
