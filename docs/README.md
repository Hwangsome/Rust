# Rust 学习笔记

这份 `docs/` 用来沉淀 Rust 学习过程中的章节笔记、实验记录和结论。当前仓库按课程大章来组织顶层章节，而不是按单个 lecture 拆顶层目录。

代码与笔记按章节一一对应：

- 第 `N` 章笔记放在 `docs/chapterNN/`
- 第 `N` 章代码放在 `chapters/chapterNN/`
- 每章都是一个独立 Cargo package
- 每章通过 `src/main.rs` 统一演示本章主题

## 当前章节

### [第 1 章：Quick Startup](./chapter01/README.md)

1. [Rust 环境配置与安装比较](./chapter01/1-Rust环境配置与安装比较.md)
2. [运行并编译你的第一个 Rust 程序](./chapter01/2-运行并编译你的第一个Rust程序.md)
3. [变量：不可变、`mut`、遮蔽](./chapter01/3-变量.md)
4. [Primitive 数据类型：整数、浮点、布尔、字符](./chapter01/4-Primitive数据类型.md)
5. [Compound 数据类型：元组与数组](./chapter01/5-Compound数据类型.md)
6. [函数与代码块](./chapter01/6-函数与代码块.md)
7. [函数参数中的可变性](./chapter01/7-函数参数中的可变性.md)
8. [条件、控制流与循环](./chapter01/8-条件、控制流与循环.md)
9. [注释与打印](./chapter01/9-注释与打印.md)
10. [变量约定与静态量](./chapter01/10-变量约定与静态量.md)
11. [编译器指令](./chapter01/11-编译器指令.md)
12. [错误信息与错误码](./chapter01/12-错误信息与错误码.md)
13. [Rust 中的运算符](./chapter01/13-Rust中的运算符.md)
14. [结合性与运算符重载](./chapter01/14-结合性与运算符重载.md)
15. [运算符优先级](./chapter01/15-运算符优先级.md)

### [第 2 章：Ownership and Borrowing](./chapter02/README.md)

1. [所有权基础](./chapter02/1-所有权.md)
2. [函数中的所有权](./chapter02/2-函数中的所有权.md)
3. [借用基础](./chapter02/3-借用基础.md)
4. [函数中的借用](./chapter02/4-函数中的借用.md)
5. [解引用](./chapter02/5-解引用.md)

### [第 3 章：Custom and Library Provided](./chapter03/README.md)

1. [Struct 基础](./chapter03/1-Struct基础.md)
2. [为 Struct 添加功能](./chapter03/2-为Struct添加功能.md)
3. [Enum](./chapter03/3-Enum.md)
4. [Option](./chapter03/4-Option.md)
5. [Result](./chapter03/5-Result.md)
6. [HashMap](./chapter03/6-HashMap.md)
7. [模式匹配上下文](./chapter03/7-模式匹配上下文.md)
8. [解构的 Struct 参数](./chapter03/8-解构的Struct参数.md)
9. [引用的转换与赋值](./chapter03/9-引用的转换与赋值.md)
10. [方法链的约束](./chapter03/10-方法链的约束.md)

### [第 4 章：Organizing your Code](./chapter04/README.md)

1. [代码组织](./chapter04/1-代码组织.md)
2. [模块基础](./chapter04/2-模块基础.md)
3. [可见性说明符](./chapter04/3-可见性说明符.md)
4. [模块中的隐私与重导出](./chapter04/4-模块中的隐私与重导出.md)
5. [使用外部依赖](./chapter04/5-使用外部依赖.md)
6. [发布你的 Crate](./chapter04/6-发布你的Crate.md)

### [第 5 章：Testing](./chapter05/README.md)

1. [单元测试](./chapter05/1-单元测试.md)
2. [控制测试如何运行](./chapter05/2-控制测试如何运行.md)
3. [集成测试](./chapter05/3-集成测试.md)
4. [基准测试](./chapter05/4-基准测试.md)

### [第 6 章：Generics](./chapter06/README.md)

1. [泛型](./chapter06/1-泛型.md)

### [第 7 章：Traits](./chapter07/README.md)

