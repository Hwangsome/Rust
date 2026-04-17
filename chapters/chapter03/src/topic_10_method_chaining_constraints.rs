// 方法链看起来很顺，但它其实依赖“每一步返回的类型仍然支持下一步调用”。
// 一旦某个方法返回 `Result`，链的结构就会发生变化。
#[derive(Debug)]
struct TextBuilder {
    value: String,
}

impl TextBuilder {
    fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn append(mut self, text: &str) -> Self {
        // 这里按值拿走 `self`，是 builder 风格链式调用里很常见的写法。
        self.value.push_str(text);
        self
    }

    fn try_limit(self, max_len: usize) -> Result<Self, String> {
        if self.value.len() <= max_len {
            Ok(self)
        } else {
            Err(format!("length {} exceeds {max_len}", self.value.len()))
        }
    }
}

pub fn run() {
    println!("== Method Chaining Constraints ==");

    // 到这里还是纯粹的 builder 链。
    let success = TextBuilder::new().append("Rust").append(" Lang");
    println!("plain chaining result = {:?}", success);

    // `try_limit` 一旦返回 `Result`，后面就不再直接是 `TextBuilder` 了。
    let checked = TextBuilder::new()
        .append("Rust")
        .append(" Language")
        .try_limit(20);
    println!("once a method returns Result, chaining shape changes => {:?}", checked);
    println!();
}
