//! 第一个 Rust 程序：从"能跑起来"开始建立信心。
//!
//! 这一节的目标不是 `Hello, world!` 本身，而是让你确认以下几件事都成立：
//! 1. 代码能被 `cargo run -p chapter01` 找到并执行
//! 2. `main.rs` 能调用当前模块的 `run()`
//! 3. 我们看到的输出来自真正的二进制，不是文档里的截图
//! 4. `println!` 后面那个 `!` 表示它是**宏**，不是普通函数
//! 5. 标准输出（stdout）和标准错误（stderr）是两条不同的流

/// 宏 vs 函数：`println!` 名字末尾的 `!` 是**强制提示**——
/// 它不是函数调用，而是编译期展开的宏。
/// 这个区别在后面学自定义宏、错误信息时会反复出现。
fn demonstrate_println_is_a_macro() {
    // 最小、最经典的一行 Rust 程序。
    println!("Hello, world!");

    // 宏和函数的一个直观差异：宏可以接收任意数量、任意类型的参数，
    // 而普通函数的参数数量和类型都是固定的。
    println!("Hello, {} and {}!", "world", 42);
}

/// 标准输出 vs 标准错误：看起来一样，但它们是两条不同的流。
///
/// 在终端里 `./chapter01` 会把两条流都打到屏幕上；
/// 如果运行 `./chapter01 > out.txt`，你会发现：
/// - `println!` 的内容进了 `out.txt`
/// - `eprintln!` 的内容仍然留在屏幕上
///
/// 这是所有命令行工具（比如 cargo 本身）在底层的标准做法。
fn demonstrate_stdout_vs_stderr() {
    println!("[stdout] 这条由 println! 打到标准输出");
    eprintln!("[stderr] 这条由 eprintln! 打到标准错误");
}

/// `print!` vs `println!`：一个换行，一个不换行。
///
/// 在做进度条、做 REPL、做行末刷新的时候，你常常需要 `print!` + 手动 `flush`。
fn demonstrate_print_vs_println() {
    // print! 不加换行，所以下面两次输出会接在一起。
    print!("no newline here, ");
    print!("still same line");
    // 最后再显式加一个换行，让后续输出整洁一些。
    println!();
}

pub fn run() {
    println!("-- (1) println! 是宏，注意末尾的 `!` --");
    demonstrate_println_is_a_macro();
    println!();

    println!("-- (2) stdout vs stderr 是两条不同的流 --");
    demonstrate_stdout_vs_stderr();
    println!();

    println!("-- (3) print! 不换行 / println! 自动换行 --");
    demonstrate_print_vs_println();
    println!();
}
