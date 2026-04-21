//! 为 Struct 实现 `Display` Trait
//!
//! `std::fmt::Display` 控制 `{}` 格式化的输出，与 `Debug`（`{:?}`）的区别：
//! - `Debug`：面向开发者，`#[derive(Debug)]` 自动生成，显示字段名和原始值
//! - `Display`：面向用户，**必须手写**，自由定义展示格式
//!
//! 实现方式：`impl std::fmt::Display for MyStruct { fn fmt(...) }`
//! 提供 `Display` 后，以下全部生效：
//! - `println!("{}", value)`
//! - `format!("{}", value)`
//! - `to_string()`（通过 `ToString` blanket impl 自动获得）

use std::fmt;

// ---------------------------------------------------------------------------
// Money — 货币金额
// ---------------------------------------------------------------------------

/// 带货币符号的金额。
#[derive(Debug)]
pub struct Money {
    pub amount: f64,
    pub currency: &'static str,
}

impl Money {
    pub fn new(amount: f64, currency: &'static str) -> Self {
        Self { amount, currency }
    }
}

/// `Display` 输出：`¥1,234.56`  / `$99.00`
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency, self.amount)
    }
}

// ---------------------------------------------------------------------------
// Color — RGB 颜色
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}

/// `Display` 输出：`rgb(255, 128, 0)`
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

// ---------------------------------------------------------------------------
// Invoice — 发票（包含嵌套 Display）
// ---------------------------------------------------------------------------

/// 单条发票：描述、税前金额、税率。
#[derive(Debug)]
pub struct Invoice {
    pub description: String,
    pub amount: Money,
    pub tax_rate: f64,
}

impl Invoice {
    pub fn new(description: &str, amount: Money, tax_rate: f64) -> Self {
        Self {
            description: description.to_string(),
            amount,
            tax_rate,
        }
    }

    fn tax(&self) -> f64 {
        self.amount.amount * self.tax_rate
    }

    fn total(&self) -> f64 {
        self.amount.amount + self.tax()
    }
}

/// `Display` 输出：多行格式化发票，内部直接用 `{}` 嵌套 `Money` 的 `Display`。
impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌─────────────────────────────────")?;
        writeln!(f, "│ {}", self.description)?;
        writeln!(f, "│ 税前：{}", self.amount)?;   // 嵌套调用 Money::Display
        writeln!(f, "│ 税率：{:.0}%", self.tax_rate * 100.0)?;
        writeln!(f, "│ 税额：{}{:.2}", self.amount.currency, self.tax())?;
        write!  (f, "│ 合计：{}{:.2}", self.amount.currency, self.total())?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------

pub fn run() {
    println!("== display — 为 Struct 实现 Display Trait ==");

    println!();
    println!("-- (1) Money：{{}} vs {{:?}} --");
    let price = Money::new(1_234.56, "¥");
    println!("  Display  : {price}");           // ¥1234.56
    println!("  Debug    : {price:?}");         // Money { amount: 1234.56, currency: "¥" }
    println!("  to_string: {}", price.to_string()); // Display 自动赋予 to_string()

    println!();
    println!("-- (2) Color --");
    let bg = Color::new(30, 144, 255);
    println!("  Display : {bg}");
    println!("  Debug   : {bg:?}");

    println!();
    println!("-- (3) Invoice（嵌套 Display）--");
    let invoice = Invoice::new(
        "年终奖所得税",
        Money::new(50_000.0, "¥"),
        0.20,
    );
    println!("{invoice}");

    println!();
    println!("-- (4) format! 与 to_string --");
    let label = format!("本月工资: {}", Money::new(20_000.0, "¥"));
    println!("  format! → {label}");
    let s: String = Color::new(255, 0, 0).to_string();
    println!("  to_string → {s}");

    println!();
}
