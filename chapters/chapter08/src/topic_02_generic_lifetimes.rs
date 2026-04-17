// 这个文件演示显式生命周期参数：它不是延长引用寿命，而是描述多个引用之间的关系。
// 运行时要观察：`longer` 返回的引用，其生命周期只能和两个输入里较短的那个一样长。
// 所以生命周期标注更像“约束说明”，不是“内存保活开关”。
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub fn run() {
    println!("== Generic Lifetimes ==");

    let title = String::from("Rust");
    let topic = "ownership";
    let chosen = longer(title.as_str(), topic);

    println!("longer text => {}", chosen);
    println!(
        "'static example => {}",
        "string literal lives for the whole program"
    );
    println!();
}
