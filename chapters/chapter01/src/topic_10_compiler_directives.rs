//! 编译器指令（Attributes）：`#[...]` 与 `#![...]` 的入门用法。
//!
//! Rust 用 **attribute** 向编译器和工具链传递元信息，比如：
//! - 允许/拒绝某类 lint
//! - 让 struct 自动实现一组常用 trait（`derive`）
//! - 限定某段代码只在测试或某个平台编译
//! - 标记"这个返回值别扔"
//!
//! 两种语法要分清：
//! - `#[...]`  作用于**紧跟其后的那个 item**（函数、struct、字段……）
//! - `#![...]` 作用于**当前所在的 item / 文件 / 模块**（以 `!` 提示"作用于自己所属的范围"）
//!
//! 本节演示几个**最常见**的 attribute。真正实用的还有很多，后面章节会陆续遇到。

// `#![...]` 写在文件/模块开头，作用于整个文件或模块。
// 这里放开对"未使用变量"的警告，方便教学示例留下一些"暂时没有用到"的绑定。
#![allow(unused_variables)]

/// `#[derive(...)]`：让编译器自动生成一组常用 trait 的实现。
///
/// 这是最常见、最"无感"的 attribute 用法之一——
/// 我们写一个简单的 struct，`derive` 就让它同时拥有 `Debug` 打印、`Clone` 复制、`PartialEq` 相等性比较。
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// `#[allow(dead_code)]`：告诉编译器"这个项目我知道暂时没用，别再警告了"。
///
/// 与之配套的还有 `#[deny(...)]`（升级为错误）、`#[warn(...)]`（显式保持警告）。
#[allow(dead_code)]
fn square(x: i32) -> i32 {
    let unused_local = 42;
    x * x
}

/// `#[must_use]`：如果调用方丢弃了这个函数的返回值，会得到警告。
///
/// 这是一个非常"防止 bug"的 attribute——比如 `Result` 类型上就有它，
/// 所以忘了处理 `Result` 的返回值就会收到警告。
#[must_use = "计算结果应当被使用，否则为什么要算一遍？"]
fn double(x: i32) -> i32 {
    x * 2
}

/// `#[inline]`：给编译器一个建议——把这个函数的调用展开成内联代码。
///
/// 这只是**建议**，编译器最终决定要不要采纳。写不写几乎不影响正确性，但偶尔影响性能。
#[inline]
fn add_one(x: i32) -> i32 {
    x + 1
}

/// `#[cfg(...)]`：条件编译。只有当条件满足时，这个 item 才会被编译进来。
///
/// 最常见的用途是 `#[cfg(test)]` —— 只在 `cargo test` 下编译测试代码。
/// 这里写一个平台相关的例子：只在 macOS 上编译。
#[cfg(target_os = "macos")]
#[allow(dead_code)]
fn only_on_macos() -> &'static str {
    "你正在 macOS 上运行这段代码"
}

pub fn run() {
    println!("-- (1) #[derive(...)] 自动实现常用 trait --");
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // 因为 derive 了 Clone
    println!("p1 = {p1:?}, p2 = {p2:?}, p1 == p2? {}", p1 == p2);
    println!();

    println!("-- (2) #[allow(dead_code)] 静默未使用告警 --");
    println!("square(5) = {}", square(5));
    println!();

    println!("-- (3) #[must_use] 强制检查返回值 --");
    // 下面这样调用才是正确的——接住返回值并使用它。
    let doubled = double(21);
    println!("double(21) = {doubled}");
    // 如果我们写成 `double(21);` 直接丢弃，编译器会给 unused_must_use 警告。
    println!();

    println!("-- (4) #[inline] 只是建议 --");
    println!("add_one(9) = {}", add_one(9));
    println!();

    println!("-- (5) #[cfg(...)] 条件编译 --");
    println!("本节展示了 `#[cfg(target_os = \"macos\")]` 的写法（在非 macOS 上不会编译该函数）");
    println!();
}
