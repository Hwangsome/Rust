//! 遍历集合：`iter()` / `iter_mut()` / `into_iter()` 三种方式，一次看清边界。
//!
//! | 方法           | 产出类型  | 对原集合的影响                |
//! |---------------|---------|------------------------|
//! | `iter()`      | `&T`    | 只读借用，原集合保留            |
//! | `iter_mut()`  | `&mut T`| 独占借用，可就地改；原集合保留       |
//! | `into_iter()` | `T`     | **消费**集合（取所有权），原集合失效 |
//!
//! 对应的 `for` 循环写法：
//!
//! ```ignore
//! for x in &v        { /* &T */ }
//! for x in &mut v    { /* &mut T */ }
//! for x in v         { /* T，消费 v */ }
//! ```
//!
//! `HashMap` / `BTreeMap` 的遍历还会产出 `(K, V)` 的元组——同样有三种借用形态。

use std::collections::HashMap;

pub fn run() {
    println!("== Iterating Through Collections ==");

    println!("-- (1) Vec::iter() 产出 &T，原 Vec 保留 --");
    let numbers = vec![1, 2, 3];
    let borrowed_sum: i32 = numbers.iter().sum();
    println!("sum = {borrowed_sum}");
    println!("numbers 仍可用 = {numbers:?}");
    println!();

    println!("-- (2) Vec::iter_mut() 产出 &mut T，原 Vec 保留但元素被改 --");
    let mut scores = vec![10, 20, 30];
    for score in scores.iter_mut() {
        *score += 5;
    }
    println!("scores 变成 = {scores:?}");
    println!();

    println!("-- (3) Vec::into_iter() 产出 T，原 Vec 被消费 --");
    let consumed: Vec<i32> = scores.into_iter().map(|s| s * 2).collect();
    println!("consumed = {consumed:?}");
    // println!("{scores:?}"); // ← scores 已被消费，不能再用
    println!();

    println!("-- (4) HashMap 遍历 --");
    let mut status = HashMap::new();
    status.insert("build", "ok");
    status.insert("test", "ok");
    status.insert("lint", "warn");

    // &HashMap → 产出 (&K, &V)
    for (step, result) in &status {
        println!("  {step} => {result}");
    }

    // HashMap::iter_mut() → 产出 (&K, &mut V)（只能改 value，不能改 key）
    let mut counts: HashMap<&str, u32> = [("a", 1), ("b", 2)].iter().copied().collect();
    for (_k, v) in counts.iter_mut() {
        *v += 100;
    }
    println!("counts after iter_mut = {counts:?}");
    println!();

    println!("-- (5) .iter() vs `&v` 等价 --");
    let v = vec![1, 2, 3];
    let a: i32 = v.iter().sum();
    let b: i32 = (&v).into_iter().sum(); // for-in 语法糖的本质
    println!("两种写法都得 {a} / {b}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//         - Iterating Through Collections
// -------------------------------------------

use std::collections::HashMap;

fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.into_iter();
    // let value_1 = vec_1_iter.next();

    for values in vec_1 {
        println!("{values}");
    }

    // println!("{:?}", vec_1);

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Hannash".to_string(), 40);
    person.insert("Joseph".to_string(), 44);
    person.insert("Sara".to_string(), 55);

    for (name, age) in person {
        println!("The person {} has an age of {}", name, age);
    }
}

/*
----------------------------------------------------------------------------------------------------------
Concept/Topic             | Explanation
--------------------------|-------------------------------------------------------------------------------
Iterators from Vec        | Collections like Vec can produce iterators using three primary methods.
                          | iter(): convert a vec<T> into iterator over &T (immutable references).
                          | iter_mut(): convert a vec<T> into iterator over &mut T (mutable references).
                          | into_iter(): convert a vec<T> into iterator over T (owned values).

for Loop Behavior         | for loops automatically call the appropriate iterator method.
                          | for x in &vec → uses iter().
                          | for x in &mut vec → uses iter_mut().
                          | for x in vec → uses into_iter() and moves ownership.

Ownership After Iteration | Using into_iter() moves the collection which disallows further access.

HashMap Iteration         | iter() yields (&K, &V).
                          | iter_mut() yields (&K, &mut V), i.e., only value as mutable reference.
----------------------------------------------------------------------------------------------------------
*/
"###;
