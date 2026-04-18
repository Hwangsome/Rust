//! 泛型中的 coercion：**不**会自动发生。
//!
//! 这是很多初学者踩到的坑：coercion 只在**具体类型**位置触发，**不**会在泛型参数位置"自动"发生。
//!
//! ```ignore
//! fn f<T>(x: T) { ... }
//! f(5);           // T 被推断为 i32
//! f::<i64>(5);    // ❌ E0308：5 是 i32，不会自动变 i64
//! f::<i64>(5_i64); // ✅ 或 `5 as i64`
//! ```
//!
//! 泛型想要"兼容多种类型"要靠 **trait bound**（比如 `T: Into<i64>`），而不是依赖 coercion。

fn print_as_slice(s: &[i32]) {
    println!("  print_as_slice: {s:?}");
}

fn print_generic<T: AsRef<[i32]>>(s: T) {
    println!("  print_generic via AsRef: {:?}", s.as_ref());
}

pub fn run() {
    println!("== Coercion in Generics ==");

    let arr = [1, 2, 3];

    println!("-- (1) 具体参数位置：coercion 生效 --");
    print_as_slice(&arr); // &[i32;3] → &[i32]，正常
    println!();

    println!("-- (2) 通过 AsRef<[i32]> bound 让泛型接受多种类型 --");
    print_generic([1, 2, 3]);         // 数组实现了 AsRef<[i32]>
    print_generic(vec![4, 5, 6]);     // Vec 也实现了
    print_generic(&[7, 8, 9] as &[i32]);
    println!();

    println!("-- 记忆 --");
    println!("  coercion 只看‘这个位置是不是在触发列表里’，不看泛型");
    println!("  想在泛型上做‘类型兼容’，用 AsRef / Borrow / Into 等 bound");
    println!();
}
