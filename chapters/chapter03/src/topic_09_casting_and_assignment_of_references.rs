// 这一节说明两件容易混淆的事：
// 1. 可变引用可以临时“借出一个只读视图”
// 2. 不同种类的引用在赋值时，行为并不完全一样
pub fn run() {
    println!("== Casting And Assignment Of References ==");

    let mut data = 42;
    let mutable_ref = &mut data;

    // 这里把 `&mut i32` 临时看成 `&i32`，这类临时只读借用常被称为 reborrow。
    let immutable_view: &i32 = &*mutable_ref;
    println!("immutable view from mutable ref = {immutable_view}");
    *mutable_ref = 43;
    println!("mutable ref after reborrow ends = {}", *mutable_ref);

    // 不可变引用可以同时存在多个，所以赋值给另一个变量时更像“复制引用”。
    let text = String::from("Rust");
    let ref_text_1 = &text;
    let ref_text_2 = ref_text_1;
    println!("immutable references are Copy-like: {ref_text_1}, {ref_text_2}");
    println!();
}
