pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：用 newtype 给 Vec<String> 实现 Display");
    println!("▷ 练习 2：定义 `trait ReadLog: Read + Clone {{}}`，观察组合 trait 的价值");
    println!("▷ 练习 3：尝试给 Box<dyn Iterator> 指定 Item 类型");
    println!("▷ 练习 4：写 object-safe 的 trait，里面混入一个 `where Self: Sized` 的默认方法");
    println!("▷ 练习 5：给自定义 struct 实现 `Add` / `Sub`，用 `+` 调用");
    println!("▷ 练习 6：用 sealed trait 模式写一个第三方看不到的 trait");
    println!();
}
