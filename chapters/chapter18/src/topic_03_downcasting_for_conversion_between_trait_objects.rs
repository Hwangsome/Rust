//! Trait object 之间的"转换"：通常要先 downcast 到具体类型，再重新 upcast 到另一个 trait。
//!
//! Rust 没有直接 `&dyn A → &dyn B` 的转换。必须：
//!
//! 1. 通过 `as_any()` 获取 `&dyn Any`
//! 2. downcast 到具体类型
//! 3. 再用 `as &dyn B` 重新 upcast

use std::any::Any;

trait Drawable: Any {
    fn draw(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

trait Printable {
    fn print_info(&self);
}

struct Card { title: String }

impl Drawable for Card {
    fn draw(&self) -> String { format!("draw: {}", self.title) }
    fn as_any(&self) -> &dyn Any { self }
}

impl Printable for Card {
    fn print_info(&self) { println!("  card-info: {}", self.title); }
}

pub fn run() {
    println!("== Downcasting for Conversion ==");

    let d: Box<dyn Drawable> = Box::new(Card { title: "Rust".into() });
    println!("  {}", d.draw());

    // dyn Drawable → Card → &dyn Printable
    if let Some(card) = d.as_any().downcast_ref::<Card>() {
        let p: &dyn Printable = card;
        p.print_info();
    }
    println!();
}
