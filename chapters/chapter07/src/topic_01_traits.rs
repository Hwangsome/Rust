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
//! 4. `&impl Trait`（静态分派）作为参数——编译期为每个具体类型生成一份函数，零虚调用开销
//! 5. 孤儿规则（orphan rule）：在哪能给谁加 trait 实现
//! 6. 业务示例 `Accommodation`：`&mut self` 与预订、`HashMap` / `Vec` 存状态
//! 7. 泛型 + trait bound：`fn f<T: Accommodation>(x: &mut T)` 与 `impl Trait` 的对比
//! 8. **第二个业务 trait `StayPolicy`**：与 `Accommodation` 正交；演示 `T: Accommodation + StayPolicy` 多 bound
//!
//! **关于 `&mut`：** 若 trait 方法需要修改接收者（如 `book` 写入订单），参数必须是 `&mut T`，
//! 不能对 `&T` 调用 `&mut self` 方法——这是 Rust 借用检查器的核心规则。

use std::collections::HashMap;

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

/// 接受「任意实现了 `Shape` 的类型」的引用。
///
/// **写法一：`&impl Shape`**（本函数所用）
/// - 含义：参数类型在编译期推断为某个实现了 `Shape` 的具体类型。
/// - 单态化：对每个调用点（`&Rectangle`、`&Square`…）生成一份特化代码，**静态分派**。
///
/// **等价写法二：** `fn print_shape_details<S: Shape>(name: &str, shape: &S)`
/// - `S` 是显式类型参数；当多个参数需要「同一个 `S`」时，泛型写法更方便。
fn print_shape_details(name: &str, shape: &impl Shape) {
    println!("{name} => {}", shape.describe());
}

// ---------------------------------------------------------------------------
// 场景：旅游搜索 / 预订（Expedia、Kayak、Skyscanner 等）
// ---------------------------------------------------------------------------
// 用户搜索不同住宿并下单；调用方只关心「能描述、能预订」的抽象，不关心是酒店还是民宿。
// **trait 名称**表示能力合同：实现者可以有很多种，今天 `Hotel` / `Airbnb`，以后可加 `Hostel`。
//
trait Accommodation {
    /// 只读展示名（搜索列表、详情页标题）；不修改 `self`，故 `&self`。
    fn description(&self) -> &str;

    /// 下单：会写入 `reservations` / `guests` 等状态，**必须** `&mut self`。
    /// - 若函数参数只有 `&impl Accommodation`，只能调用 `&self` 方法；要调用 `book` 须传入 `&mut T`。
    fn book(&mut self, name: &str, nights: u32) -> Result<(), String>;

    /// 默认实现：**搜索/列表页**共用的一行展示文案，只依赖 `description()`。
    /// 场景：Expedia 类网站要在结果列表里渲染成千上万条住宿，规则相同（前缀 + 名称），
    /// 多数类型直接用默认即可；少数（如连锁酒店）可覆盖以加品牌、会员提示等。
    /// 这与上面 `Shape::describe` 一样：默认方法里调用**尚未由本类型实现的方法**。
    fn search_listing_line(&self) -> String {
        format!("Book · {}", self.description())
    }
}

/// **入住政策**（与 `Accommodation` 分离）：描述「单次预订允许的最长晚数」等**规则**，
/// 不包含下单、改库存等动作。酒店 / 民宿可以分别实现不同上限。
///
/// 与 `Accommodation` **组合**：`Hotel` / `Airbnb` 同时 `impl Accommodation + StayPolicy`，
/// 泛型里可写 `T: Accommodation + StayPolicy`，在一份函数里既读政策又走预订流程。
trait StayPolicy {
    /// 单次入住允许的最大晚数（酒店通常更短，民宿可能更长）。
    fn max_nights_per_booking(&self) -> u32;
}

/// 酒店：用 **`HashMap<姓名, 晚数>`** 表示「每个客人当前预订的晚数」；同名再次预订会**覆盖**旧值。
#[derive(Debug)]
struct Hotel {
    name: String,
    availability: bool,
    reservations: HashMap<String, u32>,
}

/// 民宿：用 **`Vec<(姓名, 晚数)>`** 保留**多次预订顺序**；同一人多次下单会产生多条记录（与 `HashMap` 键唯一不同）。
#[derive(Debug)]
struct Airbnb {
    name: String,
    availability: bool,
    guests: Vec<(String, u32)>,
}

impl Hotel {
    fn new(name: String) -> Self {
        Self {
            name,
            availability: true,
            reservations: HashMap::new(),
        }
    }

    /// 演示：**固有 `impl Hotel`** 里也能调用 `Accommodation` 的方法。
    /// 条件：`Hotel: Accommodation` 已实现，且 `Accommodation` 在当前作用域可见（同模块已满足）。
    /// 这与 `impl Accommodation for Hotel { ... }` 里互相调用 `description` / `search_listing_line` 一样，
    /// 都是「通过 `self` 调用该类型所实现的 trait 项」。
    fn debug_banner(&self) -> String {
        format!("{} → {}", self.description(), self.search_listing_line())
    }
}

impl Airbnb {
    fn new(name: String) -> Self {
        Self {
            name,
            availability: true,
            guests: Vec::new(),
        }
    }
}

impl StayPolicy for Hotel {
    fn max_nights_per_booking(&self) -> u32 {
        14
    }
}

impl StayPolicy for Airbnb {
    fn max_nights_per_booking(&self) -> u32 {
        30
    }
}

// --- `impl Trait for Type`：为具体类型提供 trait 合同 ---
impl Accommodation for Hotel {
    fn description(&self) -> &str {
        &self.name
    }

