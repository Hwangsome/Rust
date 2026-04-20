//! `Box<T>` 的典型用法：大对象上堆、trait object 异构集合、返回递归/动态大小类型。

trait Storage {
    fn description(&self) -> String;
    fn capacity_gb(&self) -> u32;
}

struct DiskStorage { size_gb: u32 }
struct CloudStorage { provider: &'static str }
struct TapeStorage;

impl Storage for DiskStorage {
    fn description(&self) -> String { format!("disk ({} GB)", self.size_gb) }
    fn capacity_gb(&self) -> u32 { self.size_gb }
}
impl Storage for CloudStorage {
    fn description(&self) -> String { format!("cloud: {}", self.provider) }
    fn capacity_gb(&self) -> u32 { 0 } // "无限"，简化演示
}
impl Storage for TapeStorage {
    fn description(&self) -> String { "tape".into() }
    fn capacity_gb(&self) -> u32 { 500 }
}

/// 返回一个 `Box<dyn Storage>`：调用方只知道它实现了 Storage，不知道具体类型。
fn make_storage(kind: &str) -> Box<dyn Storage> {
    match kind {
        "disk" => Box::new(DiskStorage { size_gb: 1024 }),
        "cloud" => Box::new(CloudStorage { provider: "S3" }),
        _ => Box::new(TapeStorage),
    }
}

pub fn run() {
    println!("== Box Pointer Use Cases ==");

    println!("-- (1) 大对象放堆上 --");
    let boxed_numbers = Box::new([0_u8; 1024]);
    println!("boxed array length = {}", boxed_numbers.len());
    println!();

    println!("-- (2) Vec<Box<dyn Trait>> 异构集合 --");
    let stores: Vec<Box<dyn Storage>> = vec![
        Box::new(DiskStorage { size_gb: 512 }),
        Box::new(CloudStorage { provider: "S3" }),
        Box::new(TapeStorage),
    ];
    for s in &stores {
        println!("  {} (cap = {} GB)", s.description(), s.capacity_gb());
    }
    println!();

    println!("-- (3) 工厂函数返回 Box<dyn Trait> --");
    for kind in ["disk", "cloud", "other"] {
        let s = make_storage(kind);
        println!("  make_storage({kind:?}) => {}", s.description());
    }
    println!();

    println!("-- (4) Box<dyn Error> 是错误处理常见写法 --");
    println!("  fn do_stuff() -> Result<(), Box<dyn std::error::Error>>");
    println!("  可以同时容纳不同具体错误类型");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//         Box Smart Pointer (Part 2)
// -------------------------------------------

// Example 1
/* 
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );

    println!("{:?}", list);
}
*/

// Example 2
struct Huge_Data;
struct Small_Data;

trait Storage {}

impl Storage for Huge_Data {}
impl Storage for Small_Data {}

fn main() {
    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1;
    let data_4 = data_2;

    let data_5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}

/* 
--------------------------------------------------------------------------------------------------
Concept / Topic                | Explanation
-------------------------------|------------------------------------------------------------------
Issue in List enum             |The enum List { Cons(i32, Box<List>), Nil} 
                               | The terminating Nil variant caused unnecessary heap allocation.
                               | Wrapping the Cons variant by an Option, avoids this. 

Use Cases of Box           
1. Avoiding Copying Large Data | Moving a large stack-allocated struct copies all of its data.
                               | Moving a Box copies only the pointer stored on the stack.
                               | The heap allocation itself is not duplicated.
                               | This is beneficial for performance-sensitive code.

2. Heterogeneous Collections   | Vectors require all elements to have the same concrete type.
                               | Trait objects allow storing different types under a common trait.
                               | Box<dyn Trait> enables heterogeneous collections in vectors.
---------------------------------------------------------------------------------------------------
*/
"###;
