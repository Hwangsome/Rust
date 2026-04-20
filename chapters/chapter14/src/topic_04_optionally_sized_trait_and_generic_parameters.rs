//! `?Sized` 与泛型参数的配合：什么时候你真的需要 `?Sized`？
//!
//! 几个典型触发场景：
//!
//! 1. **方法的第一个参数是 `&self`**，而你想让类型 `T` 可以是 unsized
//!    —— `trait AsDebug { fn as_debug(&self) -> &dyn Debug; }`
//!    这里 `Self` 默认是 `?Sized`，不用额外加
//! 2. **泛型 struct 要能装 unsized 类型**
//!    —— `struct Wrapper<T: ?Sized> { inner: T }` （必须通过 `Box<Wrapper<str>>` 之类使用）
//! 3. **泛型函数需要接受 &str / &[T] 等**
//!    —— `fn print<T: ?Sized + Debug>(x: &T)`
//!
//! 常见陷阱：`?Sized` 和 `Sized` 不能同时出现；不能对**字段**直接放置 `?Sized` 的 T（只能放 `Box<T>` 之类）。

use std::fmt::Debug;

/// 泛型结构体：只要 T 实现了 Debug 就能打印。默认 T 是 Sized。
#[derive(Debug)]
#[allow(dead_code)]
struct Wrapper<T: Debug> {
    inner: T,
}

/// 能容纳 unsized T 的包装器：T: ?Sized。
/// 注意字段必须是"最后一个字段"才能是 unsized 的——类似 `Box<[T]>` 的设计。
#[allow(dead_code)]
struct UnsizedWrapper<T: ?Sized> {
    header: u32,
    tail: T, // 必须是最后一个字段
}

pub fn run() {
    println!("== Optionally Sized Trait & Generic Parameters ==");

    println!("-- (1) 默认 Sized 的 Wrapper --");
    let w = Wrapper { inner: vec![1, 2, 3] };
    println!("  {w:?}");
    println!();

    println!("-- (2) UnsizedWrapper<T: ?Sized>：通过 Box 承载 unsized --");
    // 这里只能通过指针使用，具体构造较复杂（需要 unsize coercion），本节只强调类型声明。
    println!("  UnsizedWrapper 的声明允许 T 是 unsized（例如 str、[u8]）");
    println!("  实际用法常见于 Box<UnsizedWrapper<[u8]>> 等自引用 trick");
    println!();

    println!("-- (3) 总结 --");
    println!("  泛型参数默认 Sized——绝大多数情况下不用管");
    println!("  只有当你想让 T 能代表 str / [T] / dyn Trait 时才加 ?Sized");
    println!();
}
