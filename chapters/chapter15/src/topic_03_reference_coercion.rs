//! 引用强转（Reference Coercion）：`&mut T` 可以隐式变 `&T`，但反过来不行。
//!
//! 规则：
//!
//! | 原类型  | 可隐式变成 |
//! |--------|----------|
//! | `&mut T` | `&T`     |
//! | `&mut T` | `&mut U`（仅当 T: DerefMut<Target = U>） |
//! | `&T`     | `&U`（仅当 T: Deref<Target = U>）      |
//!
//! **`&T` 不能变 `&mut T`**——这会违反借用规则。
//!
//! 实际意义：当函数签名是 `&T`，你可以直接传 `&mut T` 进去，Rust 替你降级。

fn read_only(v: &Vec<i32>) {
    println!("  read_only 看到 len = {}", v.len());
}

pub fn run() {
    println!("== Reference Coercion ==");

    let mut v = vec![1, 2, 3];

    println!("-- (1) &mut T 自动变 &T --");
    // 这里调用方明明给的是 &mut，但函数只想要 &，所以发生 coercion。
    read_only(&mut v);
    read_only(&v); // 当然也能直接传 &
    println!();

    println!("-- (2) Deref + DerefMut --");
    let boxed = Box::new(String::from("hi"));
    let _len = boxed.len(); // &Box<String> → &String → &str 的 deref 链
    println!("  boxed.len() = {}", boxed.len());
    println!();

    println!("-- 反向不成立 --");
    println!("  &T 不能隐式变 &mut T：那会让多个地方同时写，破坏借用规则");
    println!();
}
