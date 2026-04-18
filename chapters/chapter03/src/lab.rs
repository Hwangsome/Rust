//! 第 3 章练习说明。
//!
//! 这一章把"自定义类型"和"标准库提供的类型"放到一起学。
//! 练习的核心是：**让类型表达业务含义**，而不只是"能用就行"。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：Struct 三种形式");
    println!("  - 写一个具名字段 struct `User`，一个元组结构体 `Meters(f64)`，一个单元结构体");
    println!("  - 给 `User` 派生 `Debug`，再用 `{{user:?}}` / `{{user:#?}}` 打印");

    println!();

    println!("▷ 练习 2：impl 里的四种函数");
    println!("  - 写一个关联函数 `new`");
    println!("  - 写一个 `&self` 方法做查询");
    println!("  - 写一个 `&mut self` 方法做修改");
    println!("  - 写一个 `self` 方法消费实例");
    println!("  - 分别调用，观察各自对实例所有权的影响");

    println!();

    println!("▷ 练习 3：Enum 带数据");
    println!("  - 写一个 `Shape` enum，包含 `Circle(f64)`、`Rect {{w,h}}`、`Point`");
    println!("  - 给它实现 `fn area(&self) -> f64`");
    println!("  - 用 match 穷尽处理；再刻意漏掉一个分支，观察 E0004");

    println!();

    println!("▷ 练习 4：Option / Result 的组合子");
    println!("  - 写 `fn first_positive(v: &[i32]) -> Option<i32>`");
    println!("  - 写 `fn parse_pair(s: &str) -> Result<(i32, i32), String>`，用 `?` 传播错误");
    println!("  - 用 `unwrap_or_else` 给 Result 一个兜底");

    println!();

    println!("▷ 练习 5：HashMap 典型惯用法");
    println!("  - 给一句话做词频统计：`entry(key).or_insert(0) += 1`");
    println!("  - 用 `iter()` 找出出现次数最多的单词");

    println!();

    println!("▷ 练习 6：模式匹配多场景");
    println!("  - `match` + 范围 `1..=5`");
    println!("  - `match` + 或模式 `1 | 2 | 3`");
    println!("  - `match` + 守卫 `x if x > 0`");
    println!("  - `if let` + `else`");
    println!("  - `while let` 消费 `Vec`");

    println!();

    println!("▷ 练习 7：解构 struct");
    println!("  - 写一个函数签名 `fn describe(User {{ name, age, .. }}: &User)`");
    println!("  - 让 `match` 同时匹配字段具体值和绑定变量");

    println!();

    println!("▷ 练习 8：引用的赋值行为");
    println!("  - 写 `let r1 = &x; let r2 = r1;`，确认 r1/r2 都能用（& 是 Copy）");
    println!("  - 写 `let r1 = &mut x; let r2 = r1;`，确认 r1 被 move，不能再用");

    println!();

    println!("▷ 练习 9：方法链");
    println!("  - 给 `TextBuilder` 加一个 `trim()` 方法（返回 Self）继续链");
    println!("  - 给它加一个 `try_assert_no_spaces()`（返回 Result）");
    println!("  - 写一个 `build_validated() -> Result<TextBuilder, String>` 用 `?` 串联多步");

    println!();

    println!("完成标准：");
    println!("  - 能独立写一个带 `new` + 方法 + Debug 派生的 struct");
    println!("  - 能独立写一个含数据的 enum + 穷尽 match");
    println!("  - 能用 Option/Result 的组合子替代半数以上的 match");
    println!("  - 能指出方法链在什么时候会被 Result 打断，并用两种方式把它接回去");

    println!();
}
