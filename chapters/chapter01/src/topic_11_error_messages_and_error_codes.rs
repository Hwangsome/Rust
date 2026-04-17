// 这里不真的制造编译错误，因为这个 chapter 需要保持可运行。
// 所以我们把“会报错的代码”以字符串形式展示，再告诉读者如何查看错误码说明。
pub fn run() {

    // 这段代码如果真的写进可编译代码里，会得到类型不匹配错误。
    let example = r#"let x: i32 = String::from("Hello");"#;
    println!("示例错误代码片段: {example}");
    println!("这类类型不匹配通常会得到像 E0308 这样的错误码。");
    println!("遇到错误码时，可以运行: rustc --explain E0308");
    println!();
}
