// 这一节不发明新的业务逻辑，而是聚焦“怎样挑选、忽略、观察测试”。
pub fn run() {
    println!("== Controlling How Tests Are Run ==");
    println!("cargo test -p chapter05");
    println!("cargo test -p chapter05 add_returns_sum");
    println!("cargo test -p chapter05 -- --ignored");
    println!("cargo test -p chapter05 -- --show-output");
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn named_filter_demo() {
        // 这个名字本身就是给“按名称筛选测试”准备的。
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "radius should be positive")]
    fn should_panic_demo() {
        // `#[should_panic]` 让测试的“预期结果”变成 panic，而不是成功返回。
        panic!("radius should be positive");
    }

    #[test]
    #[ignore]
    fn ignored_demo() {
        // `#[ignore]` 适合默认不跑的慢测试或环境敏感测试。
        assert!(true);
    }
}
