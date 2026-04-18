//! 第 15 章练习：进一步巩固对各种引用 / 指针 / 借用的直觉。

pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：给一个 enum 添加两个不同形态的 &T 字段，用 match 拆出来");
    println!("▷ 练习 2：写一个 `fn owners<'a>(v: &'a Vec<String>) -> Vec<&'a str>`");
    println!("▷ 练习 3：为 `for x in v / &v / &mut v / v.iter() / v.iter_mut()` 各写一段，理解每种的产出类型");
    println!("▷ 练习 4：把 Rc 换成 Arc，观察代码变化");
    println!("▷ 练习 5：用 `*const T` 实现一个 C 风格的函数，观察为什么必须 unsafe");
    println!();
}
