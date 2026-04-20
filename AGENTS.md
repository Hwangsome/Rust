# AGENTS.md

## 1. 项目定位

这个仓库是一个 **Rust 入门学习项目**，不是通用工程模板。

目标只有两个：

- 用最小、可运行、可解释的代码示例帮助初学者理解 Rust
- 用结构化笔记沉淀学习过程，方便后续复盘、回查和继续扩展

因此，所有代码、注释和文档都必须优先服务于：

- 初学者可读
- 章节边界清晰
- 笔记与代码一一对应
- 能在 IDE 和终端里稳定运行

---

## 2. 仓库结构

当前仓库必须遵守下面这套结构：

```text
Rust/
├── Cargo.toml                  # workspace
├── AGENTS.md
├── docs/
│   ├── chapter01/
│   ├── chapter02/
│   ├── chapter03/
│   ├── chapter04/
│   └── chapter05/
└── chapters/
    ├── chapter01/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       ├── first_program.rs
    │       ├── variables.rs
    │       ├── functions_and_code_blocks.rs
    │       └── ...
    ├── chapter02/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       ├── ownership_basics.rs
    │       ├── ownership_in_functions.rs
    │       ├── borrowing_basics.rs
    │       ├── borrowing_in_functions.rs
    │       ├── dereferencing.rs
    │       └── lab.rs
    ├── chapter03/
        ├── Cargo.toml
        └── src/
            ├── main.rs
            ├── structs_basics.rs
            ├── adding_functionality_to_structs.rs
            ├── enums.rs
            ├── option_type.rs
            ├── result_type.rs
            ├── hashmaps.rs
            ├── pattern_matching_contexts.rs
            ├── destructured_struct_parameters.rs
            ├── casting_and_assignment_of_references.rs
            ├── method_chaining_constraints.rs
            └── lab.rs
    ├── chapter04/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       ├── code_organization.rs
    │       ├── modules.rs
    │       ├── visibility.rs
    │       ├── privacy_in_modules.rs
    │       ├── using_external_dependencies.rs
    │       ├── publishing_your_crate.rs
    │       └── lab.rs
    └── chapter05/
        ├── Cargo.toml
        ├── src/
        │   ├── main.rs
        │   ├── unit_testing.rs
        │   ├── integration_testing.rs
        │   ├── benchmark_basics.rs
        │   └── lab.rs
        └── tests/
            └── smoke.rs
```

强制规则：

- 根目录 `Cargo.toml` 只能承担 workspace 入口职责
- 代码放在 `chapters/chapterNN/`
- 笔记放在 `docs/chapterNN/`
- 不再使用 `src/bin/` 组织教学代码
- 不再把新增教学示例继续堆到根目录 `src/main.rs`

---

## 3. 章节与代码的对应规则

这是本项目的核心约束。

- `docs/chapter01/` 对应 `chapters/chapter01/`
- `docs/chapter02/` 对应 `chapters/chapter02/`
- `docs/chapter03/` 对应 `chapters/chapter03/`
- `docs/chapter04/` 对应 `chapters/chapter04/`
- `docs/chapter05/` 对应 `chapters/chapter05/`
- 后续新增章节也必须保持同样规则

不允许出现下面这些情况：

- 第 2 章的笔记去引用 `chapters/chapter03/` 的代码
- 第 3 章的代码写进 `chapters/chapter02/`
- 同一主题的笔记和代码放在不同章节目录

如果新增一个章节，至少要同步创建：

- `docs/chapterNN/README.md`
- `chapters/chapterNN/Cargo.toml`
- `chapters/chapterNN/src/main.rs`

如果该章包含多个主题，再继续补该章的主题模块文件。

---

## 4. Chapter Crate 设计规范

每个 chapter 都是一个独立的 Cargo package。

### 4.1 `main.rs` 的职责

`chapters/chapterNN/src/main.rs` 负责：

- 作为该章唯一入口
- 声明本章模块
- 统一控制本章主题的演示顺序
- 输出该章标题或章节提示

