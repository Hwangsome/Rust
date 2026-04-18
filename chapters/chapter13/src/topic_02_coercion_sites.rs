//! Coercion site（强转触发位置）：编译器只在**特定位置**做 coercion。
//!
//! 常见触发位置：
//!
//! 1. **函数调用参数**：`foo(&my_string)` 这里 `&String` → `&str`
//! 2. **let 的显式类型标注**：`let s: &str = &my_string;`
//! 3. **静态变量 / const 初始化**
//! 4. **return 表达式**（返回类型与表达式类型不同时）
//! 5. **方法调用**（尤其是 deref 链式查找）
//! 6. **array / struct / tuple 的字面量**里某些子表达式
//!
//! **不**触发 coercion 的位置：
//! - 裸变量赋值（非 let 带类型）
//! - 在条件分支里（需要手动让两边类型一致）

fn needs_str(s: &str) -> usize { s.len() }

pub fn run() {
    println!("== Coercion Sites ==");

    let owned = String::from("hello");

    println!("-- (1) 函数参数位置 --");
    let n = needs_str(&owned); // &String → &str
    println!("  needs_str(&owned) = {n}");

    println!("-- (2) let 带类型标注 --");
    let s: &str = &owned; // 同样是 &String → &str
    println!("  s = {s}");

    println!("-- (3) return 表达式 --");
    fn first_line(text: &String) -> &str {
        text.lines().next().unwrap_or("")
    }
    println!("  first_line = {}", first_line(&owned));

    println!("-- (4) Box<T> 的 deref 方法链 --");
    let boxed = Box::new(vec![1, 2, 3]);
    // (*boxed).len() == boxed.len() == (*boxed).as_slice().len()
    // 这里发生了 &Box<Vec<i32>> → &Vec<i32> → &[i32] 的多层 deref
    println!("  boxed.len() = {}", boxed.len());
    println!();
}
