// 这个文件演示函数指针：它只接受“真正的函数项”，或者能退化为函数指针的零捕获闭包。
// 运行时要观察：普通函数和零捕获闭包都能传给 `apply_twice`，但捕获环境的闭包不行。
// 这能帮助区分 `fn` 和 `Fn` 族 trait 的边界。
fn add_one(value: i32) -> i32 {
    value + 1
}

fn apply_twice(value: i32, operation: fn(i32) -> i32) -> i32 {
    operation(operation(value))
}

pub fn run() {
    println!("== Function Pointers ==");

    let double = |value| value * 2;

    println!("function pointer => {}", apply_twice(3, add_one));
    println!("zero-capture closure => {}", apply_twice(3, double));
    println!();
}
