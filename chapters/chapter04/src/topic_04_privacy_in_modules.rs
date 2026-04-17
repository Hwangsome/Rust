// 这一节把“公开类型”和“公开字段”明确区分开。
// 很多初学者会误以为 `pub struct` 就等于字段也全公开了。
pub use self::catalog::{Category, Product};

mod catalog {
    #[derive(Debug)]
    pub enum Category {
        Electronics,
        Books,
    }

    #[derive(Debug)]
    pub struct Product {
        id: u32,
        name: String,
        category: Category,
    }

    impl Product {
        // `new` 是一个常见的构造函数模式：
        // 字段继续保持私有，但外部仍然可以通过公开入口创建实例。
        pub fn new(id: u32, name: String, category: Category) -> Self {
            Self { id, name, category }
        }

        pub fn summary(&self) -> String {
            format!("id={}, name={}, category={:?}", self.id, self.name, self.category)
        }
    }
}

pub fn run() {
    println!("== Privacy In Modules ==");

    // 外部代码无法直接写 `Product { id: ..., ... }`，
    // 但可以通过 `Product::new(...)` 拿到合法实例。
    let product = Product::new(1, String::from("Laptop"), Category::Electronics);
    let book = Product::new(2, String::from("Rust Book"), Category::Books);
    println!("{}", product.summary());
    println!("{}", book.summary());
    println!("pub struct 不等于字段也自动公开，通常要通过构造函数或方法暴露访问。");
    println!();
}
