# 第 1 章：Quick Startup

这一章对应课程里的启动阶段内容。当前仓库把原本分散的“环境配置、首个程序、变量、函数、控制流、打印”统一收进 `chapter01`，这样顶层章节能和课程大章保持一致。

## 本章目标

- 搭好 Rust 开发环境，并完成最小验收
- 理解 `cargo`、`rustc`、编译产物和运行入口
- 建立变量、类型、函数、控制流、打印这套基础语法心智模型
- 能直接运行 `cargo run -p chapter01`

## 推荐阅读顺序

1. [Rust 环境配置与安装比较](./1-Rust环境配置与安装比较.md)
2. [运行并编译你的第一个 Rust 程序](./2-运行并编译你的第一个Rust程序.md)
3. [变量：不可变、`mut`、遮蔽](./3-变量.md)
4. [Primitive 数据类型：整数、浮点、布尔、字符](./4-Primitive数据类型.md)
5. [Compound 数据类型：元组与数组](./5-Compound数据类型.md)
6. [函数与代码块](./6-函数与代码块.md)
7. [函数参数中的可变性](./7-函数参数中的可变性.md)
8. [条件、控制流与循环](./8-条件、控制流与循环.md)
9. [注释与打印](./9-注释与打印.md)
10. [变量约定与静态量](./10-变量约定与静态量.md)
11. [编译器指令](./11-编译器指令.md)
12. [错误信息与错误码](./12-错误信息与错误码.md)
13. [Rust 中的运算符](./13-Rust中的运算符.md)
14. [结合性与运算符重载](./14-结合性与运算符重载.md)
15. [运算符优先级](./15-运算符优先级.md)

## 本章对应代码

- Cargo package：`chapter01`
- 运行方式：`cargo run -p chapter01`
- 章节入口：[chapters/chapter01/src/main.rs](../../chapters/chapter01/src/main.rs)
- 主题模块：
  - [topic_01_first_program.rs](../../chapters/chapter01/src/topic_01_first_program.rs)
  - [topic_02_variables.rs](../../chapters/chapter01/src/topic_02_variables.rs)
  - [topic_03_primitive_data_types.rs](../../chapters/chapter01/src/topic_03_primitive_data_types.rs)
  - [topic_04_compound_data_types.rs](../../chapters/chapter01/src/topic_04_compound_data_types.rs)
  - [topic_05_functions_and_code_blocks.rs](../../chapters/chapter01/src/topic_05_functions_and_code_blocks.rs)
  - [topic_06_mutability_in_function_parameters.rs](../../chapters/chapter01/src/topic_06_mutability_in_function_parameters.rs)
  - [topic_07_conditionals_control_flow_and_loops.rs](../../chapters/chapter01/src/topic_07_conditionals_control_flow_and_loops.rs)
  - [topic_08_comments_and_printing.rs](../../chapters/chapter01/src/topic_08_comments_and_printing.rs)
  - [topic_09_variable_conventions_and_statics.rs](../../chapters/chapter01/src/topic_09_variable_conventions_and_statics.rs)
  - [topic_10_compiler_directives.rs](../../chapters/chapter01/src/topic_10_compiler_directives.rs)
  - [topic_11_error_messages_and_error_codes.rs](../../chapters/chapter01/src/topic_11_error_messages_and_error_codes.rs)
  - [topic_12_operators_in_rust.rs](../../chapters/chapter01/src/topic_12_operators_in_rust.rs)
  - [topic_13_associativity_and_operator_overloading.rs](../../chapters/chapter01/src/topic_13_associativity_and_operator_overloading.rs)
  - [topic_14_operator_precedence.rs](../../chapters/chapter01/src/topic_14_operator_precedence.rs)
- 练习模块：[chapters/chapter01/src/lab.rs](../../chapters/chapter01/src/lab.rs)

## 本章产出

- 一套可以继续复用的 Rust 本地工具链
- 一个能在 workspace 中独立运行的 Quick Startup chapter crate
- 一批关于变量、数据类型、函数和控制流的基础笔记
- 一组适合继续扩展的练习入口

## 建议的维护方式

- 后续只要还是“Quick Startup”范围内的内容，就继续补在本章
- 主题页仍然保持“一篇只讲一个核心点”
- 若课程进入 Ownership 之后，不再把新内容继续塞回本章
