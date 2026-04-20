//! 部分 object safe：**混合方法**的 trait 也能做成 dyn。
//!
//! 技巧：给"违规方法"加 `where Self: Sized`，让它**只能通过具体类型**调用，不通过 dyn。
//! 这样 trait 整体仍保持 object-safe，部分方法只能走泛型/具体类型路径。

trait Catalog {
    fn describe(&self) -> String; // object-safe

    fn new_default() -> Self
    where
        Self: Sized, // 把这个方法排除在 dyn 之外
    {
        unimplemented!()
    }

    fn sum_generic<T>(&self, _x: T)
    where
        Self: Sized, // 排除带泛型的方法
    {
    }
}

struct Entry(&'static str);
impl Catalog for Entry {
    fn describe(&self) -> String { format!("entry={}", self.0) }
}

pub fn run() {
    println!("== Partial Object Safety ==");

    let items: Vec<Box<dyn Catalog>> = vec![Box::new(Entry("x")), Box::new(Entry("y"))];
    for it in &items { println!("  {}", it.describe()); }

    println!("  加 `where Self: Sized` 让部分方法只在具体类型上可用，保持 trait object-safe");
    println!();
}
