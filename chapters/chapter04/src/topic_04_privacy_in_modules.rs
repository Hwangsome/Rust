//! 模块的隐私：`pub struct` 并不等于字段也自动公开。
//!
//! 这是 Rust 初学者最容易踩的坑之一。**类型**和**字段**的可见性是两回事：
//!
//! - `pub struct Product { ... }`：外部可以**看到这个类型的名字**、可以用方法、可以当参数传
//! - 但字段仍然默认**私有**：外部写 `Product { id: 1, ... }` 会失败
//! - 所以通常配合 **构造函数** 或 **字段 getter/setter** 来暴露受控访问
//!
//! 同样的规则对 enum 也成立：
//! - `pub enum Category { A, B }`：variant 会**自动**跟着 `pub`（和 struct 字段不同！）
//! - 这是 enum 和 struct 的**关键差异**

pub use self::catalog::{Category, Product};

mod catalog {
    /// enum 变体会跟着 `pub` 一起公开——这一点和 struct 字段刚好相反。
    #[derive(Debug)]
    pub enum Category {
        Electronics,
        Books,
        Clothing,
    }

    /// struct 用 `pub` 只公开了**类型名**，字段仍然私有。
    #[derive(Debug)]
    pub struct Product {
        id: u32,
        name: String,
        category: Category,
        // 私有字段：外部既看不到也改不了，必须通过方法访问。
        internal_stock: u32,
    }

    impl Product {
        /// 构造函数：把"创建实例"这件事集中到一个地方，方便加校验。
        pub fn new(id: u32, name: String, category: Category) -> Self {
            Self {
                id,
                name,
                category,
                internal_stock: 0,
            }
        }

        /// getter：受控读取私有字段。
        pub fn name(&self) -> &str {
            &self.name
        }

        /// setter：带校验的写入——外部改不了私有字段，只能通过这里。
        pub fn restock(&mut self, delta: u32) -> u32 {
            self.internal_stock = self.internal_stock.saturating_add(delta);
            self.internal_stock
        }

        pub fn summary(&self) -> String {
            format!(
                "id={}, name={}, category={:?}, stock={}",
                self.id, self.name, self.category, self.internal_stock
            )
        }
    }
}

pub fn run() {
    println!("== Privacy In Modules ==");

    println!("-- (1) pub struct：类型公开，字段私有 --");
    let mut laptop = Product::new(1, String::from("Laptop"), Category::Electronics);
    println!("{}", laptop.summary());

    // 下面这行如果取消注释，会得到 E0451：字段 `internal_stock` 不可见。
    // laptop.internal_stock = 10;
    println!();

    println!("-- (2) 用公开方法来访问私有字段 --");
    laptop.restock(20);
    laptop.restock(5);
    println!("访问 name() 的 getter: {}", laptop.name());
    println!("restock 之后: {}", laptop.summary());
    println!();

    println!("-- (3) pub enum：variant 会跟着公开 --");
    // 这里的 Category::Books 之所以能直接写出来，是因为 `pub enum` 的所有 variant 都是 pub。
    let book = Product::new(2, String::from("Rust Book"), Category::Books);
    println!("{}", book.summary());
    println!();

    println!("-- 关键结论 --");
    println!("  pub struct 不等于字段自动公开——要通过构造函数 + 方法暴露受控访问。");
    println!("  pub enum 的 variant 自动跟着公开——这是 enum / struct 的关键差异。");
    println!();
}
