//! 零大小类型 (ZST) 之二：**Unit struct** 与 **marker struct**。
//!
//! Unit struct 长这样：
//!
//! ```ignore
//! struct Electric;
//! struct Gas;
//! ```
//!
//! 它们**大小为 0**，但**类型不同**。这让它们在**类型系统层面**做标签非常合适：
//!
//! - 类型状态机：`Car<New>` / `Car<Used>` 是不同类型，可以用 impl 限定只有 `Car<New>` 有 `deliver()` 方法
//! - unit 作为 trait 的 receiver：`impl Shape for Electric` —— 不需要任何字段就可以实现一组 trait
//! - 与 `PhantomData` 组合可以做更精细的"类型身份"（见下一节）

use std::mem::size_of;

#[derive(Debug)]
struct Electric;
#[derive(Debug)]
struct Gas;

trait Vehicle {
    fn kind(&self) -> &'static str;
}

impl Vehicle for Electric { fn kind(&self) -> &'static str { "⚡ electric" } }
impl Vehicle for Gas      { fn kind(&self) -> &'static str { "⛽ gas" } }

/// 类型状态机示例：`Car<State>` 是两个不同类型。
struct Car<State> {
    vin: u32,
    _state: std::marker::PhantomData<State>,
}

struct New;
struct Delivered;

impl Car<New> {
    fn new(vin: u32) -> Self {
        Self { vin, _state: std::marker::PhantomData }
    }

    /// 只有 Car<New> 才有 deliver 方法。调用后类型变成 Car<Delivered>。
    fn deliver(self) -> Car<Delivered> {
        Car { vin: self.vin, _state: std::marker::PhantomData }
    }
}

impl Car<Delivered> {
    fn owner_id(&self) -> u32 {
        self.vin * 100
    }
}

pub fn run() {
    println!("== ZST: Unit Structs ==");

    println!("-- (1) Unit struct 大小为 0 --");
    println!("  size_of::<Electric>() = {}", size_of::<Electric>());
    println!("  size_of::<Gas>()      = {}", size_of::<Gas>());
    println!();

    println!("-- (2) Unit struct 承载 trait --");
    let e = Electric;
    let g = Gas;
    println!("  {}", e.kind());
    println!("  {}", g.kind());
    println!();

    println!("-- (3) 类型状态机：Car<New> vs Car<Delivered> --");
    let new_car: Car<New> = Car::new(42);
    // new_car.owner_id(); // ← 编译错误：Car<New> 没有 owner_id
    let delivered = new_car.deliver();
    // delivered.deliver(); // ← 编译错误：Car<Delivered> 没有 deliver
    println!("  owner_id = {}", delivered.owner_id());
    println!();

    println!("类型状态机在编译期消灭了很多‘这个对象当前处于什么状态’的运行期分支");
    println!();
}
