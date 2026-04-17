// 第 4 章进入项目结构层面。
// 这里的重点不再是“一个语法点怎么写”，而是“代码为什么要这样组织”。
mod code_organization;
mod lab;
mod modules;
mod privacy_in_modules;
mod publishing_your_crate;
mod using_external_dependencies;
mod visibility;

fn main() {
    println!("Chapter 04: Organizing your Code");
    println!();

    // 先讲 package / crate / module 三层关系，再讲可见性和对外暴露接口。
    code_organization::run();
    modules::run();
    visibility::run();
    privacy_in_modules::run();
    using_external_dependencies::run();
    publishing_your_crate::run();
    lab::run();
}
