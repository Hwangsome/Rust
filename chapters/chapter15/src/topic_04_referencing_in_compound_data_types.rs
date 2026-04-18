//! 复合类型中的引用：struct / enum / tuple / Vec / HashMap 字段是引用时。
//!
//! 规则：
//! - struct / enum 里的引用字段，**整个类型**必须带生命周期参数（如 `<'a>`）
//! - Vec / HashMap 可以装 `&T`，但要注意 `'a` 必须覆盖所有元素
//! - 模式匹配时配合 `ref` / `&` 很常见

use std::collections::HashMap;

struct Report<'a> {
    title: &'a str,
    body: &'a str,
}

pub fn run() {
    println!("== Referencing in Compound Data Types ==");

    println!("-- (1) 带引用字段的 struct --");
    let t = String::from("Rust");
    let b = String::from("This is a great language");
    let r = Report { title: &t, body: &b };
    println!("  title = {}, body.len = {}", r.title, r.body.len());
    println!();

    println!("-- (2) Vec<&T> --");
    let a = String::from("alice");
    let b = String::from("bob");
    let names: Vec<&String> = vec![&a, &b];
    for n in &names { print!("{n} "); }
    println!();
    println!();

    println!("-- (3) HashMap 里的 &K / &V --");
    let key = String::from("hi");
    let value = String::from("world");
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert(&key, &value);
    println!("  map[\"hi\"] = {:?}", map.get("hi"));
    println!();

    println!("-- (4) 元组里的 &mut --");
    let mut x = 10;
    let mut y = 20;
    let tup = (&mut x, &mut y);
    *tup.0 += 1;
    *tup.1 += 1;
    println!("  x = {x}, y = {y}");
    println!();
}
