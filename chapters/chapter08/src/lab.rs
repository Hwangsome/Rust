//! 第 8 章练习：生命周期 + 智能指针组合使用。
//!
//! 做练习时永远问自己三个问题：
//! 1. **谁拥有这个值？**
//! 2. **它活多久？**
//! 3. **谁能修改它？**

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：具体生命周期 + NLL");
    println!("  - 写 `let r; {{ let x = 5; r = &x; }} println!(r);`，观察 E0597");
    println!("  - 改成 `let r = {{ let x = 5; x }};`，让它编译通过");

    println!();

    println!("▷ 练习 2：泛型生命周期");
    println!("  - 写 `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`");
    println!("  - 让它在跨作用域调用时被限制为较短输入的生命周期");

    println!();

    println!("▷ 练习 3：生命周期省略");
    println!("  - 写一个满足规则 2 的函数（单输入引用）");
    println!("  - 写一个 impl 里满足规则 3 的方法（&self + 其他引用）");
    println!("  - 写一个**不能**省略的场景，加手写 `<'a>` 让它通过");

    println!();

    println!("▷ 练习 4：Struct 里的生命周期");
    println!("  - 写 `struct Cursor<'a> {{ source: &'a str, pos: usize }}`");
    println!("  - 给它加 `next_char(&mut self) -> Option<char>`");

    println!();

    println!("▷ 练习 5：Box 与递归");
    println!("  - 定义 `enum Tree {{ Leaf(i32), Node(Box<Tree>, Box<Tree>) }}`");
    println!("  - 写 sum / depth / flatten 等方法");

    println!();

    println!("▷ 练习 6：Box<dyn Trait>");
    println!("  - 定义 trait Animal，实现 Dog/Cat/Cow");
    println!("  - 用 Vec<Box<dyn Animal>> 批量处理");

    println!();

    println!("▷ 练习 7：Rc 的共享");
    println!("  - `Rc<String>` 让 3 个变量同时拥有同一段文本");
    println!("  - 打印每步的 strong_count");

    println!();

    println!("▷ 练习 8：RefCell 内部可变");
    println!("  - 写 `struct Counter {{ n: RefCell<u64> }}`");
    println!("  - 方法 `fn incr(&self)` 用 `&self` 却能改 n");
    println!("  - 故意同时 borrow + borrow_mut，观察运行时 panic");

    println!();

    println!("▷ 练习 9：Rc<RefCell<T>> 组合");
    println!("  - 写一个简单的 `struct Counter`，让两个不同函数都能 increment 它");
    println!("  - 对比：如果只用 Rc（不套 RefCell），代码会怎样");

    println!();

    println!("完成标准：");
    println!("  - 能独立判断一个函数是否需要手写 `<'a>` 还是能省略");
    println!("  - 知道 Box / Rc / RefCell 各自解决哪一类问题");
    println!("  - 知道 Rc<RefCell<T>> vs Arc<Mutex<T>> 分别用于单/多线程");

    println!();
}