`main.rs` 不负责：

- 承载整章所有细节实现
- 混杂大量无结构示例
- 直接写成一大坨临时实验代码

推荐模式：

```rust
mod topic_a;
mod topic_b;
mod lab;

fn main() {
    println!("Chapter NN");
    println!();

    topic_a::run();
    topic_b::run();
    lab::run();
}
```

### 4.2 主题模块的职责

每个主题应拆到独立模块文件，例如：

- `variables.rs`
- `primitive_data_types.rs`
- `functions_and_code_blocks.rs`

约束如下：

- 文件名必须是合法 Rust 模块名
- 使用 `snake_case`
- 不要使用数字开头
- 不要使用连字符 `-`

模块对外统一暴露：

```rust
pub fn run() {
    // 本主题的最小演示
}
```

### 4.3 `lab.rs` 的职责

如果该章需要练习说明，统一放在：

- `chapters/chapterNN/src/lab.rs`

`lab.rs` 的目标是：

- 说明这个练习要做什么
- 说明想达到什么效果
- 给出完成标准

默认不要在 `lab.rs` 里直接给完整答案，除非该章本身就是“讲解答案”的章节。

---

## 5. 文档规范

### 5.1 `docs/README.md`

只负责：

- 总导航
- 当前章节目录
- 总体结构说明
- 维护规则

不要把某一章的大段正文堆在总 README 里。

### 5.2 `docs/chapterNN/README.md`

只负责：

- 本章目标
- 推荐阅读顺序
- 本章产出
- 对应代码位置
- 运行方式

不要把具体知识点的大段正文继续堆进章节 README。

### 5.3 单篇主题笔记

每篇主题笔记只讲一个主题。

如果该主题对应代码，文档开头必须固定写出这些信息：

- `Cargo package`
- `Run chapter`
- `Chapter entry`
- `Reference module`
- `Chapter lab`（如果本章有）

推荐格式：

```md
- Cargo package: `chapter01`
- Run chapter: `cargo run -p chapter01`
- Chapter entry: `chapters/chapter01/src/main.rs`
- Reference module: `chapters/chapter01/src/variables.rs`
- Chapter lab: `chapters/chapter01/src/lab.rs`
```

如果该章没有 `lab.rs`，就明确写“本章当前没有独立 lab 模块”。

### 5.4 写作原则

- 先说“这一节解决什么问题”
- 再给最小结论
- 再展开定义、作用、原理、示例、坑点
- 示例尽量可运行
- 输出尽量写出来
- 尽量让笔记在未来单独阅读时也能看懂

---

## 6. Rust 代码规范

### 6.1 初学者优先

- 优先选择最容易理解的写法
- 优先选择最直接的控制流
- 优先解释“为什么这样写”
- 优先给出最小可运行示例

### 6.2 示例代码规则

- 一个示例只突出一个核心知识点
- 如果已经进入下一章知识点，不要提前引入
- 输出要有观察价值，不要写“跑了也看不出差异”的示例
- 变量名要有语义，不要大量使用 `a`、`b`、`x1`

### 6.3 注释规则

注释应该解释：

- 这段代码在演示什么
- 为什么这样写
- 这里体现了 Rust 的什么规则
- 容易和其他语言混淆的点是什么
- 这一小段输出和实验现象应该怎么观察

不要写低信息量注释，例如：

```rust
let x = 1; // 定义一个变量
```

### 6.3.1 教学代码的注释强度

这个仓库是面向 **Rust 初学者** 的，所以注释强度要明显高于普通业务代码。

强制要求：

- 每个教学 `.rs` 文件都必须有一段**文件头注释**
- 文件头注释至少说明：
  - 这个文件要讲什么
  - 读者运行后应该观察什么
  - 这个例子和上一章/下一章的关系是什么
- 每个关键知识点前都应有**步骤级注释**
- 关键输出前后要有注释说明“为什么会打印这个结果”
- 如果一段代码故意避开了更复杂但更真实的写法，也要解释为什么先不引入

