//! 第 11 章练习：选对错误处理工具，而不是简单"能跑就行"。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：最小 Result + match");
    println!("  - 写 `fn safe_divide(a: i32, b: i32) -> Result<i32, String>`");
    println!("  - 调用方用 match 同时处理 Ok / Err");

    println!();

    println!("▷ 练习 2：? 操作符");
    println!("  - 把练习 1 改成用 `?` 传播错误");
    println!("  - 必须保证外层函数也返回 Result");

    println!();

    println!("▷ 练习 3：合并多种错误类型");
    println!("  - 写 `fn read_and_parse() -> Result<i32, AppError>`");
    println!("  - AppError 同时能容纳 io::Error 和 ParseIntError");
    println!("  - 用手动 map_err 实现一遍；再用 #[from] + thiserror 实现一遍");

    println!();

    println!("▷ 练习 4：Result<Option<T>, E>");
    println!("  - 写 `fn find_user(id: u32) -> Result<Option<User>, DbError>`");
    println!("  - 调用方需要同时区分：查到 / 没查到 / 查询失败");

    println!();

    println!("▷ 练习 5：Option<Result<T, E>>");
    println!("  - 写 `fn parse_optional(input: Option<&str>) -> Option<Result<i32, ParseIntError>>`");
    println!("  - 用 `.transpose()` 把它转成 Result<Option<i32>, E>——观察两种形状在接口上的差异");

    println!();

    println!("▷ 练习 6：anyhow + context");
    println!("  - 写 `fn pipeline() -> anyhow::Result<i32>`");
    println!("  - 每一步用 `.with_context(|| format!(\"step X\"))?`");
    println!("  - 在 main 里用 `{{:#}}` 打印错误链");

    println!();

    println!("▷ 练习 7：thiserror 精确错误");
    println!("  - 定义 `enum AppError {{ Io(...), Parse(...), BadInput(String) }}`");
    println!("  - 用 `#[error(\"...\")]` 派生 Display");
    println!("  - 调用方 match 分支按 variant 做不同的恢复");

    println!();

    println!("▷ 练习 8：方法链 + Result");
    println!("  - 写一个 builder，某步返回 `Result<Self, String>`");
    println!("  - 用 `?` 让链整体可读");

    println!();

    println!("完成标准：");
    println!("  - 能一眼判断：这里用 `?`、`match`、`unwrap_or`、`ok()?` 哪个最简洁");
    println!("  - 能在 library crate 和 application crate 里分别选 thiserror vs anyhow");
    println!("  - 能说清 Result<Option<T>> 和 Option<Result<T>> 的语义差异");

    println!();
}
