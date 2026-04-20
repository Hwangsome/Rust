//! 强转为 trait object：`&T` / `Box<T>` → `&dyn Trait` / `Box<dyn Trait>`。
//!
//! 这是上一章 `unsized coercion` 的一个特例，但足够常见值得单独看一遍：
//!
//! ```text
//! &ConcreteT                     →  &dyn Trait           （只要 T: Trait）
//! Box<ConcreteT>                 →  Box<dyn Trait>
//! Rc<ConcreteT>                  →  Rc<dyn Trait>
//! Arc<ConcreteT>                 →  Arc<dyn Trait>
//! ```
//!
//! 关键触发条件：T 必须实现 Trait，且 Trait 必须是 **object-safe**（见 chapter06）。

trait Speak { fn speak(&self) -> String; }

struct Dog;
struct Cat;

impl Speak for Dog { fn speak(&self) -> String { "Woof!".into() } }
impl Speak for Cat { fn speak(&self) -> String { "Meow!".into() } }

fn describe_ref(s: &dyn Speak) {
    println!("  ref: {}", s.speak());
}

fn describe_box(b: Box<dyn Speak>) {
    println!("  box: {}", b.speak());
}

pub fn run() {
    println!("== Trait Objects Coercion ==");

    let d = Dog;
    let c = Cat;

    println!("-- (1) &ConcreteT → &dyn Trait --");
    describe_ref(&d); // &Dog 隐式变 &dyn Speak
    describe_ref(&c); // &Cat 隐式变 &dyn Speak
    println!();

    println!("-- (2) Box<ConcreteT> → Box<dyn Trait> --");
    describe_box(Box::new(Dog)); // Box<Dog> 隐式变 Box<dyn Speak>
    describe_box(Box::new(Cat));
    println!();

    println!("-- (3) 集合里一次性装进来 --");
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Dog)];
    for a in &animals {
        println!("  loop: {}", a.speak());
    }
    println!();
}
