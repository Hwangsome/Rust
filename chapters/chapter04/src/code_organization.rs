// 这一节不做复杂代码演示，而是先把名词边界说清楚。
// 对初学者来说，package / crate / module 这三个词最容易混。
pub fn run() {
    println!("== Code Organization ==");
    println!("package 是最高层组织单位，Cargo.toml 负责描述它。");
    println!("crate 是编译单元，binary crate 产出可执行文件，library crate 产出可复用库。");
    println!("module 用来在 crate 内继续组织命名空间和可见性。");
    println!();
}
