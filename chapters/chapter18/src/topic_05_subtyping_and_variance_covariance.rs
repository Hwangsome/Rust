//! 协变（covariance）：最常见的方差方向。
//!
//! 一个泛型容器 `F<T>` 在 T 上**协变**，意思是：
//!
//! **如果 `A: B`（A 是 B 的子类型），那么 `F<A>: F<B>`**（容器关系同方向）。
//!
//! 标准库里**绝大多数**类型对生命周期都是协变的：
//!
//! - `&'a T` 对 `'a` 协变：`&'long T` 能当 `&'short T` 用
//! - `Box<T>` / `Vec<T>` / `Rc<T>` 对 T 协变
//!
//! 直觉：**"只读 / 只向外给"的容器是协变的**。

pub fn run() {
    println!("== Covariance ==");

    println!("-- (1) &'a T 对 'a 协变 --");
    let literal: &'static str = "hello";
    fn needs<'a>(_: &'a str) {}
    needs(literal); // 'static → 'a，协变允许
    println!("  literal 可以当任何 'a 用");
    println!();

    println!("-- (2) Vec<T> 对 T 协变（考虑引用层面） --");
    fn accept_v<'a>(_: &Vec<&'a str>) {}
    let v: Vec<&'static str> = vec!["a", "b"];
    accept_v(&v); // &Vec<&'static str> 能当 &Vec<&'a str>
    println!("  Vec<&'static str> 可以当 Vec<&'a str> 用（协变）");
    println!();
}
