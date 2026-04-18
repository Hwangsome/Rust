//! 可见性：`pub` / `pub(crate)` / `pub(super)` / 默认私有。
//!
//! Rust 的可见性规则非常精细，远不止"public / private"两种：
//!
//! | 修饰              | 在哪能看到                         |
//! |-----------------|--------------------------------|
//! | 默认（不写）        | 只有**定义该 item 的模块**内部可见          |
//! | `pub`           | 所有能看到该模块的地方                     |
//! | `pub(crate)`    | 只在当前 crate 内部可见（对外屏蔽）           |
//! | `pub(super)`    | 只在父模块可见                         |
//! | `pub(in path)`  | 限定在指定模块路径下可见（高级，本节不展开）           |
//!
//! 本节演示最常见的 3 种：默认私有、`pub`、`pub(crate)`。

mod school {
    /// 公开函数：模块外也能调用。
    pub fn open_class() {
        prepare_room();
        announce();
        println!("公开入口完成上课前准备。");
    }

    /// 私有函数：只在 `school` 模块内可见。
    fn prepare_room() {
        println!("  私有 prepare_room()：排桌椅、擦黑板");
    }

    /// crate 级别可见：在 `chapter04` 这个 crate 内的任何地方都可见，
    /// 但作为 library 发布时对外部不可见。
    pub(crate) fn announce() {
        println!("  pub(crate) announce()：crate 内部任何模块都可访问");
    }

    pub mod administration {
        /// `pub(super)` 只对父模块可见，出了 `school` 就看不见了。
        pub(super) fn internal_policy() {
            println!("  pub(super) internal_policy()：只有 school 内部能调用");
        }

        /// 普通 `pub` 入口：整个 crate（以及更外层）都能用。
        pub fn public_notice() {
            internal_policy(); // 在 administration 内调用 internal_policy 合法
            println!("  pub public_notice()：对外的公告");
        }
    }
}

pub fn run() {
    println!("== Visibility ==");

    println!("-- (1) pub 函数入口 --");
    school::open_class();
    println!();

    println!("-- (2) pub(crate) --");
    // crate 内部任何地方都能直接调用 `announce()`。
    school::announce();
    println!();

    println!("-- (3) pub(super) 只对父模块可见 --");
    // 以下这行如果取消注释，会得到 E0603：pub(super) 不对"祖先的兄弟模块"公开。
    // school::administration::internal_policy();
    school::administration::public_notice();
    println!();

    println!("默认私有是 Rust 的保守出发点：你必须**显式**放开一个 item，才能让外部使用它。");
    println!();
}
