// 这个文件演示组合子：先描述“想做什么转换”，最后再用消费者把结果取出来。
// 运行时要观察：`filter` 和 `map` 在 `collect` 之前都只是搭管道，不会立刻产出最终集合。
// 这正是迭代器“惰性求值”的特征。
pub fn run() {
    println!("== Combinators ==");

    let numbers = [1, 2, 3, 4, 5, 6];
    let even_squares: Vec<i32> = numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .map(|number| number * number)
        .collect();

    println!("filter + map + collect => {:?}", even_squares);
    println!();
}
