//! 组合子（combinator）：先描述"想做什么转换"，最后再用消费者触发计算。
//!
//! 两类组合子：
//!
//! - **转换型**（adapter）：`map` / `filter` / `take` / `skip` / `enumerate` / `zip` / `chain` / `rev` / `flat_map` / `flatten` / `step_by` / `peekable`...
//!   它们**返回一个新迭代器**，自身惰性，不做真正计算。
//! - **消费型**（consumer）：`collect` / `sum` / `product` / `count` / `for_each` / `fold` / `reduce` / `find` / `any` / `all` / `max` / `min` / `position`...
//!   它们**吃掉整个迭代器**，触发前面链条真正跑起来，返回最终值。
//!
//! 记忆诀窍：**转换型返回 Iterator；消费型返回一个普通值（或 Option）。**

pub fn run() {
    println!("== Combinators ==");

    println!("-- (1) filter + map + collect --");
    let numbers = [1, 2, 3, 4, 5, 6];
    let even_squares: Vec<i32> = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("even_squares = {even_squares:?}");
    println!();

    println!("-- (2) enumerate + zip + chain --");
    let words = ["rust", "trait", "iter"];
    for (i, w) in words.iter().enumerate() {
        print!("  [{i}]={w} ");
    }
    println!();

    let zipped: Vec<(i32, &&str)> = (1..).zip(words.iter()).collect();
    println!("zip with 1..  => {zipped:?}");

    let concat: Vec<i32> = [1, 2].iter().chain([3, 4].iter()).copied().collect();
    println!("chain          => {concat:?}");
    println!();

    println!("-- (3) sum / product / count / fold --");
    let s: i32 = (1..=5).sum();
    let p: i32 = (1..=5).product();
    let c: usize = [10, 20, 30].iter().count();
    let folded: i32 = (1..=5).fold(0, |acc, x| acc + x * 2);
    println!("sum 1..=5     = {s}");
    println!("product 1..=5 = {p}");
    println!("count         = {c}");
    println!("fold(0, +2x)  = {folded} (0+2+4+6+8+10)");
    println!();

    println!("-- (4) find / any / all / position --");
    let found = [1, 3, 5, 6, 7].iter().find(|&&x| x % 2 == 0);
    let any_negative = [1, 2, 3].iter().any(|&x| x < 0);
    let all_positive = [1, 2, 3].iter().all(|&x| x > 0);
    let pos = ["a", "b", "c"].iter().position(|&s| s == "b");
    println!("find first even   = {found:?}");
    println!("any negative      = {any_negative}");
    println!("all positive      = {all_positive}");
    println!("position of \"b\" = {pos:?}");
    println!();

    println!("-- (5) flat_map / flatten --");
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten nested    = {flat:?}");

    let duplicated: Vec<i32> = [1, 2, 3].iter().flat_map(|&x| [x, x * 10]).collect();
    println!("flat_map          = {duplicated:?}");
    println!();

    println!("-- (6) 惰性：只在消费者触发时真正计算 --");
    let chain = (1..=5).map(|x| {
        // 这一行在 collect/sum 之前不会执行
        x * 2
    });
    let total: i32 = chain.sum();
    println!("total = {total}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//         - Combinators
// -------------------------------------------

fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    // let mut result: Vec<String> = vec![];

    // for word in words {
    //     if word.starts_with("a") || word.starts_with("b") {
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }
    // println!("Result: {:?}", result);

    let result: Vec<String> = words
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>();

    println!("Result: {:?}", result);
}

/* 
--------------------------------------------------------------------------------------------------------
Concept/Topic           | Explanation
------------------------|-------------------------------------------------------------------------------
Combinators             | They are methods which oeprate on Iterators for transforming/filtering data.
                        | They can be chained to express complex logic concisely.

Common Combinator       |filter: It removes items based on a predicate closure.
                        | map: It transforms each item into another form.
                         
collect Combinator      | It consumes an iterator and builds a collection.
                        | Requires type information to determine output container (using turbo fish).

Lazy Evaluation         | Combinators do not perform any computation until a consuming method is called.
                        | collect() or next() triggers execution.
--------------------------------------------------------------------------------------------------------
*/
"###;
