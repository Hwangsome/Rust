//! 解构引用：在模式里用 `&` 和 `ref`。
//!
//! 两种方向：
//!
//! - **`&` 在模式里**：把引用 "解开"——`let &x = &42;` 等价于 `let x = 42;`
//! - **`ref`**：在模式里**创建**引用——`let ref y = 42;` 等价于 `let y = &42;`
//!
//! 在 match / 函数参数 / for 里尤其常见。

pub fn run() {
    println!("== Destructing References ==");

    println!("-- (1) 模式里 & 解开引用 --");
    let r = &42;
    let &x = r; // 等价于 let x = *r;
    println!("  x = {x}");

    println!("-- (2) ref 在模式里制造引用 --");
    let v = String::from("hi");
    let ref y = v; // 等价于 let y = &v;
    println!("  y 借用 v: {y}");
    println!("  v 仍然可用: {v}");

    println!("-- (3) 在 match 里解构引用 --");
    let pair = (1, 2);
    match &pair {
        &(a, b) => println!("  解出 a={a}, b={b}（& 在模式里）"),
    }

    let maybe: Option<String> = Some("hello".to_string());
    match &maybe {
        Some(s) => println!("  借用内部值: {s}"),
        None    => println!("  空"),
    }
    println!();
}
