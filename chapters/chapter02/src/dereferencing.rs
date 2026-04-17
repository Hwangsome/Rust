// 解引用是“从引用回到底层值”的显式动作。
// Rust 不希望这一步完全隐身，因为它关系到你到底在操作值还是操作引用。
pub fn run() {
    println!("== Dereferencing ==");

    let value = 42;
    let reference = &value;

    // 第一行打印的是“引用看起来像一个值”，第二行强调真正的解引用写法是 `*reference`。
    println!("reference 自身是一个引用: {reference}");
    println!("*reference 解引用后得到: {}", *reference);
    println!();
}
