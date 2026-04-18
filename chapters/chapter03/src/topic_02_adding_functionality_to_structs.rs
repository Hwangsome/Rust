//! 给 struct 加行为：`impl` 块里的方法与关联函数。
//!
//! 数据和行为分开写是 Rust 的一个重要设计：数据放在 `struct` 里，行为放在 `impl` 块里。
//! 这一节把 `impl` 里能出现的几种函数类型**都**走一遍：
//!
//! 1. **关联函数**（associated function）：没有 `self`，用 `Type::name(...)` 调用；
//!    通常扮演"构造函数"的角色，比如约定俗成的 `new`、`from_xxx`、`default`
//! 2. **`&self` 方法**：只读借用当前实例——最常见、最安全的查询接口
//! 3. **`&mut self` 方法**：可变借用当前实例，可以就地修改字段
//! 4. **`self` 方法**：按值接收，**消费**当前实例——典型用途是"转换"或 builder 终结
//! 5. **多个 `impl` 块**：同一个类型可以有多个 `impl`，编译器会合并

#[derive(Debug)]
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

// 第一个 impl 块：构造、查询、修改。
impl Car {
    /// 关联函数：没有 `self`。按 Rust 惯例命名为 `new`。
    ///
    /// 这里 `Self` 是"当前正在 impl 的类型"的别名，等价于直接写 `Car`。
    fn new(owner: String, year: u32, price: u32) -> Self {
        Self {
            owner,
            year,
            fuel_level: 0.0,
            price,
        }
    }

    /// 另一个关联函数：构造一辆"二手车"（演示同一个 `impl` 里可以有多个构造函数）。
    fn used(owner: String, year: u32, price: u32, existing_fuel: f32) -> Self {
        Self {
            owner,
            year,
            fuel_level: existing_fuel,
            price,
        }
    }

    /// 关联函数也可以返回固定常量值（没有 `self`）。
    fn monthly_insurance() -> u32 {
        123
    }

    /// `&self`：只读借用 —— 只能读字段，不能改。
    fn display(&self) {
        println!(
            "Owner: {}, Year: {}, Fuel: {:.1}, Price: {}",
            self.owner, self.year, self.fuel_level, self.price
        );
    }

    /// `&self` + 组合关联函数：计算最终售价。
    fn selling_price(&self) -> u32 {
        self.price + Self::monthly_insurance()
    }

    /// `&mut self`：独占借用 —— 可以修改字段。
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }
}

// 多个 `impl` 块完全合法；编译器会把它们视为同一类型的方法集合。
// 这里演示"消费 self"的方法族。
impl Car {
    /// `self`：按值接收 —— **消费**当前实例。
    ///
    /// 一旦调用，调用方的原变量就不能再用了（所有权转移给函数）。
    /// 典型用途：
    /// - 把一种类型"转成"另一种类型（`fn into_receipt(self) -> Receipt`）
    /// - builder 模式的终结函数（`fn build(self) -> Config`）
    fn sell(self) -> Self {
        println!("车辆 {} 已售出，给新车主重新登记。", self.owner);
        self
    }
}

pub fn run() {
    println!("== Adding Functionality To Structs ==");

    println!("-- (1) 用关联函数 new 构造 --");
    let mut my_car = Car::new(String::from("Alice"), 2010, 5_000);
    my_car.display();
    println!();

    println!("-- (2) &mut self 方法修改字段 --");
    my_car.refuel(10.5);
    my_car.refuel(5.0);
    my_car.display();
    println!();

    println!("-- (3) 关联函数被 &self 方法调用 --");
    println!("selling_price = {}", my_car.selling_price());
    println!();

    println!("-- (4) 另一个关联函数 used 构造二手车 --");
    let used = Car::used(String::from("Bob"), 2005, 3_000, 3.0);
    used.display();
    println!();

    println!("-- (5) self 方法消费实例 --");
    let sold = my_car.sell();
    // println!("{my_car:?}"); // ← 这行会触发 E0382，my_car 的所有权已被 sell() 拿走
    sold.display();
    println!();
}
