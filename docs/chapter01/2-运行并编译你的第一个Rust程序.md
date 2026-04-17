# 2. 运行并编译你的第一个 Rust 程序

> 类型：**Study note**
> 目标：搞清楚 Rust 源码是怎么变成二进制，又是怎么真正跑起来的。
> 上一篇：[1. Rust 环境配置与安装比较](./1-Rust环境配置与安装比较.md)

## 本节导航

- [返回第 1 章总览](./README.md)
- [上一节：1. Rust 环境配置与安装比较](./1-Rust环境配置与安装比较.md)

## 对应代码

- Cargo workspace: [Cargo.toml](../../Cargo.toml)
- Cargo package: `chapter01`
- Run chapter: `cargo run -p chapter01`
- Chapter entry: [chapters/chapter01/src/main.rs](../../chapters/chapter01/src/main.rs)
- Reference module: [chapters/chapter01/src/first_program.rs](../../chapters/chapter01/src/first_program.rs)
- Chapter lab: [chapters/chapter01/src/lab.rs](../../chapters/chapter01/src/lab.rs)

当前仓库采用“每章一个 Cargo package”的结构，所以本节运行的是 `chapter01` 这个 package，而不是根目录 `src/main.rs`。在当前整理方式下，环境配置、首个程序、变量、函数和控制流都统一归到 `chapter01 = Quick Startup`。

## 一分钟结论

- **`cargo run -p chapter01` = 构建 debug 版本并运行**
- **`cargo build -p chapter01 --release` = 只构建 release，不会自动运行**
- **`cargo check -p chapter01` = 更轻量的语义检查**
- **`rustc` 更适合理解单文件编译过程，`cargo` 更适合真实项目**
- 在当前仓库里，生成的二进制默认名是 **`chapter01`**，因为 package 名就是 `chapter01`

## 学习目标

- 分清 **compiling** 和 **running**
- 分清 `rustc` 和 `cargo`
- 记住 debug / release 产物分别放在哪里
- 能看懂 chapter crate 在 workspace 里是怎么运行的

## 先看当前结构

当前仓库不是“一个根 crate + 很多悬空文件”，而是：

```text
Cargo.toml                    # workspace
chapters/
└── chapter01/
    ├── Cargo.toml            # package = chapter01
    └── src/
        ├── main.rs           # 本章入口
        └── first_program.rs  # 本节主题模块
```

`chapters/chapter01/src/main.rs` 负责启动本章：

```rust
mod first_program;

fn main() {
    println!("Chapter 01: 环境与第一个程序");
    println!();
    first_program::run();
}
```

`chapters/chapter01/src/first_program.rs` 负责演示本节主题：

```rust
pub fn run() {
    println!("== First Program ==");
    println!("Hello, world!");
    println!();
}
```

## 定义

| 术语 / 命令 | 在当前仓库里代表什么 |
| --- | --- |
| **compiling** | 把 `chapter01` 的 Rust 源码构建成二进制产物 |
| **running** | 启动已经生成的二进制，让 `main` 真正执行 |
| `rustc` | 单文件层面的编译器 |
| `cargo` | 项目层面的构建、检查和运行入口 |
| workspace | 用一个根 `Cargo.toml` 管理多个 chapter package |

## 作用

为什么要刻意区分这些词？

因为初学 Rust 时最常见的混乱就是：

- 看到 `Hello, world!`，却不知道它是怎么来的
- 以为 `cargo run` 是“直接执行源代码”
- 以为 `cargo build` 和 `cargo run` 差不多
- 不清楚 workspace 里到底是哪个 package 在运行

把这些动作拆开以后，命令和 IDE 行为就不再像黑盒。

## 原理

把这次实验压成两条链路就够了。

### 链路 1：开发时最常用

```text
chapters/chapter01/src/main.rs
  -> cargo run -p chapter01
  -> target/debug/chapter01
  -> Chapter 01: 环境与第一个程序
  -> == First Program ==
  -> Hello, world!
```

### 链路 2：发布或观察 release 构建

```text
chapters/chapter01/src/main.rs
  -> cargo build -p chapter01 --release
  -> target/release/chapter01
  -> 手动运行二进制
  -> Chapter 01: 环境与第一个程序
  -> == First Program ==
  -> Hello, world!
```

所以：

- `cargo run -p chapter01` 没有跳过编译
- 它只是把 “构建 + 运行” 合并成一个入口
- 运行的入口是 `chapter01` 这个 package，不是 workspace 根目录

## 你最应该记住的四个命令

