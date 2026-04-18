//! 集成测试示例：放在 crate 根目录的 `tests/` 下。
//!
//! 运行方式：
//! - `cargo test -p chapter05` —— 同时运行单元测试与集成测试
//! - `cargo test -p chapter05 --test smoke` —— 只跑本文件
//!
//! 注意点：
//! - 每个 `tests/*.rs` 文件会被 cargo 编译成**独立的测试二进制**
//! - 集成测试不能访问 crate 的私有 item（这点正好模拟"外部使用者"视角）
//! - 需要共享代码时，使用 `tests/common/mod.rs` 子目录约定

#[test]
fn smoke_arithmetic() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn smoke_string() {
    let s = String::from("hello");
    assert!(s.starts_with("hel"));
    assert_eq!(s.len(), 5);
}

#[test]
fn smoke_vec_iteration() {
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    assert_eq!(sum, 15);

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}
