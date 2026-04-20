# 基于课程大纲的学习路线

> 依据来源：Udemy 课程 **Rust Programming Course: From Beginner to Expert 2.0**
> 公开课程页与公开课程摘要页
> 核对日期：2026-04-16

这份文档不是照搬课程目录，而是把仓库顶层章节调整成更接近课程大章的 22 个 chapter，并继续参考本地目录 `---- Part 1 Basic Programming with Rust ----`、`---- Part 2 Intermediate Level Concepts ----` 和 `---- Part 3 Advance Concepts ---` 补齐章内 topic。

## 课程大章与仓库映射

| 课程大章 | 仓库章节 | 说明 |
| --- | --- | --- |
| Quick Startup | `chapter01` | 当前仓库把环境配置和首个程序也并入这一章，避免顶层再单独拆一个 Orientation chapter |
| Ownership and Borrowing | `chapter02` | 所有权、借用、解引用 |
| Custom and Library Provided Useful Types | `chapter03` | struct、enum、Option、Result、HashMap |
| Organizing your Code | `chapter04` | modules、visibility、crate、re-export |
| Testing | `chapter05` | unit test、integration test、benchmark |
| Generics | `chapter06` | generics、monomorphization、泛型语法（trait 细节见 `chapter07`） |
| Traits | `chapter07` | trait、trait bounds、super traits、trait objects、associated types |
| Functional Programming Aspects | `chapter08` | closures、function pointers、iterators、IntoIterator、combinators |
| Memory Management Features | `chapter09` | lifetimes、Box、Rc、RefCell |
| Implementing Typical Data Structures | `chapter10` | singly linked list、doubly linked list、reference cycles |
| Useful Patterns for Handling Structs | `chapter11` | struct initialization、builder pattern、structure decomposition |
| Error Handling | `chapter12` | Result、Option、`?`、layered outcomes、`anyhow`、`thiserror` |
| Understanding Size in Rust | `chapter13` | `Sized` / `?Sized`、ZST、PhantomData |
| Coercion in Rust | `chapter14` | deref coercion、function item、trait object、LUB |
| Concurrency | `chapter15` | threads、channels、shared state、async / Tokio |
| Beyond Basic References | `chapter16` | reference kinds、destructuring、compound types |
| Variance and Its Types | `chapter17` | subtyping、covariance、contravariance、invariance |
| Trait Objects Limitations | `chapter18` | orphan rule、object safety、sealed traits |
| Downcasting | `chapter19` | `Any`、`TypeId`、trait object downcast |
| Dropcheck | `chapter20` | drop check、drop order、panic safety |
| Structing Projects | `chapter21` | Cargo features（consumer / library 视角） |
| Macros | `chapter22` | declarative macros、fragments |

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

### `chapter07`

已经创建：

- Traits
- Trait Bounds
- Super Traits
- Trait Objects
- Derived and Marker Traits
- Associated Types in Traits
- Choosing Associated vs Generic Type

### `chapter08`

已经创建：

- Closures
- Function Pointers
- Iterators
- IntoIterator
- Iterating Through Collections
- Combinators
- Iterating Through Option

### `chapter09`

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

### `chapter10`

已经创建：

- Singly Linked List (Part 1)
- Singly Linked List (Part 2)
- Doubly Linked List (Part 1)
- Doubly Linked List (Part 2)
- Reference Cycles

### `chapter11`

已经创建：

- Initializing Struct Instances
- Builder Pattern
- Simplifying Structures

### `chapter12`

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

### `chapter13`

已经创建：

- Size in Rust
- References to Unsized type
- Sized and Optionally Sized Trait
- Optionally Sized Trait and Generic Parameters
- Unsized Coercion
- Zero Sized Types: Never / Unit / Unit Structs / Phantom Data

### `chapter14`

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

### `chapter15`

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

### `chapter16`

已经创建：

- Types of References
- Destructing References
- Forcing Variables to use References
- Referencing in Compound Data Types

### `chapter17`

已经创建：

- Variance background and refresher
- Lifetime Bounds / Implied Bounds
- Covariance / Contravariance / Invariance

### `chapter18`

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

### `chapter19`

已经创建：

- Downcasting Trait Objects
- Downcasting Example
- Conversion Between Trait Objects
- TypeId Checking

### `chapter20`

已经创建：

- Drop Check
- Partial Move
- Drop Order
- Self Referencing
- Avoiding Drop
- Panic Safety

### `chapter21`

已经创建：

- Cargo Features consumer side
- Cargo Features library side

### `chapter22`

已经创建：

- Macros Basics
- Capturing Types
- Repeating Patterns
- Question Mark Operator in macros context
