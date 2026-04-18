//! Iterator trait：所有"循环产出值"的基础。
//!
//! ```ignore
//! trait Iterator {
//!     type Item;
//!     fn next(&mut self) -> Option<Self::Item>;
//!     // ... 60 多个带默认实现的方法都建立在 `next()` 之上 ...
//! }
//! ```
//!
//! 只要一个类型实现了 `next()`：
//! - 自动能用 `for x in iter { ... }` 遍历
//! - 自动拥有 `.map()` / `.filter()` / `.sum()` / `.collect()` 等几十个组合子
//! - 自动能被 `.chain()` / `.zip()` / `.rev()` 等适配器包装
//!
//! 这是 Rust 迭代器"零抽象成本"的核心：**迭代器是惰性的**，适配器链条直到被消耗时才做功。
//!
//! 本节演示：
//! 1. 自己实现 `Iterator`（控制 `next()` 的返回时机）
//! 2. 既能被 `for` 消费，也能被组合子消费
//! 3. `Counter` 类型：用自定义迭代器做平方和演示

#[derive(Debug)]
struct EmployeeRecords {
    records: Vec<String>,
    index: usize,
}

impl EmployeeRecords {
    fn new(records: Vec<String>) -> Self {
        Self { records, index: 0 }
    }
}

impl Iterator for EmployeeRecords {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.records.len() {
            None
        } else {
            let current = self.records[self.index].clone();
            self.index += 1;
            Some(current)
        }
    }
}

/// 另一个自定义迭代器：产生 1..=5。
struct Counter {
    current: u32,
    limit: u32,
}

impl Counter {
    fn new(limit: u32) -> Self {
        Self { current: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.limit {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

pub fn run() {
    println!("== Iterators ==");

    println!("-- (1) 自定义 Iterator: EmployeeRecords --");
    let records = EmployeeRecords::new(vec![
        "Ada".to_string(),
        "Grace".to_string(),
        "Linus".to_string(),
    ]);
    for record in records {
        println!("  record => {record}");
    }
    println!();

    println!("-- (2) 自定义 Iterator 自动获得 60+ 方法 --");
    let total: u32 = Counter::new(5).sum(); // 1+2+3+4+5 = 15
    println!("Counter::new(5).sum()           = {total}");

    let evens: Vec<u32> = Counter::new(10).filter(|n| n % 2 == 0).collect();
    println!("Counter::new(10).filter(even)   = {evens:?}");

    let squared_sum: u32 = Counter::new(5).map(|n| n * n).sum();
    println!("Counter::new(5).map(^2).sum()   = {squared_sum} (1+4+9+16+25)");
    println!();

    println!("-- (3) 迭代器是惰性的 --");
    let lazy = (1..=1_000_000)
        .map(|n| {
            // 这行闭包在没有被 collect/for 消费前不会执行
            n * n
        })
        .filter(|n| n % 3 == 0);
    println!("构造好链条，但还没跑——没有打印任何消息。");
    let count = lazy.take(5).count();
    println!("只取前 5 个满足条件的，count = {count}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Iterator
// -------------------------------------------

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}
fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };

    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    for employee in emp_db {
        println!("{employee}");
    }
}

/* 
-----------------------------------------------------------------------------------------------------
Concept/Topic                  | Explanation
-------------------------------|---------------------------------------------------------------------
Iterator Trait                 | Provides a uniform interface for sequential access to elements.
                               | Requires an associated type Item and implementation of next method.

next Method and type Item      | Signature: fn next(&mut self) -> Option<Self::Item>.
                               | next() returns Some(item) for the next element or None.
                               | Type Item specifies the element type yielded.
                             
for Loops and Iterators        | for loops automatically call next() internally.                          
                               | Loop stops when `None` is encountered and unwraps `Some`.

Custom Iterator Implementation | Implementers control what is yielded and how.
                               | Items can be primitive types or custom structs.
                               | Must adhere to returning Option<Item> from next().
-----------------------------------------------------------------------------------------------------
*/
"###;
