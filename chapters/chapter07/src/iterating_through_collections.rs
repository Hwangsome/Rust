// 这个文件把 `iter`、`iter_mut`、`into_iter` 放到同一处对比。
// 运行时要观察：三个方法都能“遍历”，但它们对所有权和可变性的影响完全不同。
// 这也是为什么很多集合 API 看着像一套命名，行为却有明显边界。
use std::collections::HashMap;

pub fn run() {
    println!("== Iterating Through Collections ==");

    let numbers = vec![1, 2, 3];
    let borrowed_sum: i32 = numbers.iter().sum();
    println!("iter keeps ownership => sum = {}", borrowed_sum);

    let mut scores = vec![10, 20, 30];
    for score in &mut scores {
        *score += 5;
    }
    println!("iter_mut updates in place => {:?}", scores);

    let consumed: Vec<i32> = scores.into_iter().map(|score| score * 2).collect();
    println!("into_iter consumes collection => {:?}", consumed);

    let mut status = HashMap::new();
    status.insert("build", "ok");
    status.insert("test", "ok");

    for (step, result) in &status {
        println!("{step} => {result}");
    }
    println!();
}
