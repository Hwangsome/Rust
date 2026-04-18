//! Trait：描述"一个类型承诺提供哪些能力"——Rust 里的接口机制。
//!
//! 和其他语言的"接口 / 抽象类 / 类型类"相比，trait 的核心能力：
//!
//! - 定义方法**签名**，可以带默认实现也可以不带
//! - 不同类型分别 `impl Trait for Type {...}`，**没有继承，只有组合**
//! - 调用方用 `impl Trait` 或 `dyn Trait` 消费"具备这种能力的任何类型"
//! - 可以给**别人定义的类型**实现你自己的 trait（孤儿规则下），这一点比 Java interface 强
//!
//! 本节覆盖：
//! 1. 定义 trait 与默认实现
//! 2. 为两个不同 struct 分别实现同一个 trait
//! 3. 覆盖默认实现 vs 保留默认实现
//! 4. `&impl Trait`（静态分派）作为参数
//! 5. 孤儿规则（orphan rule）：在哪能给谁加 trait 实现

trait Shape {
    /// 没有默认实现：每个实现者**必须**提供自己的 area。
    fn area(&self) -> u32;

    /// 带默认实现：实现者可以不写，直接继承。
    fn perimeter(&self) -> u32 {
        0
    }

    /// 另一个带默认实现的方法，演示"默认方法调用其他抽象方法"。
    /// 只要实现了 `area()`，`describe` 就能自动工作。
    fn describe(&self) -> String {
        format!("Shape(area = {}, perimeter = {})", self.area(), self.perimeter())
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    side: u32,
}

#[derive(Debug)]
struct Circle {
    radius: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 覆盖默认实现，提供更准确的 perimeter。
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        self.side * 4
    }
}

// Circle 只实现 area，perimeter 与 describe 都用 trait 的默认实现。
impl Shape for Circle {
    fn area(&self) -> u32 {
        // 近似：π * r^2，这里用整数简化教学
        3 * self.radius * self.radius
    }
}

/// `&impl Shape` 语法糖：编译期单态化（静态分派），没有运行期开销。
fn print_shape_details(name: &str, shape: &impl Shape) {
    println!("{name} => {}", shape.describe());
}

pub fn run() {
    println!("== Traits ==");

    println!("-- (1) 不同类型共享同一个 trait 接口 --");
    let rect = Rectangle { width: 6, height: 4 };
    let sq = Square { side: 5 };
    let c = Circle { radius: 3 };

    print_shape_details("rectangle", &rect);
    print_shape_details("square   ", &sq);
    print_shape_details("circle   ", &c);
    println!();

    println!("-- (2) 默认方法可被覆盖，也可被继承 --");
    println!("  Rectangle 覆盖了 perimeter    -> {}", rect.perimeter());
    println!("  Circle 未覆盖 perimeter，用默认 0 -> {}", c.perimeter());
    println!();

    println!("-- (3) 组合 vs 继承 --");
    println!("  Rust 没有类继承；‘共享数据’用 struct 嵌套（composition）");
    println!("  ‘共享行为’用 trait + 默认方法");
    println!();

    println!("-- (4) 孤儿规则（orphan rule） --");
    println!("  你可以：为你定义的类型 实现 标准库 trait（impl Display for MyType）");
    println!("  你可以：为标准库类型 实现 你定义的 trait（impl MyTrait for String）");
    println!("  但不能：为标准库类型 实现 标准库 trait（impl Display for String）");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Traits
// -------------------------------------------

// struct drawing_info {
//     line_width: u8,
//     color: String,
// }
struct Square {
    side: f32,
    line_width: u8,
    color: String,
    //info: drawing_info,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
    // info: drawing_info,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

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

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();
}


/* 
----------------------------------------------------------------------------------------------------------------
Concept/Topic           | Explanation
------------------------|---------------------------------------------------------------------------------------
Problem Without Traits  | Separate impl blocks for different types can define behaviour inconsistently. 
                        | Method names, return types, and signatures may differ across types.
                        | This breaks uniform usage and requires type-specific handling.

Traits                  | They define a common consistent interface shared across multiple types.
                        | They declares method signatures generally without specifying implementation details.
                        | Types must implement all required trait methods.

Benefits of Traits      | Enforces a uniform method signature.
                        | Methods can be called uniformly regardless of concrete type.

Default Implementations | Traits can provide default method implementations.
                        | Types may override defaults or inherit them automatically.

Note on Inheritance     | Rust does not support classical inheritance for sharing data.
                        | Shared data is achieved via composition using common structs.
------------------------------------------------------------------------------------------------------------------
 */
"###;
