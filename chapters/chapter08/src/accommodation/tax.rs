//! Trait 泛型参数 + 继承
//!
//! **演变路径**
//!
//! 上一版 `Investment` 把 `amount()` 的返回类型硬编码为 `f64`，
//! 导致 `QualityTime`（分钟，整数更自然）也被迫用浮点数表示。
//!
//! 本版改为 `Investment<T>`：
//! - `T` 是占位符，由**实现者**决定填入什么具体类型
//! - `Taxable: Investment<f64>` —— 凡是要征税的类型，T 固定为 f64
//! - `QualityTime` 只实现 `Investment<u32>`，分钟数保持整数
//!
//! ```text
//! Investment<T>          ← 泛型基础合同：amount() 返回 T
//!     └─ Taxable         ← 子合同：要求 Investment<f64>，加税率与税额
//!
//! Income      impl Investment<f64>  + impl Taxable
//! Bonus       impl Investment<f64>  + impl Taxable  (税率 30%)
//! QualityTime impl Investment<u32>                  (无税)
//! ```

// ---------------------------------------------------------------------------
// Super Trait（泛型）
// ---------------------------------------------------------------------------

/// 投资标的基础合同：可以查询"价值"并将其翻倍。
///
/// `T` 是价值的类型——可以是 `f64`（货币金额）、`u32`（整数分钟）等。
/// 不再内置 `set_amount` 和 `double_amount` 的默认实现，
/// 让每个类型自行决定如何翻倍（浮点 × 2.0，整数 × 2，等）。
pub trait Investment<T> {
    fn amount(&self) -> T;
    fn double_amount(&mut self);
}

// ---------------------------------------------------------------------------
// Sub Trait（指定 T = f64）
// ---------------------------------------------------------------------------

/// 可征税合同：要求同时实现 `Investment<f64>`。
///
/// `Taxable: Investment<f64>` 表示：
/// 任何实现 `Taxable` 的类型，其 `amount()` 必须返回 `f64`。
pub trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

// ---------------------------------------------------------------------------
// Income — Investment<f64> + Taxable
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Income {
    amount: f64,
}

impl Income {
    pub fn new(amount: f64) -> Self { Self { amount } }
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64      { self.amount }
    fn double_amount(&mut self)  { self.amount *= 2.0; }
}

impl Taxable for Income {
    // TAX_RATE 使用默认值 0.25
}

// ---------------------------------------------------------------------------
// Bonus — Investment<f64> + Taxable（税率 30%）
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Bonus {
    amount: f64,
}

impl Bonus {
    pub fn new(amount: f64) -> Self { Self { amount } }
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64      { self.amount }
    fn double_amount(&mut self)  { self.amount *= 2.0; }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.30;
}

// ---------------------------------------------------------------------------
// QualityTime — Investment<u32> 只（无税）
// ---------------------------------------------------------------------------

/// 与家人共度的时光：分钟数用整数表示，不征税。
///
/// 因为 `Investment<T>` 是泛型的，我们可以选择 `u32` 作为 T，
/// 完全不受"必须是 f64"的限制。
#[derive(Debug)]
pub struct QualityTime {
    minutes: u32,
}

impl QualityTime {
    pub fn new(minutes: u32) -> Self { Self { minutes } }
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32      { self.minutes }
    fn double_amount(&mut self)  { self.minutes *= 2; }
}

// QualityTime 不实现 Taxable — 家人时光无需缴税

// ---------------------------------------------------------------------------

pub fn run() {
    println!("== accommodation::tax — 泛型 Trait + 继承 ==");

    let mut income  = Income::new(20_000.0);
    let mut bonus   = Bonus::new(50_000.0);
    let mut quality = QualityTime::new(1_000); // 1000 分钟

    println!();
    println!("-- (1) 三种类型都实现 Investment，都支持 double_amount --");
    income.double_amount();
    bonus.double_amount();
    quality.double_amount();
    println!("  income  after double : ¥{:.2}", income.amount());
    println!("  bonus   after double : ¥{:.2}", bonus.amount());
    println!("  quality after double : {} 分钟（u32，不是 f64）", quality.amount());

    println!();
    println!("-- (2) Taxable 约束：Income / Bonus 可传入，QualityTime 编译报错 --");
    print_tax_summary("Income", &income);
    print_tax_summary("Bonus",  &bonus);
    // print_tax_summary("QualityTime", &quality); // ← 编译错误：未实现 Taxable

    println!();
    println!("-- (3) Investment<f64> 约束：Income / Bonus 满足 --");
    print_f64_amount("Income", &income);
    print_f64_amount("Bonus",  &bonus);

    println!();
    println!("-- (4) Investment<u32> 约束：只有 QualityTime 满足 --");
    print_u32_amount("QualityTime", &quality);

    println!();
}

fn print_tax_summary<T: Taxable>(label: &str, item: &T) {
    println!("  {label:<10}  金额 {:>10.2}  税率 {:>3.0}%  税额 {:>8.2}",
        item.amount(), T::TAX_RATE * 100.0, item.tax_bill());
}

fn print_f64_amount<T: Investment<f64>>(label: &str, item: &T) {
    println!("  {label:<10}  f64 amount = {:.2}", item.amount());
}

fn print_u32_amount<T: Investment<u32>>(label: &str, item: &T) {
    println!("  {label:<10}  u32 amount = {}", item.amount());
}
