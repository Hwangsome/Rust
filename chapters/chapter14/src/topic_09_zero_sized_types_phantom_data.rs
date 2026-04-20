//! 零大小类型 (ZST) 之三：**`PhantomData<T>`**。
//!
//! `PhantomData<T>` 是编译期的"**假装自己有一个 T 字段**"标记——它**不占内存**，
//! 但会让 struct 在**类型系统层面**表现得像持有 T：
//!
//! - 参与 `Send` / `Sync` 自动推断
//! - 参与 **drop check**（drop 顺序推断）
//! - 参与泛型协变/逆变标注
//!
//! 本节展示两个最常见的用途：
//!
//! 1. **绑定生命周期**：在一个实际不持有 `&'a T` 的 struct 上声明"我和 `'a` 有关"
//! 2. **对编译器假装持有 `T`**：比如 `Rc<()>` 让 struct 变成 `!Send + !Sync`

use std::marker::PhantomData;
use std::mem::size_of;
use std::rc::Rc;

/// 一个"装有 Rc<()> 的 ghost 字段"的 struct——
/// 这让 Rust 把它当成"不 Send、不 Sync"的类型。
/// 实际运行时没有 Rc，也不占空间。
struct NotSendOrSync {
    _ghost: PhantomData<Rc<()>>,
}

/// 这个 struct 在运行时**不持有** `&'a str`，
/// 但类型系统认为它和 `'a` 有关——这可能在 FFI / ID 生成器等场景有用。
#[allow(dead_code)]
struct Tagged<'a> {
    id: u64,
    _marker: PhantomData<&'a str>,
}

pub fn run() {
    println!("== ZST: PhantomData ==");

    println!("-- (1) PhantomData 不占空间 --");
    println!("  size_of::<NotSendOrSync>() = {}", size_of::<NotSendOrSync>());
    println!();

    println!("-- (2) 靠 PhantomData<Rc<()>> 把类型标成 !Send + !Sync --");
    #[allow(dead_code)]
    fn assert_send<T: Send>() {}
    // assert_send::<NotSendOrSync>(); // ← 会编译失败，NotSendOrSync 不是 Send
    println!("  上面的 assert_send::<NotSendOrSync>() 在代码里是注释的——");
    println!("  取消注释会得到：`NotSendOrSync` cannot be sent between threads safely");
    let _ = NotSendOrSync { _ghost: PhantomData };
    println!();

    println!("-- (3) Tagged<'a> 绑定生命周期 --");
    let _: Tagged<'_> = Tagged { id: 1, _marker: PhantomData };
    println!("  Tagged 不实际持有 &'a，但在类型系统里表现得像持有");
    println!();

    println!("记忆: PhantomData<T> = ‘我假装持有 T，但零字节开销’");
    println!();
}
