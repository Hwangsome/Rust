// 这一章的练习重点是：不要只看“能不能初始化成功”，还要看 API 是否好用、结构是否容易维护。
// 如果一个 struct 让你总想写很多重载构造函数，通常就是 builder 或结构拆分该上场了。
pub fn run() {
    println!("== Lab ==");
    println!("1. 为一个带校验的 struct 写 new()，让它返回 Result");
    println!("2. 给字段较多的 struct 设计一个 builder");
    println!("3. 尝试用 ..Default::default() 只覆盖少数字段");
    println!("4. 把一个大 struct 拆成两个子 struct，再写两个分别处理它们的函数");
    println!("5. 观察拆分后哪些借用冲突自然消失了");
    println!();
}
