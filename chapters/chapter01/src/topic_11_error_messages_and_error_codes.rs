//! 错误信息与错误码：怎么读 Rust 编译器的报错。
//!
//! Rust 的报错信息是新手最快进步的地方——它不仅告诉你哪里错，
//! 通常还会给出"建议怎么改"。本节帮你建立**阅读报错**的固定流程：
//!
//! 1. 看**错误码**（E0308、E0382、E0499……）
//! 2. 看**错误摘要**（这一行通常一两句话就能说清问题）
//! 3. 看**`-->` 定位**（哪个文件、哪一行）
//! 4. 看**带箭头的源码片段**（编译器把相关代码画出来）
//! 5. 看 `help:` / `note:` / `consider` 开头的**建议行**——常常直接给你答案
//! 6. 用 `rustc --explain EXXXX` 查详细解释
//!
//! 本章要保持可运行，所以我们不在代码里真的触发错误，而是以字符串形式把错误的**源代码片段**
//! 与**对应报错**打印出来，让你反复对照练习。

/// 打印"错误场景 + 代码片段 + 典型报错"三联。
fn scenario(title: &str, code: &str, error_excerpt: &str) {
    println!("▷ {title}");
    println!("  code:");
    for line in code.lines() {
        println!("    {line}");
    }
    println!("  typical error:");
    for line in error_excerpt.lines() {
        println!("    {line}");
    }
    println!();
}

pub fn run() {
    println!("-- 常见错误码对照 --\n");

    scenario(
        "E0308 类型不匹配",
        r#"let x: i32 = String::from("hello");"#,
        "error[E0308]: mismatched types\n  expected `i32`, found `String`",
    );

    scenario(
        "E0384 重新赋值不可变变量",
        "let x = 5;\nx = 6;",
        "error[E0384]: cannot assign twice to immutable variable `x`\n  help: consider making this binding mutable: `mut x`",
    );

    scenario(
        "E0382 使用已被 move 的值",
        "let s1 = String::from(\"hello\");\nlet s2 = s1;\nprintln!(\"{s1}\");",
        "error[E0382]: borrow of moved value: `s1`\n  note: move occurs because `s1` has type `String`, which does not implement the `Copy` trait",
    );

    scenario(
        "E0502 同时持有 & 和 &mut",
        "let mut v = vec![1, 2, 3];\nlet r = &v;\nv.push(4);\nprintln!(\"{r:?}\");",
        "error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable",
    );

    scenario(
        "E0425 找不到标识符",
        "println!(\"{y}\");",
        "error[E0425]: cannot find value `y` in this scope",
    );

    println!("-- 自救流程 --");
    println!("1. 抄下错误码 → 用 `rustc --explain E0308` 查官方解释");
    println!("2. 看 `help:` 行，它通常就是修复建议");
    println!("3. 记住最常见的 4~5 个错误码——入门阶段 80% 的报错都在里面");
    println!();
}
