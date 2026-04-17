// 单元测试通常与被测代码放在同一个文件中，
// 这样最容易就近阅读“实现”和“验证”之间的关系。
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn run() {
    println!("== Unit Testing ==");
    println!("add(2, 3) = {}", add(2, 3));
    println!("单元测试通常和被测代码放在同一个文件里。");
    println!();
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn add_returns_sum() {
        // 单元测试最重要的是断言“行为符合预期”，而不是打印大量输出。
        assert_eq!(add(2, 3), 5);
    }
}
