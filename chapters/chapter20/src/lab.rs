pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：在一个新 crate 的 Cargo.toml 里加 [features]，用 `#[cfg(feature = \"x\")]` 控制代码分支");
    println!("▷ 练习 2：让一个 feature 依赖另一个 feature：`stat = [\"basic_math\"]`");
    println!("▷ 练习 3：让一个 feature 启用可选依赖：`advance = [\"dep:rust_math\"]`");
    println!();
}
