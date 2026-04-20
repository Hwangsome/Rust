//! 关联类型 vs 泛型参数：什么时候选哪个？
//!
//! 经验判据（**一句话**）：
//!
//! > **对实现者来说，该类型是"唯一的"还是"可以多种"？**
//! > - 唯一 → 用**关联类型**
//! > - 可以有多种 → 用**泛型参数**
//!
//! ## 两个代表性案例
//!
//! ### 案例 A：`Iterator`——每个迭代器产出的 Item 只有一种 → 关联类型
//!
//! ```ignore
//! trait Iterator {
//!     type Item;
//!     fn next(&mut self) -> Option<Self::Item>;
//! }
//! ```
//!
//! `Vec<i32>::into_iter()` 返回 `Item = i32` 的迭代器，不可能"这个 Vec 同时既产 i32 又产 String"。
//!
//! ### 案例 B：`Add<Rhs>`——同一个类型可能和**多种右操作数**做加法 → 泛型参数
//!
//! ```ignore
//! trait Add<Rhs = Self> {
//!     type Output;
//!     fn add(self, rhs: Rhs) -> Self::Output;
//! }
//! ```
//!
//! `f64` 同时实现了 `Add<f64>`、`Add<&f64>`——同一个类型被允许拥有多个 `Add` impl。
//! 如果 `Rhs` 写成关联类型就做不到这一点（只能选一个）。

trait PeekableCollection {
    /// 元素类型对每个容器都是确定的 —— 关联类型是自然选择。
    type Item;
    fn first(&self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct Bag(Vec<&'static str>);

#[derive(Debug)]
struct IntBag(Vec<i32>);

impl PeekableCollection for Bag {
    type Item = &'static str;
    fn first(&self) -> Option<Self::Item> { self.0.first().copied() }
}

impl PeekableCollection for IntBag {
    type Item = i32;
    fn first(&self) -> Option<Self::Item> { self.0.first().copied() }
}

#[derive(Debug, Clone, Copy)]
struct Score(u32);

/// 同一个 Score 可能和 Score 相加，也可能和 u32 相加 —— 右值类型是"多种的"，泛型参数才合适。
trait Combine<Rhs> {
    type Output;
    fn combine(self, rhs: Rhs) -> Self::Output;
}

impl Combine<Score> for Score {
    type Output = u32;
    fn combine(self, rhs: Score) -> Self::Output { self.0 + rhs.0 }
}

impl Combine<u32> for Score {
    type Output = u32;
    fn combine(self, rhs: u32) -> Self::Output { self.0 + rhs }
}

impl Combine<i64> for Score {
    type Output = i64;
    fn combine(self, rhs: i64) -> Self::Output { self.0 as i64 + rhs }
}

pub fn run() {
    println!("== Choosing Associated vs Generic Type ==");

    println!("-- (A) 唯一的 item 类型 → 关联类型 --");
    let bag = Bag(vec!["rust", "trait", "iterator"]);
    let ints = IntBag(vec![10, 20, 30]);
    println!("bag.first()  = {:?}", bag.first());
    println!("ints.first() = {:?}", ints.first());
    println!();

    println!("-- (B) 多种 Rhs 类型 → 泛型参数 --");
    let score = Score(10);
    println!("score.combine(Score(5))  = {} (Output=u32)", score.combine(Score(5)));
    println!("score.combine(2u32)      = {} (Output=u32)", score.combine(2_u32));
    println!("score.combine(3i64)      = {} (Output=i64)", score.combine(3_i64));
    println!();

    println!("-- 选型清单 --");
    println!("  标准库的 Iterator::Item、Deref::Target、Add::Output：都是关联类型");
    println!("  标准库的 Add<Rhs>、Sub<Rhs>、From<T>、Into<T>：都是泛型参数");
    println!("  （`Add::Output` 是关联类型，`Add<Rhs>` 的 Rhs 是泛型参数——组合使用）");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -----------------------------------------------------
// 		Choosing Associated vs Generic Types
// -----------------------------------------------------

trait Addition<Rhs, Output> {
    // type Rhs;
    // type Output;
    fn add(self, rhs: Rhs) -> Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}
fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3: Point = p1.add(p2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let p3 = p1.add(2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let line: Line = p1.add(p2);

    assert!(line.start.x == 1 && line.start.y == 1 && line.end.x == 2 && line.end.y == 2);
}

/*
------------------------------------------------------------------------------------------------------------------
Concept/Topic                | Explanation
-----------------------------|------------------------------------------------------------------------------------
Associated Types Limitation  | Associated types allow only one implementation of a trait per concrete type.
                             | Once associated types are fixed, they cannot vary across implementations.

Using Generics Flexibility   | Converting associated types into generics enables multiple implementations.
                             | Each implementation can choose different generic parameters.

Associated Types vs Generics | Use associated types when a trait has exactly one logical implementation per type.
                             | Use generics when multiple implementations per type are required.
------------------------------------------------------------------------------------------------------------------
 */
"###;
