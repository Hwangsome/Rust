// 这个文件演示“具体生命周期”如何出现在实际代码里。
// 运行时要观察：虽然 `first` 还写在同一个作用域里，但它的最后一次使用结束后，借用就算结束了。
// 这就是非词法生命周期（NLL）让代码更灵活的地方。
pub fn run() {
    println!("== Concrete Lifetimes ==");

    let mut values = vec![10, 20, 30];
    let first = &values[0];
    println!("borrowed first value => {}", first);

    // 这里 push 之所以合法，是因为 `first` 的最后一次使用已经结束。
    values.push(40);
    println!("after borrow ends => {:?}", values);
    println!();
}
