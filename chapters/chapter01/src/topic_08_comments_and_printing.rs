//! 注释与打印：学习阶段最常用的两件事。
//!
//! 这一节覆盖：
//! - 注释种类：`//` 行注释、`/* ... */` 块注释、`///` 文档注释、`//!` 内层文档注释
//! - 打印宏：`print!` / `println!` / `eprint!` / `eprintln!`
//! - 格式化占位符：`{}`（Display）/ `{:?}`（Debug）/ `{:#?}`（pretty Debug）
//! - 宽度 / 对齐 / 精度 / 补零 / 进制 / 科学计数
//! - 命名参数 / 位置参数 / 捕获参数（Rust 2021 起可以直接写 `{var}`）
//!
//! 运行后请观察每个输出"前缀"和"对齐效果"。

/// 1) 注释种类简介（全部通过输出说明，不是代码演示）。
fn demonstrate_comment_kinds() {
    // 这一行是普通 // 行注释。
    /* 这是块注释，可以跨多行，但不能嵌套另一个块注释（会报错）。 */

    // /// 是文档注释，写在 item 的**前面**（比如函数、struct 之前）。
    //     它会被 cargo doc 提取，生成 HTML 文档。
    // //! 是"内层文档注释"，通常写在**文件顶部**或**模块起始处**，
    //     用来描述"**这个文件/模块本身**"而不是紧接着的那个 item。

    println!(
        "本章采用: `//` 行注释 + 文件顶部 `//!` 模块说明 + 关键 item 前 `///` 文档注释。"
    );
}

/// 2) `print!` / `println!` / `eprint!` / `eprintln!` 对照。
fn demonstrate_print_family() {
    print!("print! 不换行 -> ");
    println!("println! 换行");

    // 两条 eprintln! 把同样的信息送到 stderr，适合"错误、警告、诊断"。
    eprintln!("eprintln! 也换行，但走 stderr");
}

/// 3) 最常见的三种格式化占位符：`{}`、`{:?}`、`{:#?}`。
fn demonstrate_display_vs_debug() {
    let number = 42;
    let vector = vec![1, 2, 3];

    // `{}` 使用类型的 Display 实现——给人看的最终文本
    println!("Display: {number}");

    // `{:?}` 使用类型的 Debug 实现——给开发者看的诊断文本
    println!("Debug:   {vector:?}");

    // `{:#?}` 是 pretty Debug，对嵌套结构自动换行 / 缩进
    println!("Pretty Debug: {vector:#?}");
}

/// 4) 格式化修饰：宽度、对齐、填充、精度、补零、进制。
fn demonstrate_format_modifiers() {
    let n = 42;
    let pi = std::f64::consts::PI;

    // 宽度 + 右对齐（默认数字右对齐）
    println!("|{n:6}|");
    // 宽度 + 左对齐
    println!("|{n:<6}|");
    // 宽度 + 居中
    println!("|{n:^6}|");
    // 宽度 + 用特定字符填充
    println!("|{n:*^6}|");
    // 补零
    println!("|{n:06}|");
    // 保留小数位数
    println!("pi = {pi:.4}");
    // 宽度 + 精度
    println!("pi = {pi:10.4}");
    // 进制：二进制 / 八进制 / 十六进制（lower/upper）
    println!("255 = 0b{n:b}, 0o{n:o}, 0x{n:x}, 0X{n:X}", n = 255);
}

/// 5) 参数传递方式：位置参数、命名参数、捕获参数。
fn demonstrate_argument_styles() {
    let language = "Rust";
    let version = 2024;

    // 位置参数：按出现顺序匹配
    println!("位置参数: {} {}", language, version);

    // 命名参数
    println!("命名参数: {lang} {ver}", lang = language, ver = version);

    // 捕获参数（Rust 2021+）：`{name}` 会自动去当前作用域找同名变量
    println!("捕获参数: {language} {version}");
}

pub fn run() {
    println!("-- (1) 注释种类 --");
    demonstrate_comment_kinds();
    println!();

    println!("-- (2) print! 家族 --");
    demonstrate_print_family();
    println!();

    println!("-- (3) Display vs Debug --");
    demonstrate_display_vs_debug();
    println!();

    println!("-- (4) 格式化修饰 --");
    demonstrate_format_modifiers();
    println!();

    println!("-- (5) 参数传递方式 --");
    demonstrate_argument_styles();
    println!();
}
