//! 第 8 章：Project Structure
//!
//! 演示**文件式模块**（file-based module pattern）：
//! 将 chapter07 里定义在单一文件中的 accommodation 业务拆分到
//! `accommodation/` 子目录，通过 `mod.rs` + 独立结构体文件实现模块化。

mod accommodation;
mod display;
mod lab;

use accommodation::{Accommodation, Airbnb, Hostel, Hotel, StayPolicy};

fn main() {
    println!("=== Chapter 08 · Project Structure ===\n");

    // 通过 mod.rs 的 pub use 直接引用各类型
    let mut hotel = Hotel::new("Grand Palace");
    let mut airbnb = Airbnb::new("Cozy Downtown Flat");
    let mut hostel: Hostel<u32> = Hostel::new(1001_u32, "Backpackers Inn");

    println!("-- 住宿列表 --");
    println!("  {}", hotel.search_listing_line());
    println!("  {}", airbnb.search_listing_line());
    println!("  {}", hostel.search_listing_line());
    println!("  Hostel payload (ID): {}", hostel.payload());
    println!();

    println!("-- 预订 --");
    match hotel.book("Alice", 3) {
        Ok(()) => println!("  Hotel booking OK"),
        Err(e) => println!("  Hotel booking ERR: {e}"),
    }
    match airbnb.book("Bob", 10) {
        Ok(()) => println!("  Airbnb booking OK"),
        Err(e) => println!("  Airbnb booking ERR: {e}"),
    }
    match hostel.book("Carol", 7) {
        Ok(()) => println!("  Hostel booking OK"),
        Err(e) => println!("  Hostel booking ERR: {e}"),
    }
    // 超出限制
    match hostel.book("Dave", 10) {
        Ok(()) => println!("  Hostel booking OK"),
        Err(e) => println!("  Hostel booking ERR: {e}"),
    }
    println!();

    println!("-- 最大晚数政策 --");
    println!("  Hotel  : {} nights", hotel.max_nights_per_booking());
    println!("  Airbnb : {} nights", airbnb.max_nights_per_booking());
    println!("  Hostel : {} nights", hostel.max_nights_per_booking());

    lab::run();

    accommodation::tax::run();

    display::run();
}
