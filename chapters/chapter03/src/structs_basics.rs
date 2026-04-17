// 这一节只讲 struct 最核心的作用：把相关字段打包成一个更有语义的整体。
struct User {
    username: String,
    active: bool,
}

pub fn run() {
    println!("== Structs Basics ==");

    // 这里直接用字面量初始化，方便读者先认识字段名和字段值的对应关系。
    let user = User {
        username: String::from("bill"),
        active: true,
    };

    println!("username = {}", user.username);
    println!("active = {}", user.active);
    println!();
}
