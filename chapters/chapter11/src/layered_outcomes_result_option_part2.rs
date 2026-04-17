// 这个文件演示 `Option<Result<T, E>>`：操作本身可以“不发生”，如果发生了，再区分成功或失败。
// 运行时要观察：外层 Option 表达“有没有尝试”，内层 Result 表达“尝试后成没成功”。
// 这类形状适合可选表单字段、按条件执行的解析逻辑。
use std::num::ParseIntError;

fn handle_user_registration(
    name: &str,
    age_input: Option<&str>,
) -> Option<Result<u32, ParseIntError>> {
    println!("registering user: {name}");
    age_input.map(|value| value.parse::<u32>())
}

pub fn run() {
    println!("== Layered Outcomes: Option<Result<T, E>> ==");

    println!(
        "age provided and valid => {:?}",
        handle_user_registration("Alice", Some("25"))
    );
    println!(
        "age provided but invalid => {:?}",
        handle_user_registration("Bob", Some("twenty"))
    );
    println!(
        "age not provided => {:?}",
        handle_user_registration("Carol", None)
    );
    println!();
}
