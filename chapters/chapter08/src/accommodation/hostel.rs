//! `Hostel<T>` 结构体及其 trait 实现。

use super::{Accommodation, StayPolicy};

/// 青年旅社：`name` 字段为泛型载荷（可以是 ID、配置句柄等任意类型），
/// 对外展示用独立的 `listing_title: String`，满足 `Accommodation::description -> &str`。
#[derive(Debug)]
pub struct Hostel<T> {
    name: T,
    listing_title: String,
    availability: bool,
    guests: Vec<(String, u32)>,
}

impl<T> Hostel<T> {
    pub fn new(name: T, listing_title: impl Into<String>) -> Self {
        Self {
            name,
            listing_title: listing_title.into(),
            availability: true,
            guests: Vec::new(),
        }
    }

    /// 返回构造时存入的泛型载荷。
    pub fn payload(&self) -> &T {
        &self.name
    }
}

impl<T> StayPolicy for Hostel<T> {
    fn max_nights_per_booking(&self) -> u32 {
        7
    }
}

impl<T> Accommodation for Hostel<T> {
    fn description(&self) -> &str {
        &self.listing_title
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
            return Err(format!("Hostel stays limited to {cap} nights"));
        }
        self.guests.push((name.to_string(), nights));
        Ok(())
    }
}
