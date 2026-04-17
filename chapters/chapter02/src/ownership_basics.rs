// 这里先只讲“赋值时会发生什么”，不急着把函数调用也混进来。
// 初学者先搞清楚 move / clone / Copy 的区别，后面看函数边界会容易很多。
pub fn run() {
    println!("== Ownership Basics ==");

    // `String` 持有堆数据，所以我们用 `clone` 明确做一次深拷贝。
    let s1 = String::from("world");
    let s2 = s1.clone();

    println!("clone 后两个 String 都可用: s1 = {s1}, s2 = {s2}");

    // `i32` 这类 Copy 类型赋值时不会让原变量失效。
    let x = 15;
    let y = x;
    println!("Copy 类型赋值后原值仍可用: x = {x}, y = {y}");
    println!("String 这类堆分配值默认会 move；想保留原值时要显式 clone。");
    println!();
}
