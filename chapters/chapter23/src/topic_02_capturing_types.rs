//! 宏里捕获不同种类的片段：`expr` / `ident` / `ty` / `literal` / `tt`。
//!
//! | 捕获类型 | 匹配什么 |
//! |--------|--------|
//! | `expr` | 表达式（如 `1 + 2`、`vec![1,2,3]`）|
//! | `ident` | 标识符（变量名、类型名的单个 token）|
//! | `ty`   | 类型（如 `i32`、`Vec<u8>`）|
//! | `literal` | 字面量（数字、字符串、字符）|
//! | `tt`   | 任意 token tree——最通用但最难控制 |
//! | `pat`  | 模式 |
//! | `path` | 路径 |
//! | `stmt` | 语句 |

/// 用 ident 生成一个变量。
macro_rules! create_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
        println!("  created {}", stringify!($name));
    };
}

/// 用 ty 生成一个类型别名。
macro_rules! make_type_alias {
    ($name:ident, $t:ty) => {
        type $name = $t;
    };
}

make_type_alias!(MyInt, i32);

pub fn run() {
    println!("== Capturing Types ==");
    create_var!(x, 42);
    create_var!(greeting, String::from("hi"));
    let n: MyInt = 100;
    println!("  MyInt = {n}");
    println!();
}
