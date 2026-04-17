// 这个文件演示关联类型：trait 不只约束“必须有某个方法”，还可以约束“这个方法会产出哪类数据”。
// 运行后要观察：同一个 trait 在不同实现上，返回值类型可以不同，但每个实现内部保持一致。
// 这让 API 比“把所有类型都塞成泛型参数”更易读。
#[derive(Debug)]
struct Kmh(u32);

#[derive(Debug)]
struct Mph(u32);

trait DistanceThreeHours {
    type Distance;

    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = u32;

    fn distance_in_three_hours(&self) -> Self::Distance {
        self.0 * 3
    }
}

impl DistanceThreeHours for Mph {
    type Distance = f32;

    fn distance_in_three_hours(&self) -> Self::Distance {
        (self.0 * 3) as f32
    }
}

pub fn run() {
    println!("== Associated Types in Traits ==");

    let city_speed = Kmh(80);
    let highway_speed = Mph(65);

    println!(
        "80 km/h for 3h => {} km",
        city_speed.distance_in_three_hours()
    );
    println!(
        "65 mph for 3h => {} miles",
        highway_speed.distance_in_three_hours()
    );
    println!();
}
