//! 发布 crate 到 crates.io：流程、检查清单、版本策略。
//!
//! 这一节只讲流程和心智模型，不真的触发发布（教学项目没有发布目标）。
//! 但把这一章读完以后，你应该能独立把一个真正的 library crate 发布到 crates.io。
//!
//! ## 发布前检查清单
//!
//! 1. **package metadata**（Cargo.toml 里）
//!    - `name`（在 crates.io 唯一）、`version`（遵循 SemVer）
//!    - `authors`（可选）、`edition`、`rust-version`（最低支持 Rust 版本）
//!    - `description`、`license` 或 `license-file`、`repository`、`homepage`
//!    - `readme`、`keywords`、`categories`
//! 2. **文档**
//!    - 根文件顶部 `//!` 介绍 crate 的目标
//!    - 每个公开 item 写 `///` 文档注释
//!    - `cargo doc --open` 本地预览
//! 3. **测试**
//!    - `cargo test` 全部通过
//!    - `cargo check --all-targets` 没有 warning
//! 4. **清理**
//!    - 去掉多余的 `println!` / `dbg!`
//!    - 去掉未使用的依赖（`cargo machete` 可以帮你）
//! 5. **发布流程**
//!    - 首次：`cargo login <token>`（从 crates.io 账号后台拿 token）
//!    - 本地打包：`cargo package` 会生成 `.crate` 文件并验证能独立编译
//!    - 预演：`cargo publish --dry-run`
//!    - 正式：`cargo publish`
//!
//! ## SemVer 版本策略
//!
//! | 版本变化          | 代表什么                 |
//! |-----------------|----------------------|
//! | 补丁 `0.1.0 → 0.1.1` | Bug 修复，API 不变     |
//! | 次版 `0.1.1 → 0.2.0` | 新 API，**0.x 下的次版升级也被视为 breaking** |
//! | 主版 `1.2.3 → 2.0.0` | 不兼容改动             |
//!
//! **注意**：`0.x` 版本的每一次 `.x` 升级都被 Cargo 当作 breaking——这是最容易误判的地方。

pub fn run() {
    println!("== Publishing Your Crate ==");

    println!("-- (1) Cargo.toml metadata 最小集 --");
    println!("  [package]");
    println!("  name        = \"my-awesome-crate\"");
    println!("  version     = \"0.1.0\"");
    println!("  edition     = \"2021\"");
    println!("  description = \"A one-line description.\"");
    println!("  license     = \"MIT OR Apache-2.0\"");
    println!("  repository  = \"https://github.com/you/my-awesome-crate\"");
    println!();

    println!("-- (2) 发布命令 --");
    println!("  cargo login <token>      # 首次");
    println!("  cargo package            # 本地打包校验");
    println!("  cargo publish --dry-run  # 预演");
    println!("  cargo publish            # 正式发布");
    println!();

    println!("-- (3) SemVer 要点 --");
    println!("  0.x → 0.(x+1)：被当作 breaking（初期阶段的约定）");
    println!("  1.x → 1.(x+1)：仅新增 API，调用方平滑升级");
    println!("  1.y → 2.0   ：breaking，需要升级指南");
    println!();

    println!("-- (4) 常见失败原因 --");
    println!("  - name 已被占用（注册中心名字全局唯一）");
    println!("  - license 字段缺失");
    println!("  - 存在路径依赖（path = \"...\"）而没有对应的 version，无法上传");
    println!("  - 存在 git 依赖，同样需要替换为已发布版本");
    println!();

    println!("-- (5) yank 与撤回 --");
    println!("  cargo yank --vers 0.1.0      # 让新的依赖解析跳过这个版本（但老 Cargo.lock 仍可用）");
    println!("  不能真正删除，只能 yank。发布前一定要仔细检查。");
    println!();
}
