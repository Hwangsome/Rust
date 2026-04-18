//! 代码组织：package / crate / module 三层关系。
//!
//! 这一节要彻底分清这 3 个名词——它们是 Rust 工程化的基石。
//!
//! ```text
//!                    ┌─────────────────────────────────────┐
//!  **package** ───>  │  由一个 Cargo.toml 描述的项目单元       │
//!                    │  可以包含: 0..=1 个 library crate +    │
//!                    │            0..N 个 binary crate        │
//!                    └─────────────────────────────────────┘
//!                                    │
//!                                    ▼
//!                    ┌─────────────────────────────────────┐
//!  **crate**   ───>  │  一次编译的最小单元                     │
//!                    │  binary crate: 产出可执行文件（main.rs）│
//!                    │  library crate: 产出可复用库（lib.rs） │
//!                    └─────────────────────────────────────┘
//!                                    │
//!                                    ▼
//!                    ┌─────────────────────────────────────┐
//!  **module**  ───>  │  crate 内部的命名空间和可见性边界      │
//!                    │  通过 `mod` 声明, 通过 `use` 引入      │
//!                    └─────────────────────────────────────┘
//! ```
//!
//! 在本仓库的 workspace 里：
//! - 顶层 `Cargo.toml` 声明了 workspace（有多个 package 的容器）
//! - 每个 `chapters/chapterNN/Cargo.toml` 描述**一个 package**
//! - 每个 chapter 是一个 **binary crate**（有 `main.rs`）
//! - crate 内部用 `mod topic_01_xxx;` 等**模块**来组织主题

pub fn run() {
    println!("== Code Organization ==");

    println!("-- (1) package: 由一个 Cargo.toml 描述 --");
    println!("  本 chapter 是一个 package：chapters/chapter04/Cargo.toml");
    println!();

    println!("-- (2) crate: 编译单元 --");
    println!("  本 chapter 是一个 binary crate：入口 src/main.rs");
    println!("  如果还想让它同时作为 lib 使用，可以再加 src/lib.rs（此时 package 有 2 个 crate）");
    println!();

    println!("-- (3) module: crate 内部的命名空间 --");
    println!("  main.rs 里用 `mod topic_01_code_organization;` 把本文件注册为模块");
    println!("  调用路径是 `crate::topic_01_code_organization::run()`，但在 main.rs 里常常省略前缀");
    println!();

    println!("-- 判断小练习 --");
    println!("  本 workspace 根目录 Cargo.toml：这是 workspace 根，不对应单独的 crate");
    println!("  chapters/chapter01/Cargo.toml：一个 package");
    println!("  chapters/chapter01/src/main.rs：这个 package 的 binary crate 入口");
    println!("  chapters/chapter01/src/topic_02_variables.rs：该 crate 内的一个 module");
    println!();
}
