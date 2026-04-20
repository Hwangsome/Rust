//! 强制变量走引用：`for x in &v` vs `for x in v` / `iter()` vs `into_iter()`。
//!
//! 很多时候你不想让集合被消费——要显式让迭代走**借用**：
//!
//! | 写法         | 产出  | 是否消费集合 |
//! |------------|-----|-----|
//! | `for x in v` | `T` | ✅ 消费 |
//! | `for x in &v` | `&T` | ❌ 保留 |
//! | `for x in &mut v` | `&mut T` | ❌ 保留 |
//! | `v.iter()` / `.iter_mut()` / `.into_iter()` | 同上 | 同上 |

pub fn run() {
    println!("== Forcing variables to use references ==");

    let words = vec!["rust".to_string(), "safe".to_string(), "fast".to_string()];

    println!("-- (1) for x in &v 保留所有权 --");
    for w in &words {
        println!("  w (借用) = {w}");
    }
    println!("  words 仍可用，len = {}", words.len());

    println!("-- (2) for x in v.iter() 与 &v 等价 --");
    for w in words.iter() {
        print!("{w} ");
    }
    println!();

    println!("-- (3) 消费版 into_iter --");
    for w in words.into_iter() {
        print!("{w}|");
    }
    println!();
    // println!("{words:?}"); // ← words 已被消费
    println!();
}
