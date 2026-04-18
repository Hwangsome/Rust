//! 第 16 章练习：方差是概念题，实践里真的会遇到。

pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：把一个 &'static str 传给 `fn f<'a>(_: &'a str)`，理解协变");
    println!("▷ 练习 2：尝试把 fn(&str) 赋给 fn(&'static str)——合法，逆变");
    println!("▷ 练习 3：尝试在 &mut Vec<&'a str> 里换不同生命周期的引用，观察 E0308");
    println!("▷ 练习 4：用 PhantomData<*mut T> 让 struct 在 T 上变成不变，做个小实验");
    println!();
    println!("完成标准：");
    println!("  - 能说出 协变 / 逆变 / 不变 的典型例子");
    println!("  - 遇到 生命周期 / &mut 相关的奇怪 error 能联想到方差");
    println!();
}
