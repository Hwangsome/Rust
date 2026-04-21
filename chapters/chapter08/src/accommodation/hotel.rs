//! `Hotel` 结构体及其 trait 实现。

use std::collections::HashMap;
use super::{Accommodation, StayPolicy};

/// 酒店：用 `HashMap<姓名, 晚数>` 存预订记录；同名再次预订会覆盖旧值。
#[derive(Debug)]
pub struct Hotel {
    name: String,
    availability: bool,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            availability: true,
            reservations: HashMap::new(),
        }
    }
}

impl StayPolicy for Hotel {
    fn max_nights_per_booking(&self) -> u32 {
        14
    }
}

impl Accommodation for Hotel {
    fn description(&self) -> &str {
        &self.name
    }

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
