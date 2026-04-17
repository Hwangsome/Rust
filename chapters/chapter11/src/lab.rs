// 这一章的练习重点是：先判断你要的是“简单传递错误”，还是“保留错误的精确形状”。
// 选错工具不会立刻编译不过，但会让调用链越来越难维护。
pub fn run() {
    println!("== Lab ==");
    println!("1. 写一个返回 Result 的函数，并用 match 处理 Ok / Err");
    println!("2. 把一个冗长的 match 改写成 `?`");
    println!("3. 设计一个自定义错误 enum，把两个不同错误类型统一起来");
    println!("4. 给一个查询函数设计 Result<Option<T>, E>");
    println!("5. 给一个可选解析流程设计 Option<Result<T, E>>");
    println!("6. 分别用 anyhow 和 thiserror 写一个最小示例，比较边界和用途");
    println!();
}
