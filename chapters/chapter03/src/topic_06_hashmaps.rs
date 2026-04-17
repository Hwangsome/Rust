// HashMap 是最常见的 key-value 容器之一。
// 它特别适合“按名字找分数”“按配置项找值”这类场景。
use std::collections::HashMap;

pub fn run() {
    println!("== HashMaps ==");

    // 插入两组键值对，演示最基础的写入方式。
    let mut scores = HashMap::new();
    scores.insert("math", 95);
    scores.insert("rust", 100);

    println!("scores = {scores:?}");
    println!("rust score = {:?}", scores.get("rust"));
    println!();
}
