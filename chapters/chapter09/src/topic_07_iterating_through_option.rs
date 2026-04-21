//! 对 `Option<T>` 做迭代：把"可选值"自然地接到迭代链上。
//!
//! 关键心智：`Option<T>` 实现了 `IntoIterator`，把自己当作"0 或 1 个元素的集合"：
//!
//! - `Some(x)` ≈ `[x]`（一个元素的迭代器）
//! - `None`   ≈ `[]`（空迭代器）
//!
//! 因此 `Option` 能直接参与：
//! - `.extend(Some(x))`：如果是 Some 就 push，None 就什么都不做
//! - `.chain(some.iter())`：把可选值拼进现有迭代器
//! - `.flatten()` / `.flat_map()`：把 `Iterator<Item=Option<T>>` 压扁成 `Iterator<Item=T>`
//!
//! 这套写法比 `match` / `if let` 更紧凑，特别适合数据处理流水线。

pub fn run() {
    println!("== Iterating Through Option ==");

    println!("-- (1) extend(Option)：Some 就 push，None 什么都不做 --");
    let mut products = vec!["keyboard", "mouse"];
    products.extend(Some("monitor")); // 追加
    products.extend(None::<&str>);     // 无操作
    products.extend(Some("headphones"));
    println!("products = {products:?}");
    println!();

    println!("-- (2) chain 把 Option 接到现有迭代器后 --");
    let base = vec!["cellphone", "battery"];
    let extra: Option<&str> = Some("charger");
    let combined: Vec<&&str> = base.iter().chain(extra.iter()).collect();
    println!("combined = {combined:?}");
    println!();

    println!("-- (3) flatten：去掉 Iterator<Option<T>> 里的 None，得到 Iterator<T> --");
    let tags = [Some("rust"), None, Some("iterator"), None, Some("async")];
    let flattened: Vec<&str> = tags.into_iter().flatten().collect();
    println!("flattened = {flattened:?}");
    println!();

    println!("-- (4) filter_map：一步同时做 map + 去 None --");
    let raw = ["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = raw.iter().filter_map(|s| s.parse().ok()).collect();
    println!("filter_map 解析数字 = {numbers:?}");
    println!();

    println!("-- (5) Option::map 链式组合子 --");
    let maybe: Option<i32> = Some(10);
    let processed: Option<i32> = maybe
        .map(|x| x + 1)
        .filter(|&x| x > 5)
        .map(|x| x * 2);
    println!("链式 map/filter/map on Option = {processed:?}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//          - Iterating Through Options
// -------------------------------------------

fn main() {
    // ------ Use case 1 -----

    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    // Solution 1:
    // match some_product {
    //     Some(product) => products.push(product),
    //     _ => {}
    // };

    // Solution 2:
    // if let Some(product) = some_product {
    //     products.push(product);
    // }

    // Solution 3:
    products.extend(some_product);
    println!("{:?}", products);

    // ------- Use case 2 -----
    let mut products = vec!["cellphone", "battery", "charger"];
    let products_iter = products.iter().chain(some_product.iter());

    for prod in products_iter {
        println!("{:?} ", prod);
    }

    // ------ Use Case 3 -----
    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

    // Solution 1;
    // let mut prod_without_none = Vec::new();
    // for p in products {
    //     if p.is_some() {
    //         prod_without_none.push(p.unwrap());
    //     }
    // }

    // Solution 2:
    // let prod_without_none = products
    //     .into_iter()
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap())
    //     .collect::<Vec<&str>>();

    // Solution 3:
    let prod_wihtout_none: Vec<&str> = products.into_iter().flatten().collect();
    println!("{:?}", prod_wihtout_none);
}


/* 
----------------------------------------------------------------------------------------
Concept/Topic                  | Explanation
----------------------------------------------------------------------------------------
Option as an Iterator          | Option behaves like an iterator of zero or one item.  
                               | Can be chained with other iterators.  
                               | Useful in iterator-based workflows.

Chaining Iterators with Option | .iter() converts Option into an iterator.  
                               | chain method combines it with another iterator.  
                               | Produces a unified sequence for iteration.

Filtering Optional Values      | Manual loop can extract only Some values.  
                               | Iterator methods like filter and map improve clarity.  

Using flatten on Option        | flatten removes None automatically.  
                               | Extracts inner values from Some. 
                               | Produces a clean iterator before collect.
----------------------------------------------------------------------------------------
 */
"###;
