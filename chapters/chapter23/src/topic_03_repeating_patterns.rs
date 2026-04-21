//! 宏里的重复模式：`$(...)* / $(...)+ / $(...),*`。
//!
//! 语法：
//!
//! - `$( ... )*` 零次或多次
//! - `$( ... )+` 一次或多次
//! - `$( ... ),*` 用 `,` 作分隔重复
//!
//! 扩展侧对应同样的重复写法。

/// 仿造一个简易 `vec!`：接受任意数量表达式。
macro_rules! make_vec {
    ( $( $x:expr ),* $(,)? ) => {{
        // 本地 block 让每次调用有独立作用域
        let mut v = Vec::new();
        $( v.push($x); )*
        v
    }};
}

/// 取 max：接一次或多次。
macro_rules! max_of {
    ( $first:expr $(, $rest:expr )+ $(,)? ) => {{
        let mut best = $first;
        $( if $rest > best { best = $rest; } )+
        best
    }};
}

pub fn run() {
    println!("== Repeating Patterns ==");
    let v: Vec<i32> = make_vec![10, 20, 30, 40];
    println!("  make_vec -> {v:?}");

    let m = max_of!(3, 7, 2, 9, 5);
    println!("  max_of(3,7,2,9,5) = {m}");
    println!();
}
