// Chapter 01 的唯一入口文件。
// 这个文件负责两件事：
// 1. 声明本章的主题模块。
// 2. 按固定学习顺序依次调用每个主题的 `run()`。
//
// 这里特意让主题文件使用 `topic_XX_` 前缀，
// 这样读者在 IDE 和文件管理器里也能直接看到阅读顺序。
mod lab;
mod topic_01_first_program;
mod topic_02_variables;
mod topic_03_primitive_data_types;
mod topic_04_compound_data_types;
mod topic_05_functions_and_code_blocks;
mod topic_06_mutability_in_function_parameters;
mod topic_07_conditionals_control_flow_and_loops;
mod topic_08_comments_and_printing;
mod topic_09_variable_conventions_and_statics;
mod topic_10_compiler_directives;
mod topic_11_error_messages_and_error_codes;
mod topic_12_operators_in_rust;
mod topic_13_associativity_and_operator_overloading;
mod topic_14_operator_precedence;

fn main() {
    println!("Chapter 01: Quick Startup");
    println!("本章顺序以 main.rs 为准，而不是以文件名排序为准。");
    println!("文件名里的 topic_XX_ 只是为了让目录阅读顺序更直观。");
    println!();

    // 第一阶段：先建立“程序可以跑起来”的直觉。
    println!("[01/15] First Program");
    topic_01_first_program::run();

    // 第二阶段：进入最基础的数据与语法。
    println!("[02/15] Variables");
    topic_02_variables::run();
    println!("[03/15] Primitive Data Types");
    topic_03_primitive_data_types::run();
    println!("[04/15] Compound Data Types");
    topic_04_compound_data_types::run();

    // 第三阶段：开始理解函数、参数和控制流。
    println!("[05/15] Functions And Code Blocks");
    topic_05_functions_and_code_blocks::run();
    println!("[06/15] Mutability In Function Parameters");
    topic_06_mutability_in_function_parameters::run();
    println!("[07/15] Conditionals, Control Flow And Loops");
    topic_07_conditionals_control_flow_and_loops::run();

    // 第四阶段：补充日常编码时会马上碰到的基础写法。
    println!("[08/15] Comments And Printing");
    topic_08_comments_and_printing::run();
    println!("[09/15] Variable Conventions And Statics");
    topic_09_variable_conventions_and_statics::run();
    println!("[10/15] Compiler Directives");
    topic_10_compiler_directives::run();
    println!("[11/15] Error Messages And Error Codes");
    topic_11_error_messages_and_error_codes::run();

    // 第五阶段：最后再看运算符相关主题。
    println!("[12/15] Operators In Rust");
    topic_12_operators_in_rust::run();
    println!("[13/15] Associativity And Operator Overloading");
    topic_13_associativity_and_operator_overloading::run();
    println!("[14/15] Operator Precedence");
    topic_14_operator_precedence::run();

    // 练习统一放在最后，方便读者先看演示，再自己动手。
    println!("[15/15] Lab");
    lab::run();
}
