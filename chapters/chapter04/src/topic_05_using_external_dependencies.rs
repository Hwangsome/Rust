//! 使用外部依赖：`Cargo.toml` 的 `[dependencies]` 与 `cargo add`。
//!
//! 本节不绑定具体第三方 crate（教学项目不引入额外依赖，保持 `cargo run` 干净）。
//! 但我们把**所有需要知道的流程**都列出来，确保你在自己项目里能一次搞定。
//!
//! ## 流程总览
//!
//! 1. **搜索**：访问 `https://crates.io` 或 `https://lib.rs`，搜 crate 名
//! 2. **查文档**：访问 `https://docs.rs/<crate-name>`，确认 API 和用法
//! 3. **添加依赖**：命令行执行 `cargo add <crate-name>`（或手写 Cargo.toml）
//! 4. **启用 features**：`cargo add serde --features derive`
//! 5. **引入作用域**：代码里用 `use <crate>::...;`
//! 6. **锁定版本**：`Cargo.lock` 自动生成，提交到 Git 以保证可复现
//!
//! ## Cargo.toml 片段示例
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! anyhow = "1"
//! tokio  = { version = "1", features = ["full"] }
//!
//! [dev-dependencies]
//! # 只在 `cargo test` / benchmark 时编译的依赖
//! criterion = "0.5"
//!
//! [build-dependencies]
//! # 构建脚本 build.rs 用到的依赖
//! cc = "1"
//! ```
//!
//! ## 三类依赖的区别
//!
//! | 段                   | 什么时候编译          | 典型用途                 |
//! |---------------------|-----------------|----------------------|
//! | `[dependencies]`    | 所有 build         | 应用逻辑、运行时依赖         |
//! | `[dev-dependencies]`| 只在 test/bench    | 测试工具、基准测试、假数据     |
//! | `[build-dependencies]` | 只在 build.rs    | 构建脚本需要的代码         |

pub fn run() {
    println!("== Using External Dependencies ==");

    println!("-- (1) 搜索与文档 --");
    println!("  crates.io: 官方注册中心 (https://crates.io)");
    println!("  lib.rs:    社区整理的更易读版本 (https://lib.rs)");
    println!("  docs.rs:   自动生成的文档 (https://docs.rs/<crate>)");
    println!();

    println!("-- (2) 命令行添加 --");
    println!("  cargo add serde                       # 最新兼容版本");
    println!("  cargo add serde --features derive     # 启用某些 feature");
    println!("  cargo add serde_json@1.0              # 锁定版本前缀");
    println!("  cargo add --dev criterion             # 加到 [dev-dependencies]");
    println!("  cargo remove serde_json               # 移除");
    println!();

    println!("-- (3) 版本规则 --");
    println!("  serde = \"1\"       ≡ ^1.0.0     → 允许 1.x.y（SemVer 兼容）");
    println!("  serde = \"=1.0.203\" → 锁死精确版本");
    println!("  serde = \">=1, <2\"  → 区间");
    println!();

    println!("-- (4) 在代码里使用 --");
    println!("  use serde::{{Serialize, Deserialize}};");
    println!("  #[derive(Serialize, Deserialize)]");
    println!("  struct Config {{ name: String, port: u16 }}");
    println!();

    println!("-- (5) Cargo.lock 的角色 --");
    println!("  Cargo.lock 记录了本次 build 实际解析出来的精确版本号。");
    println!("  binary crate 应该把 Cargo.lock 提交到 Git，以保证 CI / 新机器上可复现。");
    println!("  library crate 一般不提交 Cargo.lock，让下游使用者自己决定版本解析。");
    println!();
}
