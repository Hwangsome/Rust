//! Least Upper Bound (LUB) coercion：**分支合流时**寻找"共同祖先类型"。
//!
//! 什么时候出现？
//!
//! ```ignore
//! let x = if cond { foo() } else { bar() };
//! //            ^^^^^       ^^^^^
//! //            类型 A       类型 B
//! ```
//!
//! 如果 A ≠ B，编译器会尝试找一个**两者都能 coerce 成的**类型：
//!
//! - A 和 B 都实现同一个 trait → `&dyn Trait`
//! - A 是 `!`（never） → 结果取 B 的类型
//! - A 是 `&str`、B 是 `&str` → 合并完全相同
//! - A / B 一个是 unsized 一个是 sized → 取 unsized 的公共引用
//!
//! 如果找不到共同类型 → E0308。

trait Render { fn render(&self) -> String; }
struct A;
struct B;

impl Render for A { fn render(&self) -> String { "A".into() } }
impl Render for B { fn render(&self) -> String { "B".into() } }

pub fn run() {
    println!("== Least Upper Bound Coercion ==");

    println!("-- (1) if 分支都返回 &'static str --");
    let cond = true;
    let s: &str = if cond { "yes" } else { "no" };
    println!("  s = {s}");
    println!();

    println!("-- (2) ! 类型参与：取另一支的类型 --");
    let n: i32 = if cond { 42 } else { panic!("bad") };
    //                                 ^^^^^^^^^^^^^ 返回 !
    println!("  n = {n} (分支 ! 被吸收进 i32)");
    println!();

    println!("-- (3) 需要显式 &dyn Trait 才能合流 --");
    let want_dyn = true;
    // 下面这行直接写会失败：
    // let r = if cond { &A } else { &B };
    // 因为 &A 和 &B 是两个不同类型；编译器不会"自动引入"&dyn Render。
    // 必须显式注明目标类型：
    let r: &dyn Render = if want_dyn { &A as &dyn Render } else { &B as &dyn Render };
    println!("  r.render() = {}", r.render());
    println!();

    println!("-- (4) 记忆 --");
    println!("  if/else、match 多分支，类型不同时 Rust 会尝试 LUB 合流；");
    println!("  但涉及 &dyn Trait 等需要显式 coerce 的场景，要自己加 `as &dyn T`。");
    println!();
}
