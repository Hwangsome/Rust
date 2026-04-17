// 这里用最小内嵌模块演示“模块提供命名空间”的概念。
mod greetings {
    pub fn hello() {
        println!("hello from the greetings module");
    }
}

pub fn run() {
    println!("== Modules ==");

    // 调用时要带模块路径，这正是模块树存在的意义之一。
    greetings::hello();
    println!("模块的作用是把相关代码放进同一个命名空间。");
    println!();
}
