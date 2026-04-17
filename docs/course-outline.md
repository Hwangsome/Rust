# 基于课程大纲的学习路线

> 依据来源：Udemy 课程 **Rust Programming Course: From Beginner to Expert 2.0**
> 公开课程页与公开课程摘要页
> 核对日期：2026-04-16

这份文档不是照搬课程目录，而是把仓库顶层章节调整成更接近课程大章的 5 个 chapter，并继续参考本地目录 `---- Part 1 Basic Programming with Rust ----` 补齐章内 topic。

## 课程大章与仓库映射

| 课程大章 | 仓库章节 | 说明 |
| --- | --- | --- |
| Quick Startup | `chapter01` | 当前仓库把环境配置和首个程序也并入这一章，避免顶层再单独拆一个 Orientation chapter |
| Ownership and Borrowing | `chapter02` | 所有权、借用、解引用 |
| Custom and Library Provided Useful Types | `chapter03` | struct、enum、Option、Result、HashMap |
| Organizing your Code | `chapter04` | modules、visibility、crate、re-export |
| Testing | `chapter05` | unit test、integration test、benchmark |

## 当前仓库状态

### `chapter01`

已经包含：

- 环境配置与安装比较
- 运行并编译第一个程序
- 变量与数据类型
- 函数与参数
- 控制流与打印
- 变量约定、编译器指令、错误码、运算符、结合性与优先级

### `chapter02`

已经创建：

- Ownership Basics
- Ownership in Functions
- Borrowing Basics
- Borrowing in Functions
- Dereferencing

### `chapter03`

已经创建：

- Structs Basics
- Adding Functionality to Structs
- Enum / Option / Result / HashMap
- Pattern Matching Contexts
- Destructured Struct Parameters
- Casting and Assignment of References
- Method Chaining Constraints

### `chapter04`

已经创建：

- Code Organization
- Modules Basic
- Visibility Specifiers
- Privacy in Modules
- Using External Dependencies
- Publishing your Crate

### `chapter05`

已经创建：

- Unit Testing
- Controlling How Tests Are Run
- Integration Tests
- Benchmarking

后续继续扩展时，直接在对应章节内追加主题页和模块，不再新增新的顶层 chapter 粒度。
