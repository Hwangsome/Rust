// 这个文件演示：一个函数里如果连续用了多个 `?`，这些错误最终必须汇总到同一种类型上。
// 运行时要观察：`map_err` 的作用不是“把错误藏起来”，而是显式转换到统一错误枚举。
// 这一步在服务边界清晰的应用代码里非常常见。
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
