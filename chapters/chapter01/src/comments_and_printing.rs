// 学习阶段最直接的观察工具就是打印。
// 这一节重点不是输出什么内容，而是理解不同打印宏和格式化写法的差异。
pub fn run() {
    println!("== Comments And Printing ==");

    let language = "Rust";
    let section = "Quick Startup";

    // `println!` 打印后会自动换行。
    println!("I am learning {language}");
    println!("Current section = {section}");

    // `print!` 不会自动换行，所以第二句会直接接在第一句后面。
    print!("print! 不会自动换行。");
    print!("所以这一句会接在后面。");
    println!();

    // 命名参数能让格式字符串更清晰，尤其在参数较多时更容易读。
    println!(
        "使用命名参数输出：language = {lang}, section = {sec}",
        lang = language,
        sec = section
    );
    println!();
}
