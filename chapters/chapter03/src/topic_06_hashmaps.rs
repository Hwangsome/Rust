//! `HashMap<K, V>`：键值对容器。
//!
//! 这一节覆盖 `HashMap` 在日常代码里最常用的 6 类操作：
//!
//! 1. 构造：`HashMap::new()` / `HashMap::from([...])`
//! 2. 插入：`insert` / 覆盖
//! 3. 查询：`get`（返回 `Option<&V>`）
//! 4. 有就改、没有就创建：`entry(...).or_insert(...)` 惯用法
//! 5. 遍历：`iter()` / `keys()` / `values()`
//! 6. 移除：`remove`
//!
//! `HashMap` 对 key 有要求：类型需要实现 `Eq` + `Hash`（大多数基础类型都已经实现）。

use std::collections::HashMap;

pub fn run() {
    println!("== HashMaps ==");

    println!("-- (1) 构造与插入 --");
    // 方式 A：`new()` + `insert`
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("math"), 95);
    scores.insert(String::from("rust"), 100);
    println!("scores = {scores:?}");

    // 方式 B：`from([...])`
    let colors = HashMap::from([
        ("red", "#FF0000"),
        ("green", "#00FF00"),
        ("blue", "#0000FF"),
    ]);
    println!("colors = {colors:?}");
    println!();

    println!("-- (2) 覆盖 vs 仅在不存在时插入 --");
    let mut book = HashMap::new();
    book.insert("rust", 100);
    book.insert("rust", 95); // insert 会直接覆盖旧值
    println!("after two inserts: {book:?}");

    // entry(...).or_insert(...)：如果 key 不存在才插入，否则保持原值。
    book.entry("rust").or_insert(0); // 不改动
    book.entry("go").or_insert(80);  // 新增
    println!("after entry().or_insert(): {book:?}");
    println!();

    println!("-- (3) 基于当前值更新：entry + *value += 1 --");
    // 经典应用：词频统计。
    let text = "rust is fun and rust is fast";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert 返回 `&mut V`，解引用后可以直接修改。
        let counter = counts.entry(word).or_insert(0);
        *counter += 1;
    }
    println!("word counts = {counts:?}");
    println!();

    println!("-- (4) 查询: get 返回 Option<&V> --");
    let rust_score = scores.get("rust");
    let not_there = scores.get("python");
    println!("scores.get(\"rust\")   = {rust_score:?}");
    println!("scores.get(\"python\") = {not_there:?}");
    println!();

    println!("-- (5) 遍历 --");
    for (key, value) in &scores {
        println!("  {key} => {value}");
    }
    println!("仅 keys: {:?}", scores.keys().collect::<Vec<_>>());
    println!("仅 values: {:?}", scores.values().collect::<Vec<_>>());
    println!();

    println!("-- (6) 移除 --");
    let removed = scores.remove("math");
    println!("remove(\"math\") 返回旧值 = {removed:?}");
    println!("remove 之后 scores = {scores:?}");
    println!();
}
