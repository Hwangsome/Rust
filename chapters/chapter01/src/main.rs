// 第 1 章是 Quick Startup。
// 这里把启动阶段的主题按“从最基础到稍微抽象”的顺序串起来，
// 方便初学者直接运行一次，就看到整个入门路线的最小版本。
mod associativity_and_operator_overloading;
mod comments_and_printing;
mod compiler_directives;
mod compound_data_types;
mod conditionals_control_flow_and_loops;
mod error_messages_and_error_codes;
mod first_program;
mod functions_and_code_blocks;
mod lab;
mod mutability_in_function_parameters;
mod operator_precedence;
mod operators_in_rust;
mod primitive_data_types;
mod variable_conventions_and_statics;
mod variables;

fn main() {
    println!("Chapter 01: Quick Startup");
    println!();

    // 先从“程序真的能跑起来”开始，再逐步进入语法、控制流和工具链细节。
    first_program::run();
    variables::run();
    primitive_data_types::run();
    compound_data_types::run();
    functions_and_code_blocks::run();
    mutability_in_function_parameters::run();
    conditionals_control_flow_and_loops::run();
    comments_and_printing::run();
    variable_conventions_and_statics::run();
    compiler_directives::run();
    error_messages_and_error_codes::run();
    operators_in_rust::run();
    associativity_and_operator_overloading::run();
    operator_precedence::run();

    // 最后统一打印练习建议，让读者知道这一章可以怎么自己动手验证。
    lab::run();
}
