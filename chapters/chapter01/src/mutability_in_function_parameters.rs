// 这一节专门强调：函数参数写 `mut`，只影响函数内部的那个新绑定。
// 它不会自动把调用方的变量“原地改掉”。
fn increase(mut value: i32) -> i32 {
    // 这里修改的是函数内部的参数绑定，不是外部的 `score` 本体。
    value += 1;
    value
}

pub fn run() {
    println!("== Mutability In Function Parameters ==");

    // `score` 传进函数时，因为 `i32` 是 Copy 类型，所以函数里拿到的是一个副本。
    let score = 10;
    let new_score = increase(score);

    println!("调用前的 score = {score}");
    println!("调用后的返回值 new_score = {new_score}");
    println!();
}
