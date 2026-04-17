// 这个文件演示闭包最重要的两个点：它既像函数，又能捕获外部环境。
// 运行时要观察：`min_length` 并没有作为参数显式传进去，但闭包仍然能使用它。
// 这就是闭包和普通函数最大的区别。
fn validate_user<F>(name: &str, validator: F) -> bool
where
    F: Fn(&str) -> bool,
{
    validator(name)
}

pub fn run() {
    println!("== Closures ==");

    let min_length = 4;
    let is_valid_user =
        |name: &str| name.len() >= min_length && name.chars().all(char::is_alphabetic);

    println!("Alice valid => {}", validate_user("Alice", is_valid_user));
    println!("Bo valid => {}", validate_user("Bo", is_valid_user));
    println!();
}
