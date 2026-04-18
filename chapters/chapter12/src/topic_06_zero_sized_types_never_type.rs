//! Never 类型 `!`：**永远不返回**的函数的类型。
//!
//! 在 stable Rust 里，`!` 还不能直接作为**类型**使用（`#![feature(never_type)]` 才能写 `-> !` 以外的位置），
//! 但**作为返回类型**是完全稳定的：
//!
//! ```ignore
//! fn unrecoverable() -> ! {
//!     panic!("never returns normally");
//! }
//! ```
//!
//! 常见"返回 `!`"的场景：
//! - `panic!` / `unreachable!` / `todo!` 宏内部
//! - `loop { }`（如果 loop 里永远不 break）
//! - `std::process::exit(code)`
//!
//! `!` 的妙处：它可以**被强转成任何类型**。这样写法就合法了：
//!
//! ```ignore
//! let x: i32 = if cond { 42 } else { panic!("nope") };
//! //                                  ^^^^^^^^^^^^^^ 返回 !，强转成 i32
//! ```

/// 返回 `!`：永远不会走到"`}`"结束位置。
fn unrecoverable_state() -> ! {
    loop {
        // 假设这里循环里最终会 panic，但我们为了不中断教学运行就让它立即 break 到外层。
        // 在真实代码里，返回 `!` 的函数不会走到这里。
        unreachable!("this example never actually runs");
    }
}

pub fn run() {
    println!("== Never Type `!` ==");

    println!("-- (1) `!` 作为返回类型 --");
    println!("  签名: fn unrecoverable_state() -> !");
    println!("  编译器知道：这个函数之后的代码不可达");
    println!();

    println!("-- (2) `!` 可以强转成任何类型 --");
    let n = 5;
    // `panic!(...)` 返回 `!`，可以作为 if 分支之一
    let value: i32 = if n > 0 {
        n * 2
    } else {
        // 这支永远不会跑到——n > 0
        #[allow(unreachable_code)]
        { panic!("n 非正"); 0 }
    };
    println!("  value = {value}");
    println!();

    println!("-- (3) 运行时：本节不触发 unrecoverable_state --");
    println!("  unrecoverable_state 的写法见代码；为了 chapter 可运行，这里没有调用它");
    // 下面这行如果取消注释，会让整个 chapter run 卡死或 panic
    // unrecoverable_state();
    let _ = unrecoverable_state; // 强制使用一下函数项，避免 dead_code 警告
    println!();
}
