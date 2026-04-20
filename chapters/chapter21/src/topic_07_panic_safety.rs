//! Panic 安全（panic safety）：panic 展开栈时仍要保证数据结构不腐化。
//!
//! Rust 默认的 panic 处理模式是"**栈展开**（unwind）"——每一层栈帧的 Drop 会被调用。
//! 只要你的 Drop 实现不 panic（再 panic 会导致 abort），基本都能安全展开。
//!
//! **panic safety 级别**（std 的术语）：
//!
//! - `UnwindSafe` / `RefUnwindSafe`：标记类型在 panic 后仍处于"合理状态"
//! - `catch_unwind` 可以捕获子作用域的 panic——但不是为了替代 Result，是给 FFI / 测试框架等用的

use std::panic;

pub fn run() {
    println!("== Panic Safety ==");

    println!("-- (1) Drop 在 panic 时仍然会跑 --");
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) { println!("  [guard] drop during unwinding"); }
    }

    let result = panic::catch_unwind(|| {
        let _g = Guard;
        // 这里故意 panic
        panic!("boom!");
    });
    match result {
        Ok(_) => println!("  no panic"),
        Err(_) => println!("  caught panic, continuing main"),
    }
    println!();

    println!("-- (2) 记忆 --");
    println!("  panic 展开时，所有栈上局部都会被 drop；");
    println!("  但不要在 Drop 里 panic——会升级为 abort；");
    println!("  catch_unwind 仅用于边界场景，不能替代 Result 错误处理");
    println!();
}
