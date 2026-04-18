//! `Sized` trait：编译器**默认**给每个泛型参数加的隐式 bound。
//!
//! 当你写 `fn f<T>(x: T)` 时，编译器实际看到的是 `fn f<T: Sized>(x: T)`。
//! 这就是为什么泛型参数**默认**不能是 `str` / `[T]` / `dyn Trait` 这种 unsized 类型。
//!
//! 如果你确实想让泛型参数允许 unsized，要**显式**写：
//!
//! ```ignore
//! fn f<T: ?Sized>(x: &T) { ... }
//! //       ^^^^^^ "maybe sized"：放开 Sized 要求
//! ```
//!
//! 注意：一旦 T 可能 unsized，就**只能**通过 `&T` / `Box<T>` 这些指针访问——因为按值传需要知道大小。

use std::fmt::Debug;

/// 默认 Sized：`T` 只能是 `i32`、`String` 这类大小已知的类型。
fn print_sized<T: Debug>(value: T) {
    println!("  [Sized]    {value:?}");
}

/// `?Sized`：放开要求，允许 T 是 str、[i32]、dyn Trait 等。
/// 但我们只能用 &T 接收（不能按值）。
fn print_maybe_sized<T: ?Sized + Debug>(value: &T) {
    println!("  [?Sized]   {value:?}");
}

pub fn run() {
    println!("== Sized & Optionally Sized Trait ==");

    println!("-- (1) 默认 T: Sized --");
    print_sized(42);
    print_sized("hello"); // ⚠️ 这里传的是 &str，是 Sized 的；不是 str 本身
    println!();

    println!("-- (2) T: ?Sized 让泛型接受 unsized 类型 --");
    print_maybe_sized::<str>("world");       // str 是 unsized，通过 &str 传
    print_maybe_sized::<[i32]>(&[1, 2, 3]); // [i32] 是 unsized，通过 &[i32] 传
    println!();

    println!("-- 记忆要点 --");
    println!("  `T: Sized` 是默认，几乎所有普通泛型都满足");
    println!("  `T: ?Sized` 只在你写 `&T`、`Box<T>` 等时才需要");
    println!();
}
