//! 第 1 章练习说明。
//!
//! 本章是"Quick Startup"，练习的目的是让你**把手放到键盘上**：
//! - 尽量亲自制造每一种常见报错，再把它修好
//! - 每改一步就 `cargo run -p chapter01` 重新跑一次
//! - 把"输出变化"和"改动"一一对上
//!
//! 别怕编译失败——Rust 的报错比大多数语言都友好，读懂它本身就是学习。

pub fn run() {
    println!("▷ 练习 1：Hello 与 println!");
    println!("  - 在 topic_01_first_program.rs 里用你的名字替换 `Hello, world!`");
    println!("  - 再分别用 `print!` / `println!` / `eprintln!` 各打一行，观察行为差异");

    println!();

    println!("▷ 练习 2：不可变 vs mut vs shadowing");
    println!("  - 写 `let x = 5; x = 6;`，观察 E0384，再改用 `mut` 修复");
    println!("  - 写 `let mut y = 5; y = \"hi\";`，观察 E0308");
    println!("  - 写 `let s = \"   \"; let s = s.len();`，说出两个 `s` 的类型分别是什么");

    println!();

    println!("▷ 练习 3：原生类型与整数溢出");
    println!("  - 尝试 `let x: u8 = 256;`，看编译器如何当场阻止");
    println!("  - 用 `u8::MAX.checked_add(1)` 得到 `None`；再试 `wrapping_add(1)` 得到 `0`");

    println!();

    println!("▷ 练习 4：元组与数组");
    println!("  - 写一个 `(&str, i32, bool)` 元组并解构出来");
    println!("  - 写 `let a = [1, 2, 3]; println!(\"{{}}\", a[5]);`，观察运行时 panic");
    println!("  - 改成 `a.get(5)` 得到 `None`，体会安全接口的价值");

    println!();

    println!("▷ 练习 5：函数与代码块");
    println!("  - 写一个 `fn max3(a: i32, b: i32, c: i32) -> i32`");
    println!("  - 把返回值从尾表达式改成 `return ...;`，再改回去");
    println!("  - 故意在尾表达式后加分号，观察 E0308 告诉你返回成了 `()`");

    println!();

    println!("▷ 练习 6：控制流");
    println!("  - 用 `loop {{ ... break value; }}` 写一个返回值的循环");
    println!("  - 写带 `'outer:` 标签的嵌套循环，学会跳出整层");

    println!();

    println!("▷ 练习 7：打印格式化");
    println!("  - 用 `{{:>10}}` / `{{:<10}}` / `{{:^10}}` 对齐数字");
    println!("  - 用 `{{:.3}}` / `{{:08.3}}` 控制浮点精度与补零");
    println!("  - 用 `{{:b}} / {{:o}} / {{:x}}` 打印不同进制");

    println!();

    println!("▷ 练习 8：const / static / 命名约定");
    println!("  - 用 SCREAMING_SNAKE_CASE 定义 `const MAX_USERS: u32 = 100;`");
    println!("  - 定义一个 `static GREETING: &str`，再尝试把它打印出来");

    println!();

    println!("▷ 练习 9：错误码查询");
    println!("  - 故意造一个 `let x: i32 = \"abc\";`，观察 E0308");
    println!("  - 运行 `rustc --explain E0308`，读完它给的详细解释");

    println!();

    println!("▷ 练习 10：运算符 / 结合性 / 优先级");
    println!("  - 用 `17 / 5`、`17 % 5`、`-7 % 3` 观察整数除与求余");
    println!("  - 写 `-(20 - 5 - 3)` 与 `-(20 - (5 - 3))` 对比");
    println!("  - 给 `Point` 再实现一个 `Sub`，让 `p1 - p2` 能跑起来");

    println!();

    println!("完成标准：");
    println!("  - 能在不开文档的情况下读懂 E0308 / E0384 / E0382 / E0502");
    println!("  - 能对着一段 Rust 代码说出：这是不是表达式？它的返回类型是什么？");
    println!("  - 能解释 let / const / static 各自的使用场景");
}
