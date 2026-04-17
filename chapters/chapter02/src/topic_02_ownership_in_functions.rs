// 这一节把“函数调用也会形成所有权边界”拆成三个小问题：
// 1. 函数拿走值
// 2. 函数返回值
// 3. 函数拿走后再还回来
fn takes_ownership(vec: Vec<i32>) {
    println!("function took ownership of vec: {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    // 这里在函数内部修改后再把所有权还给调用方。
    vec.push(10);
    vec
}

fn stack_function(value: i32) {
    println!("stack copy inside function = {value}");
}

pub fn run() {
    println!("== Ownership In Functions ==");

    // 这里先 clone，一方面保持例子可运行，另一方面顺便强调 clone 是显式动作。
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());
    println!("clone 后，调用方仍可继续使用 vec_1: {:?}", vec_1);

    // 返回值把新的所有权带回到调用方作用域。
    let vec_2 = gives_ownership();
    println!("function gave ownership back: {:?}", vec_2);

    // `vec_2` 在这里被移动进函数，返回后新的所有者是 `vec_3`。
    let vec_3 = takes_and_gives_ownership(vec_2);
    println!("after take and return, new owner is vec_3: {:?}", vec_3);

    // Copy 类型经过函数调用后，调用方原值仍然可用。
    let number = 10;
    stack_function(number);
    println!("Copy type after function call still available: {number}");
    println!();
}
