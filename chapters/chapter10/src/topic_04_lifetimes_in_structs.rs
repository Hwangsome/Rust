//! 带引用字段的 struct：必须给字段标**生命周期**。
//!
//! 只要 struct 的字段里**有引用**，这个 struct 本身就必须带生命周期参数。
//! Rust 通过这个要求来保证：struct 实例不会比它借用的数据活得长。
//!
//! ```ignore
//! struct ArrayProcessor<'a> {
//!     data: &'a [i32],  // ← 带生命周期的引用字段
//! }
//! ```
//!
//! 含义："`ArrayProcessor` 的实例最多活到 `'a` 结束；`'a` 是调用方提供的那段切片的寿命。"

struct ArrayProcessor<'a> {
    data: &'a [i32],
}

impl<'a> ArrayProcessor<'a> {
    fn new(data: &'a [i32]) -> Self {
        Self { data }
    }

    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }

    fn update_data(&mut self, data: &'a [i32]) {
        self.data = data;
    }
}

/// 同时持有**两个**引用字段：可以共享同一个生命周期，也可以分开。
struct Comparison<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Comparison<'a> {
    fn longer(&self) -> &'a str {
        if self.left.len() > self.right.len() { self.left } else { self.right }
    }
}

pub fn run() {
    println!("== Lifetimes in Structs ==");

    println!("-- (1) 带引用字段的 struct --");
    let first = [1, 2, 3];
    let second = [4, 5, 6];
    let mut processor = ArrayProcessor::new(&first);
    println!("first slice sum   = {}", processor.sum());
    processor.update_data(&second);
    println!("updated slice sum = {}", processor.sum());
    println!();

    println!("-- (2) 多个引用字段共享同一个生命周期 --");
    let a = String::from("hi");
    let b = String::from("world!");
    let cmp = Comparison { left: &a, right: &b };
    println!("longer of two = {}", cmp.longer());
    println!();

    println!("-- (3) struct 实例不能活得比借用的数据久 --");
    println!("  若尝试把 processor 带出底层数组的作用域，编译器会报 E0597");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Lifetimes in Structs
// -------------------------------------------

/*
1. Each paramter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of self is assigned to all output lifetime parameters.
*/
struct ArrayProcessor<'a> {
    data: &'a [i32],
}
impl<'a> ArrayProcessor<'a> {
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
        let previous_data = self.data;
        self.data = new_data;
        previous_data
    }
}
fn main() {
    let mut some_data = ArrayProcessor { data: &[4, 5, 6] };

    let previous_data = some_data.update_data(&[5, 8, 10]);
    println!("Previous data: {:?}", previous_data);
    println!("New data: {:?}", some_data.data);
}

/*
-------------------------------------------------------------------------------------------------------------
Concept / Topic                 | Explanation
--------------------------------|-----------------------------------------------------------------------------------
Lifetimes in Structs            | Structs that store references must declare lifetime parameters.
                                | This ensures the struct does not outlive the referenced data.
                                | The lifetime parameter becomes part of the struct’s type.

Impl Block constraits           | The impl block for structs must include the generic lifetime used by structs.
                                | This ties all method behavior to the struct’s stored reference lifetime.
-------------------------------------------------------------------------------------------------------------
*/
"###;
