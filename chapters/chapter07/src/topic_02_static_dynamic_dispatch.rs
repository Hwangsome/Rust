//! 静态分派与动态分派：编译期确定调用目标 vs 运行时经 vtable 分派。
//!
//! **动态分派**：在**运行时**根据值的实际类型，通过**虚表（vtable）**解析并调用对应方法实现。
//!
//! **静态分派**：在**编译期**就确定「具体类型」与「要调用的那份函数代码」，通常通过**单态化**（为每个具体类型各生成一份实例）实现；调用点无运行时按类型查表。
//!
//! 例如：`Vec<Hotel>` 里元素类型固定为 `Hotel`，调用 `description` 时编译器知道只会落到 `Hotel` 的实现；而 `Vec<&dyn Accommodation>` 中每个元素的具体类型可能不同，**同一处**方法调用在运行时才经 `dyn` 分派。
//!
//! 异构具体类型不能 `vec![hotel, airbnb, …]`；局部变量也不能写 `Vec<impl Accommodation>`（`impl Trait` 不用于绑定类型）；异构统一接口时常用 `Vec<&dyn Trait>` / `Vec<Box<dyn Trait>>`（详见 `topic_06_trait_objects`）。
//!
//! 住宿类型来自 `topic_01_traits`：`Hotel` / `Airbnb` / `Hostel`。

use crate::topic_01_traits::{Accommodation, Airbnb, Hostel, Hotel};

/// 对照「静态：同质集合」与「动态：`dyn` 异构集合」。
fn demo_static_dynamic_dispatch_accommodation_zh() {
    let hotel = Hotel::new(String::from("Hotel"));
    let airbnb = Airbnb::new(String::from("Airbnb"));
    let hostel = Hostel::new(1001_u32, String::from("Demo Hostel"));

    // 问题：如何在一个 Vec 里同时放 Hotel / Airbnb / Hostel？
    // `let stays = vec![hotel, airbnb, hostel];` → 类型不一致（expected Hotel, found Airbnb）
    //
    // `let stays: Vec<impl Accommodation> = vec![...];` → 局部变量类型不允许 `impl Trait`

    // 【静态】同质：只能放同一种具体类型；编译期即知每个元素是 Hotel。
    let hotels_only: Vec<Hotel> = vec![Hotel::new(String::from("H1")), Hotel::new(String::from("H2"))];
    println!("  [静态] Vec<Hotel>：{} 条，元素类型在编译期固定", hotels_only.len());
    println!("    例：第一项 description -> {}", hotels_only[0].description());

    // 【动态】异构：用 trait object 引用统一「实现了 Accommodation 的不同类型」。
    let stays: Vec<&dyn Accommodation> = vec![&hotel, &airbnb, &hostel];
    println!("  [动态] Vec<&dyn Accommodation>：{} 条异构住宿（运行时分派）", stays.len());
    for (i, stay) in stays.iter().enumerate() {
        println!("    {}. {}", i + 1, stay.search_listing_line());
    }
}

pub fn run() {
    println!("== Static vs Dynamic Dispatch ==");

    println!("-- (1) 同质 Vec（静态）vs 异构 Vec<&dyn Trait>（动态）--");
    demo_static_dynamic_dispatch_accommodation_zh();
    println!();
}
