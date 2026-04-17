// 这一节把借用放进函数边界里观察。
// 与“拿走所有权”不同，这里函数只临时使用调用方的值。
fn borrows_vec(vec: &[i32]) {
    println!("immutably borrowed vec = {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    // 可变借用允许函数直接修改调用方拥有的数据。
    vec.push(10);
}

fn mixed_borrows(subject: &str, scores: &mut Vec<i32>) {
    println!("{subject} before update: {:?}", scores);
    scores.push(99);
    println!("{subject} after update: {:?}", scores);
}

pub fn run() {
    println!("== Borrowing In Functions ==");

    let mut vec_1 = vec![1, 2, 3];

    // 先只读借用，再可变借用，这样更容易看出两种函数签名的区别。
    borrows_vec(&vec_1);
    mutably_borrows_vec(&mut vec_1);
    println!("vec_1 after mutable borrow = {:?}", vec_1);

    // 一个常见模式是：用不可变借用拿上下文，用可变借用改状态。
    let mut scores = vec![85, 90, 85];
    mixed_borrows("Math", &mut scores);
    println!();
}
