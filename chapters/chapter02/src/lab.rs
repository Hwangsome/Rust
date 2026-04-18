//! 第 2 章练习说明。
//!
//! 本章的练习与前面章节不同：你必须亲手去**制造编译错误**，再把它修好。
//! 借用检查器的报错信息本身就是最好的学习材料——
//! 别急着删代码、别急着加 `clone()`，先读一读错误提示。
//!
//! 每做完一小步就重新 `cargo run -p chapter02`，让“改动”和“输出变化”对上。

pub fn run() {
    println!("== Lab ==");

    println!("练习 1：制造一个 String 的 move 错误");
    println!("  - 写 `let s1 = String::from(\"hi\"); let s2 = s1; println!(\"{{}}\", s1);`");
    println!("  - 观察编译器报的 E0382，记下提示里 `note:` 开头的那一行");
    println!("  - 分别用 clone、借用、调换变量顺序 三种方式修好它");

    println!();

    println!("练习 2：写一对“吃掉所有权 / 归还所有权”的函数");
    println!("  - fn take(v: Vec<i32>)    // 吃掉");
    println!("  - fn give() -> Vec<i32>   // 归还");
    println!("  - 再写 fn take_and_give(v: Vec<i32>) -> Vec<i32>");
    println!("  - 观察：第三种写法什么时候反而不如直接用 &mut Vec<i32>");

    println!();

    println!("练习 3：触发并修复借用冲突");
    println!("  - 先同时创建一个 &T 和一个 &mut T，观察 E0502");
    println!("  - 再把两者放到不重叠的区间里，让它编译通过");
    println!("  - 最后去掉所有大括号，依靠 NLL 让它也编译通过");

    println!();

    println!("练习 4：把“按值返回”改写成“按引用接收”");
    println!("  - 先写 fn add_and_return(v: Vec<i32>) -> Vec<i32> {{ ... }}");
    println!("  - 再改写成 fn add_in_place(v: &mut Vec<i32>) {{ ... }}");
    println!("  - 对比两种签名下调用方写法的变化");

    println!();

    println!("练习 5：解引用的读与写");
    println!("  - 定义 let mut n = 0;，再定义 let r = &mut n;");
    println!("  - 用 *r = 10; 写入一次");
    println!("  - 用 println!(\"{{}}\", *r); 读取一次");
    println!("  - 去掉 *，观察编译器提示“你是不是想解引用”");

    println!();

    println!("练习 6：&[T] vs &Vec<T>");
    println!("  - 写 fn sum_vec(v: &Vec<i32>) -> i32，再调用它");
    println!("  - 改成 fn sum(v: &[i32]) -> i32，再调用同一个 Vec");
    println!("  - 给新签名再喂一个数组 `[10, 20, 30]`，看看是不是自动兼容");

    println!();

    println!("完成标准：");
    println!("  - 能分别复现 E0382、E0499、E0502 三种典型报错");
    println!("  - 能只靠“改签名”解决一半以上的借用问题，而不是每次都 clone");
    println!("  - 读到 `*r` 能立刻说出：这里是读取还是写入底层值？");

    println!();
}
