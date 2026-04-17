// 第 4 章进入项目结构层面。
// 这里的重点不再是“一个语法点怎么写”，而是“代码为什么要这样组织”。
mod topic_01_code_organization;
mod lab;
mod topic_02_modules;
mod topic_04_privacy_in_modules;
mod topic_06_publishing_your_crate;
mod topic_05_using_external_dependencies;
mod topic_03_visibility;

fn main() {
    println!("Chapter 04: Organizing your Code");
    println!();

    // 先讲 package / crate / module 三层关系，再讲可见性和对外暴露接口。
    topic_01_code_organization::run();
    topic_02_modules::run();
    topic_03_visibility::run();
    topic_04_privacy_in_modules::run();
    topic_05_using_external_dependencies::run();
    topic_06_publishing_your_crate::run();
    lab::run();
}
