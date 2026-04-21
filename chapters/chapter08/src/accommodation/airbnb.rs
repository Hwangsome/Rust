//! `Airbnb` 结构体及其 trait 实现。

use super::{Accommodation, StayPolicy};

/// 民宿：用 `Vec<(姓名, 晚数)>` 保留多次预订顺序。
#[derive(Debug)]
pub struct Airbnb {
    name: String,
    availability: bool,
    guests: Vec<(String, u32)>,
}

impl Airbnb {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            availability: true,
            guests: Vec::new(),
        }
    }
}

impl StayPolicy for Airbnb {
    fn max_nights_per_booking(&self) -> u32 {
        30
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
