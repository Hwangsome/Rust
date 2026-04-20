//! 合并多种错误类型：把 `ParseIntError` + `std::io::Error` 归一化到一个自定义 enum。
//!
//! 一个函数里用了多个 `?`，每个 `?` 可能抛出不同类型的错误。如何把它们"塞进同一个返回值"？
//!
//! 三种做法：
//!
//! 1. **自定义 enum + 手动 `map_err`**（本节示范）：最显式、最灵活
//! 2. **自定义 enum + `#[from]` derive**：`thiserror` 生成 `From` 让 `?` 自动转换（见 topic_08）
//! 3. **直接用 `Box<dyn Error>` / `anyhow::Error`**：省心但丢了精确类型（见 topic_07）
//!
//! 选择标准：
//! - **库代码** → 精确 enum，调用方可以按 variant 做分支恢复
//! - **应用/脚本代码** → `anyhow` 常常更合适
//!
//! 本节用最朴素的方式（手写 enum + `map_err`）把原理讲清楚。

#[derive(Debug)]
enum TemperatureError {
    Sensor(u8),
    Conversion(String),
}

fn temperature_from_sensor(sensor_id: u32) -> Result<f64, u8> {
    match sensor_id {
        1 => Ok(25.0),
        2 => Ok(30.5),
        _ => Err(42),
    }
}

fn convert_to_fahrenheit(celsius: f64) -> Result<f64, String> {
    if (-100.0..=100.0).contains(&celsius) {
        Ok(celsius * 1.8 + 32.0)
    } else {
        Err("temperature out of supported range".to_string())
    }
}

fn get_temperature_fahrenheit(sensor_id: u32) -> Result<f64, TemperatureError> {
    let temp_celsius = temperature_from_sensor(sensor_id).map_err(TemperatureError::Sensor)?;
    let temp_fahrenheit =
        convert_to_fahrenheit(temp_celsius).map_err(TemperatureError::Conversion)?;
    Ok(temp_fahrenheit)
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Multiple Error Types ==");

    println!("sensor 1 => {:?}", get_temperature_fahrenheit(1));
    match get_temperature_fahrenheit(99) {
        Ok(value) => println!("sensor 99 => {}", value),
        Err(TemperatureError::Sensor(code)) => {
            println!("sensor 99 => sensor failure code {}", code)
        }
        Err(TemperatureError::Conversion(message)) => {
            println!("sensor 99 => conversion error: {}", message);
        }
    }
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 	        Multiple Error Types
// -------------------------------------------

// Approach 1: With map_err()
fn temperature_from_sensor(sensor_id: u32) -> Result<f64, u8> {
    match sensor_id {
        1 => Ok(25.0),
        2 => Ok(30.5),
        _ => Err(42),
    }
}

fn convert_to_fahrenheit(celsius: f64) -> Result<f64, String> {
    if celsius < -100.0 || celsius > 100.0 {
        Err(String::from("Out of range"))
    } else {
        Ok(celsius * 1.8 + 32.0)
    }
}
enum TemperatureError {
    Sensor(u8),
    Conversion(String),
}

fn get_temperature_fahrenheit(sensor_id: u32) -> Result<f64, TemperatureError> {
    let temp_celsius = temperature_from_sensor(sensor_id).map_err(TemperatureError::Sensor)?;
    let temp_fahrenheit =
        convert_to_fahrenheit(temp_celsius).map_err(TemperatureError::Conversion)?;
    Ok(temp_fahrenheit)
}
fn main() {}

// Approach 2:
// fn temperature_from_sensor(sensor_id: u32) -> Result<f64, TemperatureError> {
//     match sensor_id {
//         1 => Ok(25.0),
//         2 => Ok(30.5),
//         _ => Err(TemperatureError::Sensor(42)),
//     }
// }

// fn convert_to_fahrenheit(celsius: f64) -> Result<f64, TemperatureError> {
//     if celsius < -100.0 || celsius > 100.0 {
//         Err(TemperatureError::Conversion(String::from("Out of range")))
//     } else {
//         Ok(celsius * 1.8 + 32.0)
//     }
// }
// enum TemperatureError {
//     Sensor(u8),
//     Conversion(String),
// }

// fn get_temperature_fahrenheit(sensor_id: u32) -> Result<f64,
// TemperatureError> {     let temp_celsius =
// temperature_from_sensor(sensor_id)?;     let temp_fahrenheit =
// convert_to_fahrenheit(temp_celsius)?;     Ok(temp_fahrenheit)
// }
// fn main() {}
// // Recap: If ? is used multiple times in function, all the error variants
// should have the same type.

/*

-------------------------------------------------------------------------------------------------------------
Concept/Topic            | Explanation
-------------------------|-----------------------------------------------------------------------------------
? Operator Requirement   | All propagated errors in a function using ? must resolve to a single error type.

Approach 1:              | This approach converts different error types at call sites into a common error type.
map_err                  | It solves incompatibility by mapping each error to a variant of a unified error enum.
                         | The function signatures are not changed in this approach.

Approach 2:              | This approach changes function signatures to return the same custom error type.
Consistent return values | It removes the need for map_err but can lead to more repetitive code.
-----------------------------------------------------------------------------------
*/
"###;
