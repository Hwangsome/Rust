// 练习目标不是背定义，而是把“泛型、trait、trait bound”连起来使用。
// 建议先自己写，再对照本章模块看哪些地方必须交给编译器做静态约束。
pub fn run() {
    println!("== Lab ==");
    println!("1. 写一个泛型 Pair<T>，并实现 new()");
    println!("2. 定义一个 trait，让两个不同 struct 都实现它");
    println!("3. 写一个只接收实现了该 trait 的泛型函数");
    println!("4. 试着把不同类型放进 Vec<Box<dyn Trait>>");
    println!("5. 为一个 trait 改写成带关联类型的版本，比较可读性");
    println!();
}
