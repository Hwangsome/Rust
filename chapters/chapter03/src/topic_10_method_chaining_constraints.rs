//! 方法链的约束：为什么链条在某一步会"断掉"？
//!
//! 方法链（method chaining）是 Rust 非常常见的写法，比如：
//!
//! ```ignore
//! let result = vec![1, 2, 3]
//!     .iter()
//!     .map(|x| x * 2)
//!     .filter(|x| *x > 2)
//!     .sum::<i32>();
//! ```
//!
//! 链条能"连下去"的条件是：**每一步的返回类型都必须是"允许下一步调用"的类型**。
//! 一旦某步返回了 `Option` / `Result`，形状就变了，必须用 `?`、`match`、`map`、`and_then` 等
//! 方式把它"接回去"。
//!
//! 本节通过一个简化的 `TextBuilder` 演示：
//! 1. 返回 `Self` 的方法可以无限链
//! 2. 返回 `Result<Self, E>` 的方法会打断普通链
//! 3. 两种恢复链条的办法：`?`（需要函数返回 Result）与组合子（`and_then` 等）

#[derive(Debug)]
struct TextBuilder {
    value: String,
}

impl TextBuilder {
    fn new() -> Self {
        Self { value: String::new() }
    }

    /// 按值接收 `self`，这样调用后"旧 builder"就被消费了，避免残留陈旧状态。
    /// 返回 `Self` 让链可以无限延续。
    fn append(mut self, text: &str) -> Self {
        self.value.push_str(text);
        self
    }

    /// 返回 `Self` 的另一个方法，同样保持链。
    fn uppercase(self) -> Self {
        Self { value: self.value.to_uppercase() }
    }

    /// 返回 `Result<Self, E>`：链到这里"断"了——下一步不能直接继续 `.append(...)`。
    fn try_limit(self, max_len: usize) -> Result<Self, String> {
        if self.value.len() <= max_len {
            Ok(self)
        } else {
            Err(format!("length {} exceeds {max_len}", self.value.len()))
        }
    }
}

/// 恢复办法 A：外部函数返回 `Result`，链里就可以用 `?` 把 Result 拆开。
fn build_bounded() -> Result<TextBuilder, String> {
    let builder = TextBuilder::new()
        .append("Rust")
        .append(" Language")
        .try_limit(20)?        // ← ? 把 Result<Self, E> 拆回 Self
        .append(" is safe");   //   于是下一步可以继续链
    Ok(builder)
}

pub fn run() {
    println!("== Method Chaining Constraints ==");

    println!("-- (1) 只要每步返回 Self，链可以无限延续 --");
    let simple = TextBuilder::new()
        .append("Hello")
        .append(", ")
        .append("world")
        .uppercase();
    println!("simple = {simple:?}");
    println!();

    println!("-- (2) 某一步返回 Result，链形状变了 --");
    let checked = TextBuilder::new()
        .append("Rust")
        .append(" Language")
        .try_limit(20); // 整个表达式的类型变成了 Result<TextBuilder, String>
    println!("checked = {checked:?}");
    // 注意：这里**不能**直接写 `.try_limit(20).append("...")`——
    // 会报错 "no method named `append` on `Result<TextBuilder, String>`"。
    println!();

    println!("-- (3) 恢复 A：用 ? 展开 Result，继续链 --");
    println!("build_bounded() = {:?}", build_bounded());
    println!();

    println!("-- (4) 恢复 B：用组合子 map / and_then --");
    let using_map = TextBuilder::new()
        .append("OK")
        .try_limit(10)
        .map(|b| b.append("!")); // map 里还原 Self，整体还是 Result
    println!("using_map = {using_map:?}");

    let will_fail = TextBuilder::new()
        .append("This is a very long text")
        .try_limit(10)
        .map(|b| b.append("?")); // Err 时 map 不做任何事
    println!("will_fail = {will_fail:?}");
    println!();
}
