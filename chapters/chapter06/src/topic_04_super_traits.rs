//! Super trait：一个 trait 可以要求"想实现我，先实现别的 trait"。
//!
//! 写法 `trait ShapeCard: Draw + Named { ... }` 表示：
//! - 要为类型 T 实现 `ShapeCard`，T 必须**已经**实现 `Draw` 和 `Named`
//! - `ShapeCard` 的默认方法体里可以**直接调用** `self.draw()` / `self.name()`
//!
//! 注意：**Rust 没有继承**。super trait 只是"在 bound 上要求这两个能力同时存在"。
//! 不会"从父 trait 继承字段"（trait 本来就没字段）。
//!
//! 本节演示：
//! 1. 多个基础 trait 的组合
//! 2. 在 super trait 默认方法里调用下层 trait
//! 3. 一个类型为什么要**显式** `impl ShapeCard for X {}`，即使它已实现了 `Draw + Named`
//! 4. super trait 的"可替代写法"：直接在泛型约束里写 `T: Draw + Named`

trait Draw {
    fn draw(&self) -> String;
}

trait Named {
    fn name(&self) -> &'static str;
}

/// Super trait：把 `Draw + Named` 打包成更高层的"Card"抽象，并提供默认 `summary`。
trait ShapeCard: Draw + Named {
    fn summary(&self) -> String {
        format!("{} => {}", self.name(), self.draw())
    }

    /// 可以定义更多依赖底层 trait 的默认方法。
    fn short_tag(&self) -> String {
        format!("[{}]", self.name())
    }
}

struct CircleCard;
struct ButtonCard { label: &'static str }

impl Draw for CircleCard {
    fn draw(&self) -> String { "draw circle".to_string() }
}
impl Named for CircleCard {
    fn name(&self) -> &'static str { "CircleCard" }
}
// 显式告诉编译器："我也是个 ShapeCard"——这一步不能省。
impl ShapeCard for CircleCard {}

impl Draw for ButtonCard {
    fn draw(&self) -> String { format!("draw button [{}]", self.label) }
}
impl Named for ButtonCard {
    fn name(&self) -> &'static str { "ButtonCard" }
}
impl ShapeCard for ButtonCard {}

/// 替代写法：不定义 super trait，直接在 bound 里写 `T: Draw + Named`——
/// 能力完全等价，只是缺少"把默认方法打包到一个命名抽象下"的方便。
fn summary_via_bounds<T: Draw + Named>(t: &T) -> String {
    format!("{} => {}", t.name(), t.draw())
}

pub fn run() {
    println!("== Super Traits ==");

    println!("-- (1) super trait 把多个基础 trait 组合成更高层 --");
    let c = CircleCard;
    let b = ButtonCard { label: "Submit" };
    println!("{}", c.summary());
    println!("{}", b.summary());
    println!();

    println!("-- (2) super trait 的默认方法可以调用下层 trait --");
    println!("short_tag: {} / {}", c.short_tag(), b.short_tag());
    println!();

    println!("-- (3) 即使已实现 Draw + Named，仍需显式 impl ShapeCard for X {{}} --");
    println!("  Rust 不会自动给你加 blanket impl，除非你写：");
    println!("  impl<T: Draw + Named> ShapeCard for T {{}}   // （一旦加了全局 blanket，就会自动应用）");
    println!();

    println!("-- (4) 不想用 super trait 也行：直接在 bound 里写 Draw + Named --");
    println!("{}", summary_via_bounds(&c));
    println!();

    println!("结论：super trait 是一种‘命名 + 打包默认方法’的便利，不是 OOP 继承。");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Super Traits
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

trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

trait OtherTrait {}
impl OtherTrait for Rectangle {}
impl OtherTrait for Square {}

trait SomeOtherTrait {}
impl SomeOtherTrait for Rectangle {}
impl SomeOtherTrait for Square {}

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

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}
impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
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

    shape_properties(r1);
    shape_properties(s1);
}


/* 
-----------------------------------------------------------------------------------------------
Concept/Topic         | Explanation
----------------------|------------------------------------------------------------------------
Super Traits          | A super trait is a trait that requires implementing another trait.
                      | Syntax: trait Trait1: Trait2 {} means Trait2 is super trait of Trait1.
                      | Any type implementing Trait1 must also implement Trait2.

Multiple Super Traits | A trait can declare multiple super traits using the + syntax.
                      | Example: trait SomeTrait: Trait1 + Trait2. 

Reducing Trait Bounds | Super traits can reduce the traits bounds.  
                      | if trait SomeTrait: trait1 + trait2 then, 
                      | T: SomeTrait guarentees the bounds of Trait1 and Trait2, 
                      | T: SomeTrait + Trait1 + Trait2 redundant. 

Marker Traits         | Traits without methods are called marker traits.
                      | They convey semantic meaning rather than behavior.
                      | They can also be used as super traits.
----------------------------------------------------------------------------------------------
*/
"###;
