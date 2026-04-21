//! 大小与 trait object：为什么 `dyn Trait` 必须藏在指针后。
//!
//! `dyn Trait` 是 **unsized**——具体类型在编译期未知，大小不固定。
//! 所以不能直接按值传或放字段，必须：
//!
//! - `&dyn Trait`
//! - `&mut dyn Trait`
//! - `Box<dyn Trait>` / `Rc<dyn Trait>` / `Arc<dyn Trait>`

use std::mem::size_of;

trait Animal { fn name(&self) -> &str; }
struct Dog;
impl Animal for Dog { fn name(&self) -> &str { "dog" } }

pub fn run() {
    println!("== Size & Trait Objects ==");

    println!("  &dyn Animal 大小 = {} bytes (胖指针：ptr + vtable)", size_of::<&dyn Animal>());
    println!("  &Dog 大小       = {} bytes (普通指针)", size_of::<&Dog>());

    let d: Box<dyn Animal> = Box::new(Dog);
    println!("  via dyn = {}", d.name());
    println!();
}