对于本项目，下面这种注释密度是推荐的：

- 进入一个新概念前先有 1 到 3 行概念注释
- 容易误解的语句前单独写注释，不要把解释挤在行尾
- `main.rs` 需要解释本章主题为什么按这个顺序运行
- `lab.rs` 需要解释练习的观察点，而不是只列题目

下面这种做法不够：

- 只有标题输出，没有代码解释
- 只在复杂类型上写注释，基础语法完全不解释
- 读者看完代码仍然不知道“为什么这里要这样写”

### 6.3.2 教学注释的写法要求

推荐写法：

- 先解释意图，再给代码
- 用“这里要观察的是……”这种句式提示初学者
- 对比概念时明确说出“这和 `mut` 不同”“这不是 move”“这里只是借用”
- 注释优先解释 Rust 规则，不要只翻译代码字面意思

不推荐写法：

- `let x = 1; // 定义变量`
- `println!("{}", x); // 打印 x`
- 把注释写成逐行中文翻译

### 6.4 入门阶段的限制

除非当前章节明确讲这些主题，否则默认避免：

- 复杂泛型技巧
- 高级宏写法
- 过深的 trait 设计
- 过早引入生命周期细节
- `unsafe`
- 为了炫技而缩短代码

### 6.5 `unwrap` 和 `expect`

- 默认不要滥用 `unwrap()`
- 如果为了聚焦当前主题而暂时使用，必须在文档里解释原因
- `expect()` 的错误信息要具体

---

## 7. Lab 规范

`lab.rs` 默认只保留练习说明，不直接给答案。

推荐包含：

- 练习目标
- 预期效果
- 完成标准
- 观察点

例如：

```rust
pub fn run() {
    println!("1. 定义一个不可变变量并打印");
    println!("2. 改成 mut 后再次运行");
    println!("3. 观察编译器对不可变变量重新赋值的提示");
}
```

`lab.rs` 不应：

- 直接把参考实现完整复制一遍
- 提前引入后续章节概念
- 与本章主题脱节

---

## 8. 新增或修改章节时的同步项

只要变更了某个章节，至少检查下面这些位置是否需要同步：

- `AGENTS.md`
- `docs/README.md`
- `docs/course-outline.md`
- `docs/chapterNN/README.md`
- 对应主题笔记中的代码路径和运行命令
- `chapters/chapterNN/src/main.rs`

如果只是改了模块文件名，也必须同步修改文档链接。

若修改了某章 **主题源码**（例如 `chapters/chapterNN/src/topic_*.rs`）中的可运行演示：新增分段、`run()` 里新输出、新概念（如 turbofish），应**同时**更新：

- 该文件内的模块注释 / 分段注释（与 `run()` 一致）
- `docs/chapterNN/` 下对应主题笔记（路径、表格、结论与代码对齐）

主题笔记开头或源码模块注释可互相引用路径，便于双向维护。

---

## 9. 验证规范

代码或结构变更完成后，默认做下面这些验证：

1. `cargo check --workspace`
2. 运行受影响的章节，例如：
   - `cargo run -p chapter01`
   - `cargo run -p chapter02`
   - `cargo run -p chapter03`
   - `cargo run -p chapter04`
   - `cargo run -p chapter05`
3. 确认文档中的路径和命令仍然准确

如果没有运行验证，要在交付说明里明确写出来。

---

## 10. RustRover 适配原则

这个仓库默认服务于 RustRover 这类 IDE 场景。

因此结构设计遵守：

- 以 Cargo workspace 为顶层
- 每个 chapter 是一个独立 crate
- 每个 chapter 只有一个 `main.rs`
- 主题作为模块挂在对应 chapter 下

这套设计的目标是：

- 在 IDE 里按 chapter 运行
- 模块跳转和自动补全正常工作
- 不再出现“孤立 `.rs` 文件缺少模块声明”的问题

如果后续要学习模块系统，可以在对应章节里继续使用这套结构演示 `mod`、`pub` 和模块边界，而不是回退到悬空文件方案。
