# 基于课程大纲的学习路线

> 依据来源：Udemy 课程 **Rust Programming Course: From Beginner to Expert 2.0**
> 公开课程页与公开课程摘要页
> 核对日期：2026-04-16

这份文档不是照搬课程目录，而是把仓库顶层章节调整成更接近课程大章的 21 个 chapter，并继续参考本地目录 `---- Part 1 Basic Programming with Rust ----`、`---- Part 2 Intermediate Level Concepts ----` 和 `---- Part 3 Advance Concepts ---` 补齐章内 topic。

## 课程大章与仓库映射

| 课程大章 | 仓库章节 | 说明 |
| --- | --- | --- |
| Quick Startup | `chapter01` | 当前仓库把环境配置和首个程序也并入这一章，避免顶层再单独拆一个 Orientation chapter |
| Ownership and Borrowing | `chapter02` | 所有权、借用、解引用 |
| Custom and Library Provided Useful Types | `chapter03` | struct、enum、Option、Result、HashMap |
| Organizing your Code | `chapter04` | modules、visibility、crate、re-export |
| Testing | `chapter05` | unit test、integration test、benchmark |
| Flexibility and Abstraction with Generics and Traits | `chapter06` | generics、traits、trait bounds、trait objects、associated types |
| Functional Programming Aspects | `chapter07` | closures、function pointers、iterators、IntoIterator、combinators |
| Memory Management Features | `chapter08` | lifetimes、Box、Rc、RefCell |
| Implementing Typical Data Structures | `chapter09` | singly linked list、doubly linked list、reference cycles |
| Useful Patterns for Handling Structs | `chapter10` | struct initialization、builder pattern、structure decomposition |
| Error Handling | `chapter11` | Result、Option、`?`、layered outcomes、`anyhow`、`thiserror` |

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

### `chapter06`

已经创建：

- Generics
- Traits
- Trait Bounds
- Super Traits
- Trait Objects
- Derived and Marker Traits
- Associated Types in Traits
- Choosing Associated vs Generic Type

### `chapter07`

已经创建：

- Closures
- Function Pointers
- Iterators
- IntoIterator
- Iterating Through Collections
- Combinators
- Iterating Through Option

### `chapter08`

已经创建：

- Concrete Lifetimes
- Generic Lifetimes
- Lifetime Elision
- Lifetimes in Structs
- Box Smart Pointer
- Box Pointer Usecases
- Rc Smart Pointer
- RefCell
- RefCell Example

### `chapter09`

已经创建：

- Singly Linked List (Part 1)
- Singly Linked List (Part 2)
- Doubly Linked List (Part 1)
- Doubly Linked List (Part 2)
- Reference Cycles

### `chapter10`

已经创建：

- Initializing Struct Instances
- Builder Pattern
- Simplifying Structures

### `chapter11`

已经创建：

- Rust Error Handling Approach
- Propagating Errors
- Multiple Error Types
- Chaining Question Marks
- Layered Outcomes: `Result<Option<T>, E>`
- Layered Outcomes: `Option<Result<T, E>>`
- anyhow
- thiserror
- Method Chaining Constraints in Error Handling

后续继续扩展时，直接在对应章节内追加主题页和模块；如果继续补课程后续部分，也继续按同样规则新增新的顶层 chapter。

### `chapter12`

已经创建：

- Size in Rust
- References to Unsized type
- Sized and Optionally Sized Trait
- Optionally Sized Trait and Generic Parameters
- Unsized Coercion
- Zero Sized Types: Never / Unit / Unit Structs / Phantom Data

### `chapter13`

已经创建：

- Coercion Basics
- Coercion Sites
- Reference Coercion
- Function Item Coercion
- Trait Objects Coercion
- Unsized Coercion
- Coercion in Generics
- Transitivity in Coercion
- Least Upper Bound Coercion

### `chapter14`

已经创建：

- Thread basics
- Ownership in Threads
- Channels
- Shared State
- Barriers
- Scoped Threads
- Thread Parking
- Async / Tokio / spawn_blocking
- Webscrapping using Threads

### `chapter15`

已经创建：

- Types of References
- Destructing References
- Forcing Variables to use References
- Referencing in Compound Data Types

### `chapter16`

已经创建：

- Variance background and refresher
- Lifetime Bounds / Implied Bounds
- Covariance / Contravariance / Invariance

### `chapter17`

已经创建：

- Orphan Rule
- Multiple Traits
- Associated Types
- Methods with Generics
- Function with No Self Parameter
- Size and Trait Objects
- Partial Object Safety
- Operator Overloading
- Sealed Traits

### `chapter18`

已经创建：

- Downcasting Trait Objects
- Downcasting Example
- Conversion Between Trait Objects
- TypeId Checking

### `chapter19`

已经创建：

- Drop Check
- Partial Move
- Drop Order
- Self Referencing
- Avoiding Drop
- Panic Safety

### `chapter20`

已经创建：

- Cargo Features consumer side
- Cargo Features library side

### `chapter21`

已经创建：

- Macros Basics
- Capturing Types
- Repeating Patterns
- Question Mark Operator in macros context
