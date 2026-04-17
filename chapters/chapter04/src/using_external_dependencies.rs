// 这一节先解释“依赖从哪里来、怎么加”，而不是直接绑定某个具体第三方 crate。
// 这样读者先建立通用流程，再去选实际依赖时更稳。
pub fn run() {
    println!("== Using External Dependencies ==");
    println!("外部依赖通常写在 Cargo.toml 的 [dependencies] 下。");
    println!("常见命令是 cargo add <crate-name>。");
    println!("引入后再用 use 把需要的项带进当前作用域。");
    println!("这一节先对齐课程结构，保持示例可运行，不额外固定第三方 crate 版本。");
    println!();
}
