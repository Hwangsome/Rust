// 这里把“函数”和“代码块返回值”放到一个最小示例里。
// 初学者最需要观察的是：Rust 不一定非要写 `return`，代码块本身也能产生值。
fn add(a: i32, b: i32) -> i32 {
    // 尾表达式没有分号，所以它会作为返回值。
    a + b
}

pub fn run() {

    // 这是最普通的函数调用。
    let result = add(2, 3);
    println!("2 + 3 = {result}");

    // 这个代码块最后一行没有分号，所以整个代码块的值是 `base + 1`。
    let computed_value = {
        let base = 5;
        base + 1
    };

    println!("computed_value = {computed_value}");
    println!();
}
