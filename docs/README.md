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

## 课程对齐路线

- [基于课程大纲的学习路线](./course-outline.md)

## 当前进度

- 已深化：第 1 章 Quick Startup 15 个主题
- 已深化：第 2 章 Ownership and Borrowing 5 个主题
- 已深化：第 3 章 Custom and Library Provided 10 个主题
- 已深化：第 4 章 Organizing your Code 6 个主题
- 已深化：第 5 章 Testing 4 个主题
- 后续方向：继续补更多实验、错误输出、对比表和章节间交叉引用
