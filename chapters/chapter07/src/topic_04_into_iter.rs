//! `IntoIterator`：决定一个类型"怎样被展开成迭代器"。
//!
//! ```ignore
//! trait IntoIterator {
//!     type Item;
//!     type IntoIter: Iterator<Item = Self::Item>;
//!     fn into_iter(self) -> Self::IntoIter;
//! }
//! ```
//!
//! `for x in collection { ... }` 实际上被编译器翻译成：
//!
//! ```ignore
//! let mut iter = collection.into_iter();
//! while let Some(x) = iter.next() { ... }
//! ```
//!
//! 所以任何想支持 `for` 循环的类型，都需要实现 `IntoIterator`。
//!
//! Rust 标准库给 `Vec<T>` 实现了**三份** `IntoIterator`：
//!
//! | 接收者形态     | 产出                   | 示例                 |
//! |-------------|----------------------|--------------------|
//! | `Vec<T>`   | `T`（按值，消费 Vec） | `for x in v`       |
//! | `&Vec<T>`  | `&T`（只读借用）        | `for x in &v`      |
//! | `&mut Vec<T>` | `&mut T`（可变借用）   | `for x in &mut v`  |
//!
//! 这就是为什么同一个 `Vec` 可以被 `for x in v` / `for x in &v` / `for x in &mut v` 三种方式遍历。

struct Playlist {
    songs: Vec<String>,
}

impl Playlist {
    fn new(songs: Vec<String>) -> Self {
        Self { songs }
    }
}

/// 按值消费：迭代后 `Playlist` 本身会被 drop。
impl IntoIterator for Playlist {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.songs.into_iter()
    }
}

/// 只读借用迭代：`for song in &playlist`。
impl<'a> IntoIterator for &'a Playlist {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.songs.iter()
    }
}

pub fn run() {
    println!("== IntoIterator ==");

    println!("-- (1) 按值消费（Vec<T>风格）--");
    let playlist = Playlist::new(vec![
        "Borrow Checker Blues".to_string(),
        "Trait Bound Jam".to_string(),
    ]);
    for song in playlist {
        println!("  song (owned) => {song}");
    }
    // println!("{playlist:?}"); // ← Playlist 已被 into_iter 消费，不能再用
    println!();

    println!("-- (2) 只读借用迭代（&Vec<T>风格）--");
    let mixtape = Playlist::new(vec!["Song A".to_string(), "Song B".to_string()]);
    for song in &mixtape {
        // 这里 song 是 &String，因为 IntoIterator for &Playlist 产出 &String
        println!("  song (borrowed) => {song}");
    }
    // 原 mixtape 仍然可用
    println!("迭代完之后 mixtape.songs.len() = {}", mixtape.songs.len());
    println!();

    println!("-- (3) 对 Vec<T> 的三种遍历方式 --");
    let v = vec![1, 2, 3];
    for x in &v { print!("{x} "); }  // &i32 → 1 2 3
    println!();
    for x in v.iter() { print!("{x} "); }  // 等价写法
    println!();
    let v2: Vec<i32> = v.iter().map(|x| x * 10).collect();
    println!("v2 = {v2:?}");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		IntoIterator
// -------------------------------------------
/*
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
*/
struct Book {
    title: String,
    author: String,
    genre: String,
}

// struct BookIterator {
//     properties: Vec<String>,
// }

// impl Iterator for BookIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

impl IntoIterator for Book {
    type Item = String;
    // type IntoIter = BookIterator;

    // fn into_iter(self) -> Self::IntoIter {
    //     BookIterator {
    //         properties: vec![self.title, self.author, self.genre],
    //     }
    // }

    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
    }
}

fn main() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let mut book_iterator = book.into_iter();

    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());

    for book_info in book_iterator {
        println!("{book_info}");
    }
}

/*
--------------------------------------------------------------------------------------------------
Concept/Topic             | Explanation
--------------------------|-----------------------------------------------------------------------
Iterator vs IntoIterator  | Iterator defines how to yield items sequentially via next().
                          | IntoIterator defines how a type can be converted into an iterator.
                          | Iterator → produces items whereas IntoIterator → produces an Iterator.

IntoIterator Trait        | Contains the method into_iter(self).
                          | It consumes self and returns another type which implements Iterator.

Iterator State Separation | Iteration logic is often separated from the main struct.
                          | A dedicated iterator struct stores iteration state.
                          | Keeps the original type clean and focused on data representation.
--------------------------------------------------------------------------------------------------
 */
"###;
