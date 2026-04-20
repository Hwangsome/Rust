//! Trait bounds：把"trait 是能力"推进到"trait bound 是函数的入场条件"。
//!
//! 当函数/struct/impl 写成泛型，你就需要告诉编译器：**这个 T 必须至少实现哪些 trait？**
//! 这就是 trait bound。Rust 对此有 **3 种等价写法**：
//!
//! 1. **内联语法** `fn f<T: Shape + Debug>(...)`
//! 2. **where 子句**  `fn f<T>(...) where T: Shape + Debug`
//! 3. **`impl Trait` 参数位置** `fn f(shape: &impl Shape)` —— 最简洁，但不能给多个参数同一个 T 绑定
//!
//! 返回值位置上的 `impl Trait`：`fn build() -> impl Shape`
//! - 调用者知道"返回值实现了 Shape"
//! - 但**具体类型是编译期推断**，调用者不能假设它是某个特定类型
//! - 适合返回闭包、impl Iterator 等"类型名字根本写不出来"的场景

use std::fmt::{Debug, Display};

trait Shape {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

#[derive(Debug)]
struct Circle { radius: u32 }

impl Shape for Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}

impl Shape for Circle {
    fn area(&self) -> u32 { 3 * self.radius * self.radius }
}

// 写法 1：内联 `<T: Trait>`，多个 trait 用 `+` 连接。
fn shape_summary_inline<T: Shape + Debug>(shape: &T) {
    println!("[inline] {:?} area = {}", shape, shape.area());
}

// 写法 2：where 子句——复杂约束更可读。
fn shape_summary_where<T>(shape: &T)
where
    T: Shape + Debug,
{
    println!("[where ] {:?} area = {}", shape, shape.area());
}

// 写法 3：参数位置 `impl Trait`——最简洁。
fn shape_summary_impl(shape: &(impl Shape + Debug)) {
    println!("[impl  ] {:?} area = {}", shape, shape.area());
}

// 多参数 + 同一个 T 必须用泛型参数，`impl Trait` 做不到。
fn compare_area<T: Shape>(a: &T, b: &T) -> bool {
    a.area() == b.area()
}

// 返回位置 `impl Shape`：调用方知道有 area()，但不知道具体是什么类型。
fn build_square(side: u32) -> impl Shape {
    struct Square { side: u32 }

    impl Shape for Square {
        fn area(&self) -> u32 { self.side * self.side }
    }

    Square { side }
}

// 把两个 bound 组合起来：`T: Display + PartialOrd` —— 既能打印又能比较。
fn larger_of<T: Display + PartialOrd>(a: T, b: T) -> T {
    if a > b { println!("picked {a}"); a } else { println!("picked {b}"); b }
}

pub fn run() {
    println!("== Trait Bounds ==");

    let rect = Rectangle { width: 8, height: 3 };
    let _circle = Circle { radius: 5 }; // 留作 compare_area 反例提示（注释里）

    println!("-- (1) 三种等价写法：内联 / where / impl Trait 参数 --");
    shape_summary_inline(&rect);
    shape_summary_where(&rect);
    shape_summary_impl(&rect);
    println!();

    println!("-- (2) 多参数同类型只能用泛型参数 --");
    let r2 = Rectangle { width: 4, height: 6 };
    println!("compare_area(rect, r2) = {}", compare_area(&rect, &r2));
    // compare_area(&rect, &circle); // ← 会 E0308：第二个参数不是 Rectangle
    println!();

    println!("-- (3) 返回位置 impl Shape：隐藏具体类型 --");
    let hidden_square = build_square(4);
    println!("impl Shape return => area = {}", hidden_square.area());
    // 调用方知道它实现 Shape，但不能当 Rectangle 用。
    println!();

    println!("-- (4) 多个 trait bound --");
    let _ = larger_of(7, 3);
    let _ = larger_of("banana", "apple");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Trait Bounds
// -------------------------------------------

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }
    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

fn shape_properties<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}

fn returns_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    sq
    // let rect = Rectangle {
    //     length: 5.0,
    //     width: 10.0,
    //     line_width: 5,
    //     color: String::from("Red"),
    // };

    // let x = false;
    // if x {
    //     sq
    // } else {
    //     rect
    // }
}

struct Circle {
    radius: f32,
}
fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: String::from("Red"),
    };

    let s1 = Square {
        side: 3.2,
        line_width: 1,
        color: String::from("Red"),
    };

    let c1 = Circle { radius: 5.0 };
    shape_properties(r1);
    shape_properties(s1);
    // shape_properties(c1); // Trait bound not satisfied
}

/*
---------------------------------------------------------------------------------------------
Concept/Topic         | Explanation
----------------------|----------------------------------------------------------------------
Trait Bounds          | It restrict a generic type to those implementing a trait.
                      | Syntax: T: SomeTrait ensures, only types with SomeTrait are allowed.
                      | Trait bounds enables access to the trait’s methods inside the function.

Bound Syntax Variants | Trait bounds can be expressed in multiple equivalent forms.
                      | T: Trait, impl Trait, and where clause are alternatives.
                      | where clause improves readability for multiple complex bounds.

impl Trait in Returns | impl Trait can be used as a function return type.
                      | It indicates the function returns some type implementing the trait.
                      | impl Trait return requires one concrete type.
                      | Conditional returns of different concrete types are not allowed.
                      | Trait objects are required for such dynamic return scenarios.
----------------------------------------------------------------------------------------------
*/
"###;
