// struct 只放数据还不够，真正常见的写法是再用 `impl` 给它补行为。
// 这一节把方法、关联函数、构造函数放在一个最小例子里。
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    // `new` 是最常见的构造函数名字，但它只是普通关联函数，不是语言关键字。
    fn new(owner: String, year: u32, price: u32) -> Self {
        Self {
            owner,
            year,
            fuel_level: 0.0,
            price,
        }
    }

    fn display_car_info(&self) {
        // `&self` 表示只读借用当前实例。
        println!(
            "Owner: {}, Year: {}, Fuel: {}, Price: {}",
            self.owner, self.year, self.fuel_level, self.price
        );
    }

    fn refuel(&mut self, gallons: f32) {
        // `&mut self` 表示方法内部允许修改当前实例。
        self.fuel_level += gallons;
    }

    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Self::monthly_insurance()
    }

    fn sell(self) -> Self {
        // `self` 表示把整个实例按值拿走。
        self
    }
}

pub fn run() {
    println!("== Adding Functionality To Structs ==");

    // 这里按顺序演示：构造 -> 读取 -> 修改 -> 消费自身。
    let mut my_car = Car::new(String::from("ABC"), 2010, 5_000);
    my_car.display_car_info();
    my_car.refuel(10.5);
    println!("selling price = {}", my_car.selling_price());

    let sold_car = my_car.sell();
    sold_car.display_car_info();
    println!();
}