| 命令 | 它做什么 | 什么时候用 |
| --- | --- | --- |
| `cargo run -p chapter01` | 构建 debug 并运行 `chapter01` | 改完代码想立刻看结果 |
| `cargo build -p chapter01` | 构建 debug，不运行 | 只想拿到产物或确认能编过 |
| `cargo build -p chapter01 --release` | 构建 release，不运行 | 想看优化后的产物 |
| `cargo check -p chapter01` | 做语义检查，不真正运行程序 | 高频、快速检查 |

如果你想对整个仓库做一次结构级检查，可以再用：

```bash
cargo check --workspace
```

## `rustc` 该怎么理解

`rustc` 最适合用来理解“一个 `.rs` 文件怎么变成一个可执行文件”：

```bash
rustc hello.rs
./hello
```

但在真实项目里，它不是主入口。真实项目需要：

- 项目结构
- 依赖管理
- 多 package 管理
- debug / release 区分
- 测试和检查命令

这些都是 `cargo` 擅长的部分。

## 示例

### 示例 1：直接跑本章 package

```bash
cargo run -p chapter01
```

你最应该盯住的不是最后那句 `Hello, world!`，而是这条命令背后的事实：

- Cargo 会先构建 `chapter01`
- 默认构建的是 `dev` profile
- 然后再运行 `target/debug/chapter01`

### 示例 2：只构建 release

```bash
cargo build -p chapter01 --release
```

这条命令只有“构建”动作，没有“运行”动作，所以不会自动打印程序输出。

如果要验证 release 产物，你需要手动执行：

```bash
./target/release/chapter01
```

### 示例 3：快速确认代码是否还能过检查

```bash
cargo check -p chapter01
```

这很适合你频繁改代码时用来快速确认“至少目前语义层面没炸”。

## `cargo` vs `rustc`

| 对比项 | `rustc` | `cargo` |
| --- | --- | --- |
| 关注对象 | 一个源文件怎么编译 | 一个项目或一个 package 怎么构建和运行 |
| 适合场景 | 学编译过程、最小示例 | 日常开发、真实项目 |
| 管理依赖 | 否 | 是 |
| 管理 profile | 否 | 是 |
| 管理 workspace | 否 | 是 |

## debug 和 release 的区别

当前仓库里最先要记住的不是优化细节，而是这两个事实：

- debug 版本更适合开发阶段，构建快、调试信息更完整
- release 版本会做优化，更接近发布形态

默认产物位置：

```text
target/debug/chapter01
target/release/chapter01
```

这两个路径都位于 workspace 根目录下的 `target/`，而不是 `chapters/chapter01/target/`。

## 在 IDE 里怎么理解

如果你用 RustRover，这一章的运行入口是：

- package：`chapter01`
- 入口文件：`chapters/chapter01/src/main.rs`

`first_program.rs` 不是独立入口，它是被 `main.rs` 调用的主题模块。

这也是当前仓库选择“每章一个 Cargo package”的原因：

- IDE 能直接按 chapter 运行
- 模块跳转和补全正常
- 不会再出现悬空 `.rs` 文件缺少模块声明的问题

## 常见误解

### 误解 1：`cargo run -p chapter01` 直接执行源码

不对。  
它执行的是 Cargo 先构建出的二进制，再运行这个二进制。

### 误解 2：`cargo build -p chapter01 --release` 会自动打印程序结果

不对。  
它只负责构建，不负责运行。

### 误解 3：每个 `.rs` 文件都会变成一个独立程序

不对。  
在当前结构下，独立程序是 `chapter01` 这个 package；`first_program.rs` 只是它的一个模块。

## 复盘时最值得记住的结论

- Rust 代码不是“解释执行”，而是先编译再运行
- `cargo run` 本质上是“构建 + 运行”
- `cargo build --release` 本质上是“只构建优化版产物”
- workspace 里的真正运行单元是 package
- 当前第 1 章运行的是 `chapter01`，不是根目录 `src/main.rs`

## 复盘模板

以后回看这一节时，可以快速检查这几个问题：

1. 我能不能说清楚 `compiling` 和 `running` 的区别？
2. 我知不知道 `cargo run -p chapter01` 最终运行的是哪个二进制？
3. 我知不知道 debug 和 release 产物分别在哪？
4. 我能不能解释为什么当前仓库要用 chapter crate，而不是把示例全堆在根目录？

## 下一步

- 回到目录：[第 1 章：环境与第一个程序](./README.md)
- 继续进入第 2 章变量与数据类型，开始写真正的基础语法示例