    // 覆盖默认：酒店希望在列表里突出「酒店」与政策提示（晚数上限来自 `StayPolicy`）。
    fn search_listing_line(&self) -> String {
        format!(
            "🏨 Hotel · {} — max {} nights per stay",
            self.description(),
            self.max_nights_per_booking()
        )
    }

    fn book(&mut self, name: &str, nights: u32) -> Result<(), String> {
        if !self.availability {
            return Err(String::from("Not available"));
        }
        if name.trim().is_empty() {
            return Err(String::from("Guest name required"));
        }
        if nights == 0 {
            return Err(String::from("Must book at least 1 night"));
        }
        let cap = self.max_nights_per_booking();
        if nights > cap {
            return Err(format!("Hotel bookings limited to {cap} nights"));
        }
        self.reservations.insert(name.to_string(), nights);
        Ok(())
    }
}

impl Accommodation for Airbnb {
    fn description(&self) -> &str {
        &self.name
    }
    fn book(&mut self, name: &str, nights: u32) -> Result<(), String> {
        if !self.availability {
            return Err(String::from("Not available"));
        }
        if name.trim().is_empty() {
            return Err(String::from("Guest name required"));
        }
        if nights == 0 {
            return Err(String::from("Must book at least 1 night"));
        }
        let cap = self.max_nights_per_booking();
        if nights > cap {
            return Err(format!("Airbnb stays limited to {cap} nights"));
        }
        self.guests.push((name.to_string(), nights));
        Ok(())
    }
}

/// 订一晚：演示 **泛型类型参数 + trait bound** `T: Accommodation`。
///
/// **`<T: Accommodation>` 读法**
/// - `T`：一个待定的具体类型（由调用处推断：传 `&mut Hotel` 则 `T = Hotel`）。
/// - `: Accommodation`：**约束**——`T` 必须实现 `Accommodation`，否则编译错误。
///
/// **与 `fn f(x: &mut impl Accommodation)` 的对比**
/// - `impl Trait`：匿名，适合「单参数、不关心名字」。
/// - `<T: Accommodation>`：显式 `T`，便于同一函数里多个参数**共享同一类型**（如 `fn eq<T: Accommodation>(a: &T, b: &T)`），
///   或返回 `T` / 与 `T` 关联的其它类型。
///
/// **为何参数是 `&mut T` 而不是 `&T`？**
/// - `book` 的接收者是 `&mut self`，会修改住宿内部数据；必须通过可变引用传入。
fn book_for_one_night<T: Accommodation>(accommodation: &mut T) -> Result<(), String> {
    accommodation.book("John", 1)
}

/// 两个**不同**泛型参数，各自约束为 `Accommodation`：可同时订「酒店 + 民宿」而无需统一成同一具体类型。
fn mix_and_match<T: Accommodation, U: Accommodation>(accommodation1: &mut T, accommodation2: &mut U) -> Result<(), String> {
    accommodation1.book("John", 1)?;
    accommodation2.book("Jane", 1)?;
    Ok(())
}

/// **多 trait 绑定（本场景自建特质）**：`T` 必须同时实现 `Accommodation` 与 `StayPolicy`。
///
/// - `Accommodation`：描述、下单、列表文案等**行为**。
/// - `StayPolicy`：**政策数据**（单次最多几晚），与是否可预订、如何存储订单正交。
///
/// 只有「既是一种住宿、又声明了入住上限」的类型才能传入（本例中 `Hotel`、`Airbnb` 均满足）。
fn preview_long_stay_quote<T: Accommodation + StayPolicy>(stay: &T, nights: u32) -> String {
    format!(
        "{} — policy cap {} nights / booking, requested {} nights",
        stay.description(),
        stay.max_nights_per_booking(),
        nights
    )
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

    println!("-- (5) 场景：预订住宿 --");
    let mut hotel = Hotel::new(String::from("Hotel"));
    let mut airbnb = Airbnb::new(String::from("Airbnb"));
    println!("  Hotel description: {}", hotel.description());
    println!("  Airbnb description: {}", airbnb.description());
    println!("  Listing line (Hotel 覆盖默认 search_listing_line): {}", hotel.search_listing_line());
    println!("  Listing line (Airbnb 用 trait 默认实现): {}", airbnb.search_listing_line());
    println!(
        "  固有 impl 中调用 trait 方法 (Hotel::debug_banner): {}",
        hotel.debug_banner()
    );
    println!();

    println!("  Booking hotel: {:?}", hotel.book("John", 2));
    println!("  Booking airbnb: {:?}", airbnb.book("John", 2));
    println!("  Hotel: {:?}", hotel);
    println!("  Airbnb: {:?}", airbnb);
    println!();



    // `book_for_one_night`：同一函数分别单态化为 `T=Hotel` 与 `T=Airbnb`，无需两套函数名。
    let mut hotel2 = Hotel::new(String::from("Hotel2"));
    let mut airbnb2 = Airbnb::new(String::from("Airbnb2"));
    println!("  Booking for one night: {:?}", book_for_one_night(&mut hotel2));
    println!("  Booking for one night: {:?}", book_for_one_night(&mut airbnb2));
    println!("  Hotel2: {:?}", hotel2);
    println!("  Airbnb2: {:?}", airbnb2);
    println!();

    // `T: Accommodation + StayPolicy`：见 `preview_long_stay_quote` 与 `StayPolicy` 注释。
    println!(
        "  preview (Hotel2): {}",
        preview_long_stay_quote(&hotel2, 10)
    );
    println!(
        "  preview (Airbnb2): {}",
        preview_long_stay_quote(&airbnb2, 10)
    );
    println!();

    println!("  Mixing and matching: {:?}", mix_and_match(&mut hotel2, &mut airbnb2));
    println!("  Hotel2: {:?}", hotel2);
    println!("  Airbnb2: {:?}", airbnb2);
    println!();
}