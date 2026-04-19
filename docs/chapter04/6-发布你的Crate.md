# 6. 发布你的 Crate

> - **所属章节**：第 4 章 · Organizing Your Code
> - **Cargo package**：`chapter04`
> - **运行方式**：`cargo run -p chapter04`
> - **代码位置**：`chapters/chapter04/src/topic_06_publishing_your_crate.rs`
> - **上一篇**：[5. 使用外部依赖](./5-使用外部依赖.md)
> - **下一篇**：本章最后一篇
> - **关键词**：`cargo publish`、crates.io、SemVer、`Cargo.toml` metadata、`cargo doc`

---

## 这一节解决什么问题

如何把自己写的库发布到 crates.io 让别人使用？发布之前需要准备什么？

---

## 发布前检查清单

```toml
# 必须有的字段
[package]
name = "my-awesome-lib"   # 全局唯一名字
version = "0.1.0"         # 遵循 SemVer
edition = "2021"

# 强烈建议有的字段
description = "一句话描述这个 crate 做什么"
license = "MIT OR Apache-2.0"    # 或 license-file = "LICENSE"
repository = "https://github.com/user/my-awesome-lib"
readme = "README.md"
keywords = ["rust", "awesome"]   # 最多5个
categories = ["algorithms"]      # crates.io 的分类

# 可选但有用
homepage = "https://my-awesome-lib.rs"
documentation = "https://docs.rs/my-awesome-lib"
authors = ["Your Name <you@example.com>"]
```

---

## 发布流程

```bash
# 1. 登录（只需一次）
cargo login <your-token>   # 从 crates.io 账户页面获取

# 2. 检查你的 crate 能否正常打包
cargo package

# 3. 预演（不真正发布）
cargo publish --dry-run

# 4. 正式发布
cargo publish

# 5. 撤回（yank）某个版本（不删除，只是标记不推荐）
cargo yank --version 0.1.0
```

---

## SemVer 语义版本规范

```
主版本.次版本.修订版本
  x  .  y   .   z

修订版本（z）：仅修复 bug，API 不变 → 1.0.0 → 1.0.1
次版本（y）：新增功能，API 向后兼容 → 1.0.0 → 1.1.0
主版本（x）：破坏性 API 变更       → 1.0.0 → 2.0.0

特殊规则：
  0.x.y：尚未稳定，任何次版本升级都可能是 breaking
```

---

## 完整运行示例

```rust
pub fn run() {
    println!("=== 发布到 crates.io ===");
    println!();
    println!("发布前检查项:");
    println!("  ✓ Cargo.toml 有 name / version / description / license");
    println!("  ✓ README.md 存在且内容清晰");
    println!("  ✓ cargo test 全部通过");
    println!("  ✓ 公开 API 都有 /// 文档注释");
    println!("  ✓ cargo doc --open 预览文档");
    println!();
    println!("发布命令:");
    println!("  cargo login <token>");
    println!("  cargo publish --dry-run  # 预演");
    println!("  cargo publish            # 正式发布");
    println!();
    println!("注意: 发布后无法删除版本，只能 yank（标记不推荐）");
    println!("所以发布前一定仔细检查！");
}
```

---

## 文档注释最佳实践

```rust
/// 这是公开 API 文档
///
/// # Examples
///
/// ```
/// use my_lib::add;
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # Panics
///
/// 当 `b == 0` 时 panic
///
/// # Errors
///
/// 如果输入超出范围，返回 `Err`
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 下一步

第 4 章完成！

- 回到目录：[第 4 章：代码组织](./README.md)
- 下一章：[第 5 章：测试](../chapter05/README.md)