1. [Trait](./chapter07/1-Trait.md)
2. [Trait 约束](./chapter07/2-Trait约束.md)
3. [Super Trait](./chapter07/3-SuperTrait.md)
4. [Trait Object](./chapter07/4-TraitObject.md)
5. [Derive 与 Marker Trait](./chapter07/5-Derive与MarkerTrait.md)
6. [Trait 中的关联类型](./chapter07/6-Trait中的关联类型.md)
7. [关联类型与泛型参数的取舍](./chapter07/7-关联类型与泛型参数的取舍.md)

### [第 8 章：Functional Programming Aspects](./chapter08/README.md)

1. [闭包](./chapter08/1-闭包.md)
2. [函数指针](./chapter08/2-函数指针.md)
3. [迭代器](./chapter08/3-迭代器.md)
4. [IntoIterator](./chapter08/4-IntoIterator.md)
5. [遍历集合](./chapter08/5-遍历集合.md)
6. [组合子](./chapter08/6-组合子.md)
7. [遍历 Option](./chapter08/7-遍历Option.md)

### [第 9 章：Memory Management Features](./chapter09/README.md)

1. [具体生命周期](./chapter09/1-具体生命周期.md)
2. [泛型生命周期](./chapter09/2-泛型生命周期.md)
3. [生命周期省略](./chapter09/3-生命周期省略.md)
4. [Struct 中的生命周期](./chapter09/4-Struct中的生命周期.md)
5. [Box 智能指针](./chapter09/5-Box智能指针.md)
6. [Box 的典型用法](./chapter09/6-Box的典型用法.md)
7. [Rc 智能指针](./chapter09/7-Rc智能指针.md)
8. [RefCell](./chapter09/8-RefCell.md)
9. [RefCell 示例](./chapter09/9-RefCell示例.md)

### [第 10 章：Typical Data Structures](./chapter10/README.md)

1. [单向链表（基础结构）](./chapter10/1-单向链表（基础结构）.md)
2. [单向链表（添加与移除）](./chapter10/2-单向链表（添加与移除）.md)
3. [双向链表（前端插入）](./chapter10/3-双向链表（前端插入）.md)
4. [双向链表（移除与遍历）](./chapter10/4-双向链表（移除与遍历）.md)
5. [引用环](./chapter10/5-引用环.md)

### [第 11 章：Useful Patterns for Structs](./chapter11/README.md)

1. [初始化 Struct 实例](./chapter11/1-初始化Struct实例.md)
2. [Builder 模式](./chapter11/2-Builder模式.md)
3. [简化大型 Struct](./chapter11/3-简化大型Struct.md)

### [第 12 章：Error Handling](./chapter12/README.md)

1. [Rust 错误处理思路](./chapter12/1-Rust错误处理思路.md)
2. [传播错误](./chapter12/2-传播错误.md)
3. [统一多种错误类型](./chapter12/3-统一多种错误类型.md)
4. [链式调用中的问号运算符](./chapter12/4-链式调用中的问号运算符.md)
5. [分层结果类型：Result<Option<T>, E>](./chapter12/5-分层结果类型：ResultOption.md)
6. [分层结果类型：Option<Result<T, E>>](./chapter12/6-分层结果类型：OptionResult.md)
7. [anyhow](./chapter12/7-anyhow.md)
8. [thiserror](./chapter12/8-thiserror.md)
9. [错误处理里的方法链约束](./chapter12/9-错误处理里的方法链约束.md)

### [第 13 章：Understanding Size in Rust](./chapter13/README.md)

1. `Size in Rust`
2. `References to Unsized type`
3. `Sized and Optionally Sized Trait`
4. `Optionally Sized Trait and Generic Parameters`
5. `Unsized Coercion`
6. `Zero Sized Types- Never Type`
7. `Zero Sized Types- Unit Type`
8. `Zero Sized Types- Unit Structs`
9. `Zero Sized Types - Phantom Data`

### [第 14 章：Coercion in Rust](./chapter14/README.md)

1. `Coercion Basics (Deref Coercion)`
2. `Coercion Sites`
3. `Reference Coercion`
4. `Function Item Coercion`
5. `Trait Objects Coercion`
6. `Unsized Coercion`
7. `Coercion in Generics`
8. `Transitivity in Coercion`
9. `Least Upper Bound Coercion`

