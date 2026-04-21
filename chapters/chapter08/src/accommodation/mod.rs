//! # accommodation 模块
//!
//! 演示**文件式模块**（file-based module pattern）：
//! - `mod.rs` 定义公开的 trait 与 re-export
//! - 每个结构体独占一个子文件（`hotel.rs`、`airbnb.rs`、`hostel.rs`）
//!
//! 调用方只需 `use accommodation::{Hotel, Airbnb, Hostel, Accommodation, StayPolicy}`
//! 就能访问所有对外类型，内部文件划分对外部透明。

use std::collections::HashMap;

// 声明子模块 —— Rust 编译器会去找同目录下的同名 .rs 文件
pub mod hotel;
pub mod airbnb;
pub mod hostel;
pub mod tax;

// Re-export：让 `accommodation::Hotel` 而不是 `accommodation::hotel::Hotel` 可用
pub use hotel::Hotel;
pub use airbnb::Airbnb;
pub use hostel::Hostel;

// ─── Trait 定义放在 mod.rs，所有子模块均可直接引用 ───────────────  ────────

/// 住宿的基本合同：展示与预订。
pub trait Accommodation {
    fn description(&self) -> &str;
    fn book(&mut self, name: &str, nights: u32) -> Result<(), String>;

    fn search_listing_line(&self) -> String {
        format!("Book · {}", self.description())
    }
}

/// 入住政策：最大晚数限制。
pub trait StayPolicy {
    fn max_nights_per_booking(&self) -> u32;
}

// 仅供模块内部使用，避免 HashMap 被外层 use 导入时 clippy 警告
#[allow(dead_code)]
pub(crate) type Reservations = HashMap<String, u32>;
