// 这一节对比“模块内能用”和“模块外能不能用”。
mod school {
    pub fn open_class() {
        // 同一个模块内部可以调用私有函数。
        prepare_room();
        println!("公开函数可以被模块外调用。");
    }

    fn prepare_room() {
        println!("私有函数只能在当前模块内部使用。");
    }
}

pub fn run() {
    println!("== Visibility ==");

    // 模块外只能调用 `pub` 的入口。
    school::open_class();
    println!();
}
