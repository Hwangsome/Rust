// 这一节把“变量默认不可变”“mut”“遮蔽”放到同一个最小例子里。
// 初学者运行后，应该重点观察三件事：
// 1. 同一个绑定默认不能随便改
// 2. `mut` 是显式允许修改
// 3. 遮蔽不是修改旧值，而是创建同名新绑定
pub fn run() {

    // `language` 没有写 `mut`，所以这个绑定默认不可变。
    let language = "Rust";
    println!("默认不可变变量 language = {language}");

    // `score` 写了 `mut`，表示同一个绑定后面允许被重新赋值。
    let mut score = 60;
    println!("修改前的 score = {score}");
    score = 80;
    println!("修改后的 score = {score}");

    // 这里不是在“修改字符串本身”，而是用同名变量创建了一个新的绑定。
    // 第一个 `spaces` 是字符串，第二个 `spaces` 是长度值。
    let spaces = "   ";
    let spaces = spaces.len();
    println!("经过遮蔽后，spaces 变成长度值 = {spaces}");
    println!();
}
