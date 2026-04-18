//! Trait 对象不允许方法有**泛型参数**。
//!
//! ```ignore
//! trait Bad { fn g<T>(&self, x: T); }
//! // Bad 不是 object-safe：方法带泛型 T 会让编译器无法生成 vtable
//! // （单态化需要在编译期知道 T，但 dyn Trait 的 T 是运行期才知道）
//! ```
//!
//! 解决：把泛型**挪到 trait 级别**，或改用 `Box<dyn Any>`。

trait Fixed {
    fn name(&self) -> &str;
    fn show_i32(&self, x: i32);
}

struct A;
impl Fixed for A {
    fn name(&self) -> &str { "A" }
    fn show_i32(&self, x: i32) { println!("  [{}] x = {x}", self.name()); }
}

pub fn run() {
    println!("== Methods with Generics (limitation) ==");

    let obj: &dyn Fixed = &A;
    obj.show_i32(42);

    println!("  object-safe 的方法只能有具体类型，不能带泛型参数");
    println!();
}
