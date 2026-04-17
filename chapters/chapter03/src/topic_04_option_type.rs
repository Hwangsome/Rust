// `Option<T>` 用来表达“这里可能有值，也可能没有值”。
// 它比空指针式的隐式约定更安全，因为类型里已经把“可能为空”写出来了。
pub fn run() {
    println!("== Option ==");

    let maybe_name: Option<&str> = Some("Rust");

    // 这里用 `match` 把有值和没值两种情况都写出来。
    match maybe_name {
        Some(name) => println!("value inside Some = {name}"),
        None => println!("no value present"),
    }
    println!();
}
