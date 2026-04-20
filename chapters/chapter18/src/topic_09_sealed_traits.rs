//! Sealed trait（封闭 trait）：让外部 crate **不能**实现你的 trait。
//!
//! 标准库大量使用——比如 `std::error::Error` 的一些辅助 trait。
//!
//! 常见实现方式：用一个**私有模块**里的 trait 做基础：
//!
//! ```ignore
//! mod private { pub trait Sealed {} }
//! pub trait MyTrait: private::Sealed { ... }
//! ```
//!
//! 外部 crate 看不到 `private::Sealed`，就无法实现 `MyTrait`。

mod sealed_mod {
    pub trait Sealed {}
}

pub trait OnlyMyTypes: sealed_mod::Sealed {
    fn describe(&self) -> String;
}

struct MyType;
impl sealed_mod::Sealed for MyType {}
impl OnlyMyTypes for MyType {
    fn describe(&self) -> String { "MyType".into() }
}

pub fn run() {
    println!("== Sealed Traits ==");

    let m = MyType;
    println!("  {}", m.describe());
    println!();
    println!("  外部 crate 看不到 sealed_mod::Sealed → 没法实现 OnlyMyTypes");
    println!("  这是库作者保证\"只有我认可的类型\"才能实现 trait 的常用手法");
    println!();
}