### [第 15 章：Concurrency](./chapter15/README.md)

1. `Thread basics`
2. `Ownership in Threads`
3. `Message passing through channels (Part 1)`
4. `Message passing through channels (Part 2)`
5. `Sharing States (Part 1)`
6. `Sharing States (Part 2)`
7. `Synchronization through Barriers`
8. `Scoped Threads`
9. `Thread Parking`
10. `Async-Await (Basics)`
11. `Tokio Tasks`
12. `Computationally Expensive Functions`
13. `Project- Webscrapping using Threads`

### [第 16 章：Beyond Basic References](./chapter16/README.md)

1. `Types of References`
2. `Destructing References`
3. `Forcing Variables to use References`
4. `Referencing in Compound Data Types`

### [第 17 章：Variance and Its Types](./chapter17/README.md)

1. `Background for Variance (References Refresher (Part 1))`
2. `Background for Variance (References Refresher (Part 2))`
3. `Background for Variance (Lifetime Bounds)`
4. `Background for Variance (Implied bounds)`
5. `Subtyping and Variance (Covariance )`
6. `Contravariance`
7. `Invariance (Part 1)`
8. `Invariance (Part 2)`

### [第 18 章：Trait Objects Limitations](./chapter18/README.md)

1. `Orphan Rule`
2. `Multiple Traits`
3. `Associated Types`
4. `Methods with Generics`
5. `Function with No Self Parameter`
6. `Size and Trait Objects`
7. `Partial Object Safety`
8. `Operator Overloading`
9. `Sealed Traits`

### [第 19 章：Downcasting](./chapter19/README.md)

1. `Downcasting Trait Objects`
2. `Downcasting Example`
3. `Downcasting for Conversion Between Trait Objects`
4. `Checking type without Downcasting using TypeId`

### [第 20 章：Dropcheck](./chapter20/README.md)

1. `Drop Check`
2. `Drop Check and Partial Move`
3. `Drop Order (Part 1)`
4. `Drop Order (Part 2)`
5. `Drop Order and Self referencing`
6. `Avoiding Drop`
7. `Panic safety`

### [第 21 章：Structing Projects](./chapter21/README.md)

1. `consumer_of_lib`
2. `math`

### [第 22 章：Macros](./chapter22/README.md)

1. `Macros Basics`
2. `Capturing Types`
3. `Repeating Patterns`
4. `Question Mark Operator`

## 课程对齐路线

- [基于课程大纲的学习路线](./course-outline.md)

## 当前进度

- 已深化：第 1 章 Quick Startup 15 个主题
- 已深化：第 2 章 Ownership and Borrowing 5 个主题
- 已深化：第 3 章 Custom and Library Provided 10 个主题
- 已深化：第 4 章 Organizing your Code 6 个主题
- 已深化：第 5 章 Testing 4 个主题
- 已新增：第 6 章 Generics 1 个主题
- 已新增：第 7 章 Traits 7 个主题
- 已新增：第 8 章 Functional Programming Aspects 7 个主题
- 已新增：第 9 章 Memory Management Features 9 个主题
- 已新增：第 10 章 Typical Data Structures 5 个主题
- 已新增：第 11 章 Useful Patterns for Structs 3 个主题
- 已新增：第 12 章 Error Handling 9 个主题
- 已新增：第 13 章 Understanding Size in Rust 9 个主题
- 已新增：第 14 章 Coercion in Rust 9 个主题
- 已新增：第 15 章 Concurrency 13 个主题
- 已新增：第 16 章 Beyond Basic References 4 个主题
- 已新增：第 17 章 Variance and Its Types 8 个主题
- 已新增：第 18 章 Trait Objects Limitations 9 个主题
- 已新增：第 19 章 Downcasting 4 个主题
- 已新增：第 20 章 Dropcheck 7 个主题
- 已新增：第 21 章 Structing Projects 2 个主题
- 已新增：第 22 章 Macros 4 个主题
- 后续方向：继续补更多实验输出、对比表、练习答案版和章节间交叉引用
