//! 模块：命名空间、路径、嵌套、`use` 别名。
//!
//! 模块系统的核心价值不仅是"命名空间"，还有**可见性边界**（下一节细讲）。
//! 本节只关注**怎么把代码拆进模块、怎么引用它们**：
//!
//! - 用 `mod name { ... }` 内联声明模块（常用于小示例）
//! - 用 `mod name;` 在**另一个文件**里定义模块（正式项目更常用）
//! - **绝对路径** `crate::a::b::c` vs **相对路径** `self::x` / `super::y`
//! - `use` 把路径引入当前作用域，配合 `as` 起别名、`{A, B, C}` 一次导入多个

/// 内联模块：和 struct / fn 一样，直接写在当前文件里。
mod greetings {
    // 默认私有，带 `pub` 的才能被模块外访问。
    pub fn hello() {
        println!("hello from the greetings module");
    }

    pub fn goodbye() {
        println!("goodbye from the greetings module");
    }

    // 私有函数：只在本模块内可见。
    fn secret() -> &'static str {
        "🤫 this is private"
    }

    /// 一个 pub 函数可以在内部调用同模块的私有函数。
    pub fn reveal() {
        println!("reveal() 转发调用了私有函数 secret(): {}", secret());
    }
}

/// 嵌套模块：`math::arithmetic::add`。
mod math {
    pub mod arithmetic {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub mod geometry {
        /// 使用 `super::arithmetic::multiply` 访问"上层模块"里的 `arithmetic`。
        pub fn square_area(side: i32) -> i32 {
            super::arithmetic::multiply(side, side)
        }
    }
}

pub fn run() {
    println!("== Modules ==");

    println!("-- (1) 调用内联模块 --");
    greetings::hello();
    greetings::goodbye();
    greetings::reveal();
    println!();

    println!("-- (2) 嵌套模块：math::arithmetic / math::geometry --");
    println!("2 + 3           = {}", math::arithmetic::add(2, 3));
    println!("4 * 5           = {}", math::arithmetic::multiply(4, 5));
    println!("square_area(7)  = {}  (通过 super:: 访问兄弟模块)", math::geometry::square_area(7));
    println!();

    println!("-- (3) use 把长路径引入当前作用域 --");
    // 在函数内 use：作用域限定在函数内。
    use crate::topic_02_modules::math::arithmetic::add as sum;
    println!("use ... as sum: sum(10, 20) = {}", sum(10, 20));
    println!();

    println!("-- (4) use 一次导入多个项 --");
    use crate::topic_02_modules::math::arithmetic::{add, multiply};
    println!("批量 use: add(1,2) = {}, multiply(3,4) = {}", add(1, 2), multiply(3, 4));
    println!();
}
