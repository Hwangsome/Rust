// 这一节只讨论借用规则本身，不把函数边界混进来。
// 重点观察“多个不可变借用”和“单个可变借用”为什么不能同时存在。
pub fn run() {
    println!("== Borrowing Basics ==");

    let mut values = vec![4, 5, 6];

    // 这里同时创建两个不可变借用是允许的，因为它们都只是只读访问。
    let ref1 = &values;
    let ref2 = &values;
    println!("immutable refs => {:?}, {:?}", ref1, ref2);

    // 当上面的不可变借用不再使用后，再创建一个可变借用。
    let ref3 = &mut values;
    ref3.push(7);
    println!("mutable ref after immutable refs end => {:?}", ref3);
    println!("同一时刻可以有多个不可变借用，或一个可变借用，但不能同时混用。");
    println!();
}
