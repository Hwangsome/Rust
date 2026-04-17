// `lab.rs` 不直接给答案，而是提醒读者如何自己动手验证这一章的主题。
// 这里的练习顺序跟着 `main.rs` 的运行顺序走，
// 同时和带编号的 `topic_XX_*.rs` 文件名保持一致。
pub fn run() {
    println!("1. 用 cargo 运行 chapter01，并观察每个主题的输出顺序");
    println!("2. 修改 topic_01_first_program.rs，让第一个程序输出自己的名字");
    println!("3. 在 topic_02_variables.rs 里补一个 shadowing 示例并重新运行");
    println!("4. 在 topic_05_functions_and_code_blocks.rs 里新增一个返回值函数");
    println!("5. 在 topic_07_conditionals_control_flow_and_loops.rs 里补一个新的 for range 例子");
    println!("6. 在 topic_08_comments_and_printing.rs 里尝试不同的 println! 格式字符串");
    println!("7. 写一个 const 和一个 static，并比较它们的使用位置");
    println!("8. 手动制造一个类型错误，再用 rustc --explain 查看错误码说明");
    println!("9. 写出一个需要括号才能得到不同结果的表达式");
    println!();
}
