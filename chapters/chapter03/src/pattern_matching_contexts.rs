// 这一节要说明：模式匹配并不只存在于 `match`。
// `if let`、`while let`、解构赋值和函数参数，本质上都在使用模式。
fn print_coords((x, y): (i32, i32)) {
    println!("coords from function parameter = ({x}, {y})");
}

pub fn run() {
    println!("== Pattern Matching Contexts ==");

    // 1. 最经典的模式匹配场景：`match`
    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // 2. `if let` 适合只关心某一个分支的场景。
    let maybe_value = Some(5);
    if let Some(value) = maybe_value {
        println!("if let matched = {value}");
    }

    // 3. `while let` 适合“匹配成功就继续循环”的场景。
    let mut stack = vec![1, 2, 3];
    while let Some(value) = stack.pop() {
        println!("while let popped = {value}");
    }

    // 4. `let` 本身也能做解构。
    let (a, b) = (10, 20);
    println!("let destructuring => a = {a}, b = {b}");
    print_coords((5, 8));
    println!();
}
