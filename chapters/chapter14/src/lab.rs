//! 第 13 章练习：识别代码里"偷偷发生的 coercion"。

pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：写 `fn f(s: &str)` 然后传 &String、&Box<String>、&Rc<String>，全部能通过");
    println!("▷ 练习 2：在 if/else 里用 &A 和 &B 两个不同类型，观察 E0308；改用 `as &dyn Trait` 让它通过");
    println!("▷ 练习 3：对比 `fn f(x: i32)` 和 `fn g<T>(x: T)`——泛型上没有 coercion");
    println!("▷ 练习 4：测试 `Box<Vec<i32>>` 在传给 `fn f(xs: &[i32])` 时发生的完整 coercion 链");
    println!("▷ 练习 5：用 `let s: &str = &String::from(\"x\");` 观察 let 带类型标注位置的 coercion");
    println!();
    println!("完成标准：");
    println!("  - 能指出一段代码里所有 coercion 发生的具体位置");
    println!("  - 能说清为什么泛型位置不触发 coercion");
    println!();
}
