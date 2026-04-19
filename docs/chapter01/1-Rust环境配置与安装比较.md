# 1. Rust 环境配置与安装比较

> 类型：**Study note**
> 目标：装好 Rust，并且知道这套环境以后怎么维护、怎么验收、怎么排错。
> 下一篇：[2. 运行并编译你的第一个 Rust 程序](./2-运行并编译你的第一个Rust程序.md)

## 本节导航

- [返回第 1 章总览](./README.md)
- [继续阅读：2. 运行并编译你的第一个 Rust 程序](./2-运行并编译你的第一个Rust程序.md)

## 一分钟结论

- 默认方案直接选：`**rustup + stable`**
- 不要把 `**rustup` / `cargo` / `rustc**` 当成一回事
- 验收不要只看版本号，至少再跑一次 `**cargo run**`
- 交叉编译不是 “加个 target 就完了”，链接器和系统库经常才是真问题
- **安装方式** 和 **开发环境放在哪里** 是两个不同维度，不要混着讨论

## 学习目标

- 知道 Rust 工具链到底由哪些部分组成
- 能为个人学习机、团队机器、离线环境选合适方案
- 记住最小安装命令和最小验收命令
- 能快速判断环境问题大概卡在哪一层

## 证据来源

### 官方资料

- Rust 官方安装页：[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Rust Forge 其他安装方式：[https://forge.rust-lang.org/infra/other-installation-methods.html](https://forge.rust-lang.org/infra/other-installation-methods.html)
- rustup book：[https://rust-lang.github.io/rustup/](https://rust-lang.github.io/rustup/)

### 当前机器的实际结果

```bash
$ rustc --version
rustc 1.94.1 (e408947bf 2026-03-25)

$ cargo --version
cargo 1.94.1 (29ea6fb6a 2026-03-24)

$ rustup --version
rustup 1.29.0 (28d1352db 2026-03-05)

$ rustup show active-toolchain
stable-aarch64-apple-darwin (default)
```

```bash
$ rustup component list --installed
cargo-aarch64-apple-darwin
clippy-aarch64-apple-darwin
rust-docs-aarch64-apple-darwin
rust-src
rust-std-aarch64-apple-darwin
rustc-aarch64-apple-darwin
rustfmt-aarch64-apple-darwin
```

### 当前笔记没有直接验证的部分

- Windows 上 `MSVC` 工具链安装过程
- Linux 发行版包管理器安装的完整流程
- `rust-analyzer` 是否通过 `rustup` 安装到当前机器

## 知识点

### 定义

Rust 开发环境最容易混的三个名字：


| 名称       | 它是什么    | 它负责什么                                       | 不负责什么              |
| -------- | ------- | ------------------------------------------- | ------------------ |
| `rustc`  | 编译器     | 把 Rust 源码编译成二进制                             | 管理项目依赖、工具链版本       |
| `cargo`  | 项目工作流工具 | 构建、运行、测试、依赖管理                               | 安装/切换整套工具链         |
| `rustup` | 工具链管理器  | 安装、升级、切换 `stable/beta/nightly`，补组件和 targets | 直接替代 `cargo` 做项目管理 |


一句话记忆：

- `**rustc` 负责编译**
- `**cargo` 负责项目工作流**
- `**rustup` 负责工具链生命周期**

### 作用

为什么官方默认推荐 `rustup`，而不是只给你一个编译器安装包？

因为真正麻烦的不是“第一次装上”，而是后面的这些事情：

- 升级版本
- 切换 `stable` / `nightly`
- 给单个项目固定工具链
- 安装 `clippy`、`rustfmt`
- 增加 `wasm32-unknown-unknown` 这类 target

如果这些动作都靠手工维护，环境很快就会乱。

### 原理

把 Rust 环境看成三层会清楚很多：

```text
rustup
  -> 管理工具链和组件
  -> 提供 cargo / rustc 等命令

cargo
  -> 组织项目
  -> 调用 rustc

rustc
  -> 把源码编译成产物
```

这就是为什么：

- 你平时最常敲的是 `cargo`
- 你真正跑编译器时底层用的是 `rustc`
- 你想升级或切版本时要用 `rustup`

## 先别混淆：安装方式 vs 开发环境形态

很多资料把这两类东西写在一起，但它们其实不是一个问题。


| 你在决定什么            | 常见选项                        | 结论            |
| ----------------- | --------------------------- | ------------- |
| **如何获取 Rust**     | `rustup`、系统包管理器、官方离线包、源码构建  | 默认首选 `rustup` |
| **把 Rust 装在哪里开发** | 本机原生、WSL、Dev Container、远程机器 | 这是环境形态，不是安装方式 |


组合示例：

- 本机原生 + `rustup`
- WSL + `rustup`
- Dev Container + 预装工具链
- 企业镜像 + 离线安装包

所以不要把“我在容器里开发”和“我用什么安装 Rust”说成同一件事。

## 怎么选安装方案

### 推荐表


| 场景                 | 推荐方案                 | 原因             | 不建议           |
| ------------------ | -------------------- | -------------- | ------------- |
| 个人学习 / 日常开发        | `rustup + stable`    | 资料最多，升级和补组件最顺手 | 直接源码构建        |
| 团队统一运维             | 系统包管理器或内部镜像 + 明确版本策略 | 方便审计和统一部署      | 每人自己切换混乱的全局配置 |
| 内网 / 离线环境          | 官方离线包或企业镜像           | 便于分发和校验        | 临时找不明来源镜像     |
| 研究编译器 / 参与 Rust 本身 | 源码构建                 | 控制力最强          | 把它当普通业务开发默认方案 |


### 默认推荐为什么仍然是 `rustup`

`rustup` 的优势不在“安装快”，而在“后续不麻烦”：

- 工具链切换成本低
- 组件管理统一
- 与官方发布节奏一致
- 出问题时，排查路径也更统一

如果机器能联网，又没有企业合规限制，直接用它最省事。

## 最小用法

### 安装

macOS / Linux 常见安装方式：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
```

补常用组件：

```bash
rustup component add rustfmt clippy rust-analyzer
```

### 验收

最小验收不要低于这一组：

```bash
rustc --version
cargo --version
rustup show
rustup component list --installed
```

如果你已经在一个 Cargo 项目里，最好再补一句：

```bash
cargo run
```

原因很简单：

- `rustc --version` 只能证明命令存在
- `cargo run` 才能证明这套环境真的能构建和运行项目

## 平台提示

### macOS

- 常见前置：`xcode-select --install`
- 重点检查：`~/.cargo/bin` 是否在 `PATH`
- 某些 crate 仍可能依赖系统库或开发工具链

### Linux

- 常见前置：`build-essential`、`pkg-config`、`curl`
- 重点检查：发行版仓库里的 Rust 版本是否过旧
- 编译依赖系统库的 crate 时，往往还要补 `-dev` 包

### Windows

- 默认优先：`*-pc-windows-msvc`
- 重点检查：是否装好了 Visual Studio C++ Build Tools
- 只有明确依赖 GNU / MinGW 生态时再考虑 `*-pc-windows-gnu`

## 注意点

### 注意点 1：版本号不是验收完成

❌ 错误理解：

- `rustc --version` 有输出，说明环境全好了

✅ 正确理解：

- 这只说明编译器命令能执行
- 不代表组件齐全
- 不代表项目能跑
- 不代表 IDE 分析能力就正常

当前机器就是例子：`clippy` 和 `rustfmt` 已经在 `rustup component list --installed` 里，但没有直接证明 `rust-analyzer` 也是这样安装的。

### 注意点 2：`target add` 不等于交叉编译就成功

❌ 错误理解：

```bash
rustup target add wasm32-unknown-unknown
```

“加完 target 就能编任何目标平台了。”

✅ 正确理解：

- `target` 解决的是目标平台标准库和编译声明
- 链接器、SDK、系统库可能仍然缺失
- 真正报错时，根因常常在系统层，不在 Rust 语法层

### 注意点 3：系统包管理器和 `rustup` 没差

✅ 正确理解：

- 系统包管理器更像“运维视角”
- `rustup` 更像“开发者视角”

如果你需要统一运维，前者合理；如果你要长期写 Rust，后者明显顺手。

## 重要命令速查

```bash
rustup default stable
rustup toolchain install nightly
rustup override set nightly
rustup override unset
rustup component add rustfmt clippy rust-analyzer
rustup target add wasm32-unknown-unknown
rustup update
rustup show
rustup toolchain list
rustup component list --installed
```

## 我的理解

- 安装 Rust 本质上是在选一套**可维护的工具链策略**
- 默认场景下，`rustup + stable` 的收益远大于它引入的概念成本
- 真正的验收必须落到 **“能构建、能运行、能补组件”**
- 环境问题的排查顺序应该是：
  1. 命令是否存在
  2. 工具链是否正确
  3. 组件是否齐全
  4. 项目是否能构建
  5. 系统依赖 / 链接器是否缺失

## 下一步

下一篇不再讨论“怎么装”，而是直接看一个最小 Rust 项目是怎么被编译并运行起来的。

- 继续阅读：[2. 运行并编译你的第一个 Rust 程序](./2-运行并编译你的第一个Rust程序.md)
- 回到目录：[第 1 章：环境与第一个程序](./README.md)

