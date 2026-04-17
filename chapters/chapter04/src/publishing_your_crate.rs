// 发布 crate 不只是按一次命令这么简单。
// 对教学项目来说，先让读者知道发布前需要检查哪些内容更重要。
pub fn run() {
    println!("== Publishing Your Crate ==");
    println!("发布前通常会先检查 package metadata、README、license 和版本号。");
    println!("常见流程包括 cargo package、cargo publish --dry-run、cargo publish。");
    println!("公开发布前还应确认 API 是否稳定，以及文档是否足够清楚。");
    println!();
}
