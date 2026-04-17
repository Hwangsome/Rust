// `Result<T, E>` 表示“成功时拿到 T，失败时拿到 E”。
// 它是 Rust 错误处理体系里的核心类型之一。
pub fn run() {
    println!("== Result ==");

    // `parse()` 成功时给 `Ok(42)`，失败时给一个错误值。
    let parsed: Result<i32, _> = "42".parse();

    match parsed {
        Ok(value) => println!("parsed successfully = {value}"),
        Err(error) => println!("parse error = {error}"),
    }
    println!();
}
