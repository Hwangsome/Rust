//! `Box<T>`：把 T **放到堆上**，当前位置只留一个指针。
//!
//! `Box` 是最简单的**智能指针**——它独占所有权，和普通值的语义几乎一致，只是数据在堆上。
//!
//! 为什么需要 `Box`？典型 3 个场景：
//!
//! 1. **大对象**：避免在栈上放大块数据（Rust 默认栈 8MB，超过就栈溢出）
//! 2. **递归类型**：`enum List { Cons(i32, List), ... }` 直接写会让编译器无法计算 `List` 大小，
//!    因为它"可能包含另一个 List"——嵌套无底。Box 把下一节点放到堆上，当前节点只保留指针大小
//! 3. **trait object**：`Box<dyn Trait>` 在集合里统一存放不同类型（见下一节）
//!
//! 所有权语义和普通值相同：按 move 传递、离开作用域时堆上数据自动释放（Drop）。

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn len(&self) -> usize {
        match self {
            Self::Cons(_, next) => 1 + next.len(),
            Self::Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            Self::Cons(value, next) => *value + next.sum(),
            Self::Nil => 0,
        }
    }
}

pub fn run() {
    println!("== Box Smart Pointer ==");

    println!("-- (1) 堆上的 i32 --");
    let heap_int: Box<i32> = Box::new(5);
    println!("*heap_int = {}", *heap_int); // 自动解引用也能写 heap_int + 1 等

    println!("-- (2) 递归 enum 需要 Box --");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("list = {list:?}");
    println!("len = {}, sum = {}", list.len(), list.sum());
    println!();

    println!("-- (3) Box 是独占所有权 --");
    let b = Box::new(42);
    let b2 = b; // move；b 不再可用
    println!("b2 = {}", *b2);
    println!();

    println!("-- (4) drop 自动释放堆内存 --");
    {
        let _tmp = Box::new(vec![0u8; 1024 * 1024]); // 1MB 在堆上
        // 离开作用域时 _tmp 被 drop，堆内存自动归还
    }
    println!("临时的 1MB Box 已被释放");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
//         Box Smart Pointer (Part 1)
// -------------------------------------------

//       Simple Pointer          ||         Smart Pointers
// ----------------------------------------------------------------------
// Just stores memory address    ||   Special capabilities
// Indicated by &                ||   Not just simple references
// Also called references        ||
// No special capabilities       ||

/*
enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk
}
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let x = 0.625;
    // let y = Box::new(x);
    // let z = &x;

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);
}

/* 
----------------------------------------------------------------------------------------------
Concept / Topic          | Explanation
-------------------------|--------------------------------------------------------------------
Pointer vs Smart Pointer | A regular pointer stores only a memory address.
                         | References in Rust are simple pointers without extra capabilities.
                         | Smart pointers provide additional behavior and metadata.

Box Smart Pointer        | Box<T> stores data on the heap and gives its ownership rights.
                         | The Box itself is stored on the stack and points to heap memory.
                             
Use Case of Box          | Recursive types contain instances of themselves.
                         | For such types, the compiler is unable to infer the size.
                         | Wrapping recursive enum variants in a Box solves out the problem.
--------------------------------------------------------------------------------------------
*/
"###;
