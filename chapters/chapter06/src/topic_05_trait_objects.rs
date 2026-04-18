//! Trait object（`dyn Trait`）：把"具体类型不同、行为接口相同"的值统一存放。
//!
//! 两种分派方式：
//!
//! | 方式       | 写法                    | 分派   | 开销     | 能否放进同一个集合？      |
//! |-----------|------------------------|--------|---------|---------------------|
//! | 静态分派   | `impl Trait` / 泛型 `<T>` | 编译期 | 零开销   | 不能：每个 T 是不同类型 |
//! | 动态分派   | `dyn Trait`            | 运行期 | 虚函数调用 | 可以：`Vec<Box<dyn T>>` |
//!
//! ### Trait object 安全
//!
//! 不是所有 trait 都能做成 object。要成为 **object-safe**：
//! - 方法不能返回 `Self`
//! - 方法不能有泛型参数
//! - 方法必须有 `&self` / `&mut self` / `self: Box<Self>` 其中之一
//!
//! 比如 `Clone` 就**不是** object-safe（`fn clone(&self) -> Self` 返回了 Self）。
//!
//! 本节演示：
//! 1. `Vec<Box<dyn Draw>>` 异构集合
//! 2. `&dyn Draw` / `Box<dyn Draw>` / `Arc<dyn Draw>` 三种常见 wrapper
//! 3. 动态分派 vs 静态分派的代码层面差异
//! 4. object-safe 失败示例（注释形式展示）

trait Draw {
    fn draw(&self) -> String;

    /// 带默认实现的方法也能被 trait object 调用。
    fn tag(&self) -> &'static str {
        "widget"
    }
}

struct Button { label: &'static str }
struct Label  { text:  &'static str }
struct Icon   { emoji: char }

impl Draw for Button {
    fn draw(&self) -> String { format!("button: {}", self.label) }
    fn tag(&self) -> &'static str { "button" }
}

impl Draw for Label {
    fn draw(&self) -> String { format!("label: {}", self.text) }
}

impl Draw for Icon {
    fn draw(&self) -> String { format!("icon: {}", self.emoji) }
    fn tag(&self) -> &'static str { "icon" }
}

/// 静态分派版本：泛型函数 + `impl Trait`，编译期单态化。
fn render_static<T: Draw>(widget: &T) {
    println!("[static]  tag={}, draw={}", widget.tag(), widget.draw());
}

/// 动态分派版本：`&dyn Draw`，一份代码服务所有实现者，运行期通过虚表查方法。
fn render_dynamic(widget: &dyn Draw) {
    println!("[dynamic] tag={}, draw={}", widget.tag(), widget.draw());
}

pub fn run() {
    println!("== Trait Objects ==");

    println!("-- (1) Vec<Box<dyn Draw>> 异构集合 --");
    let widgets: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "Submit" }),
        Box::new(Label  { text: "Ready" }),
        Box::new(Icon   { emoji: '🦀' }),
    ];

    for widget in &widgets {
        println!("  -> {} ({})", widget.draw(), widget.tag());
    }
    println!();

    println!("-- (2) 静态分派 vs 动态分派，同一个 item 两种调用方式 --");
    let b = Button { label: "OK" };
    render_static(&b);    // 编译期为 `Button` 单独生成一份 render_static
    render_dynamic(&b);   // 运行期通过 vtable 查到 Button::draw / Button::tag
    println!();

    println!("-- (3) 常见 wrapper 对比 --");
    println!("  &dyn Draw     ：借用型，生命周期受限");
    println!("  Box<dyn Draw> ：堆上独占所有权，适合集合中异构存储");
    println!("  Rc<dyn Draw>  ：单线程共享所有权");
    println!("  Arc<dyn Draw> ：多线程共享所有权");
    println!();

    println!("-- (4) object-safe（对象安全）--");
    println!("  不能做成 dyn 的 trait 例子：Clone、Iterator 的某些变体");
    println!("  规则：方法不能返回 Self / 不能有泛型参数 / 必须有 self 形式的接收者");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Trait Objects
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

trait Shape: Draw {
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

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
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
        println!("drawing Square");
    }
}
fn returns_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: dimension[0],
            line_width: 5,
            color: String::from("Red"),
        };
        Box::new(sq)
    } else {
        let rect = Rectangle {
            length: dimension[0],
            width: dimension[1],
            line_width: 5,
            color: String::from("Red"),
        };
        Box::new(rect)
    }
}

struct Circle {
    radius: f32,
}

fn shape_properties_static<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}

fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}
// fn shape_properties_rect(object: Rectangle) {
//     object.area();
//     object.perimeter();
// }

// fn shape_properties_sq(object: Square) {
//     object.area();
//     object.perimeter();
// }
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
    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));
}

/* 
---------------------------------------------------------------------------------------------------------
Concept/Topic             | Explanation
--------------------------|------------------------------------------------------------------------------
Static Dispatch           | Generics are compiled into specialized versions for each concrete type used.
                          | The compiler generates separate functions for each instantation.  
                          | Method calls are resolved at compile time with no runtime overhead.

Trait Objects             | Trait objects enable method resolution at runtime.
(Dynamic Dispatch)        | They are crated using dyn Trait syntax and are used with reference.
                          | The method implementation is chosen during execution.

Pointer Requirement       | Trait objects must be used behind a pointer type.
                          | Common options include Box<dyn Trait> and &dyn Trait.
                          | The pointer enables runtime indirection.

Static vs Dynamic dispatch| Static dispatch prioritizes performance and zero-cost abstraction.
                          | Dynamic dispatch provides flexibility at the cost of runtime lookup.
                          | Choice depends on performance vs extensibility needs.

Returning Trait Types     | impl Trait in return position requires a single concrete type.
                          | Using trait objects allows multiple return types dynamically.
---------------------------------------------------------------------------------------------------------
*/
"###;
