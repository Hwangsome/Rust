//! 第 4 章练习：从"写能跑"升级到"写得像一个项目"。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：画出 package / crate / module 关系图");
    println!("  - 用 ASCII 画一个三层结构：workspace → package → crate → module");
    println!("  - 把本仓库 chapters/chapter04 填进每一层");

    println!();

    println!("▷ 练习 2：在当前 chapter 再加一个内嵌模块");
    println!("  - 在本 chapter 的某个文件里加 `mod utils {{ pub fn now_secs() -> u64 {{ 0 }} }}`");
    println!("  - 从 run() 里调用 utils::now_secs()，确认能运行");
    println!("  - 再把 `pub` 去掉，观察编译错误（E0603）");

    println!();

    println!("▷ 练习 3：提取到独立文件");
    println!("  - 把练习 2 的 mod utils 提取到 utils.rs 文件");
    println!("  - 在原文件用 `mod utils;` 声明，注意两个位置的语法差异");

    println!();

    println!("▷ 练习 4：四种可见性全部演示");
    println!("  - 在一个 mod 里定义：默认私有 / pub / pub(crate) / pub(super) 各一个");
    println!("  - 分别从同模块 / 父模块 / crate 其他地方尝试调用，记录哪些能通、哪些报错");

    println!();

    println!("▷ 练习 5：pub struct 的封装");
    println!("  - 定义 `pub struct Counter {{ value: u64 }}`（字段故意不 pub）");
    println!("  - 提供 `new()` / `increment(&mut self)` / `value(&self) -> u64` 方法");
    println!("  - 尝试从外部直接写 `c.value = 5;`，观察 E0451");

    println!();

    println!("▷ 练习 6：Cargo.toml 依赖写法");
    println!("  - 自己写一段 `[dependencies]`，包含常规、features、dev-dependencies");
    println!("  - 运行 `cargo tree` 观察依赖图");

    println!();

    println!("▷ 练习 7：模拟发布流程");
    println!("  - 把 metadata 填完整（license、description、repository）");
    println!("  - 运行 `cargo package` 和 `cargo publish --dry-run`");
    println!("  - 故意缺失 license，看 publish 时的错误提示");

    println!();

    println!("完成标准：");
    println!("  - 能准确说出 package / crate / module 三者的关系");
    println!("  - 能解释 pub / pub(crate) / pub(super) / 默认私有 在什么场景下用");
    println!("  - 能独立给出一个最小可发布 crate 的 Cargo.toml");
    println!();
}
