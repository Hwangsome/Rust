//! 第 7 章练习：让函数式风格（闭包 + 迭代器 + 组合子）成为你的默认工具。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：三种闭包 trait 的界限");
    println!("  - 写一个 Fn 版本：不可变捕获外部阈值");
    println!("  - 写一个 FnMut 版本：递增捕获的计数器");
    println!("  - 写一个 FnOnce 版本：drop 掉捕获的 String，调用两次看 E0382");

    println!();

    println!("▷ 练习 2：fn 指针 vs impl Fn");
    println!("  - 写 `fn apply(x: i32, f: fn(i32) -> i32) -> i32`");
    println!("  - 把一个捕获闭包传进去，观察编译错误");
    println!("  - 把签名改成 `f: impl Fn(i32) -> i32`，让它编译通过");

    println!();

    println!("▷ 练习 3：自定义 Iterator");
    println!("  - 实现 `struct Fib` 和 `impl Iterator for Fib`");
    println!("  - `Fib::new().take(10).collect::<Vec<_>>()` 验证输出");

    println!();

    println!("▷ 练习 4：IntoIterator 三份实现");
    println!("  - 写 `struct MyVec(Vec<i32>)`");
    println!("  - 分别为 `MyVec` / `&MyVec` / `&mut MyVec` 实现 IntoIterator");
    println!("  - 用三种 for 循环写法各遍历一次");

    println!();

    println!("▷ 练习 5：iter / iter_mut / into_iter");
    println!("  - 对同一个 Vec 用三种方式遍历");
    println!("  - 观察哪个之后原 Vec 还能继续使用，哪个不能");

    println!();

    println!("▷ 练习 6：组合子流水线");
    println!("  - 给定 `let words = vec![\"apple\",\"banana\",\"cherry\",\"date\"];`");
    println!("  - 目标：挑出长度 >= 5 的，大写化，得到 Vec<String>");
    println!("  - 用 filter + map + collect 一行完成");

    println!();

    println!("▷ 练习 7：Option 融进迭代链");
    println!("  - 给一个 Vec<&str> 和一个 Option<&str>，用 `.chain()` 拼成一个迭代器");
    println!("  - 用 `filter_map(|s| s.parse::<i32>().ok())` 把字符串数组里的合法数字挑出来");

    println!();

    println!("▷ 练习 8：性能直觉");
    println!("  - 手写 for + if + push 做和 `filter + map + collect` 相同的事");
    println!("  - 用 `cargo run --release` 粗测两者耗时，确认迭代器没有额外开销");

    println!();

    println!("完成标准：");
    println!("  - 闭包三大 trait（Fn/FnMut/FnOnce）的差异能独立画出来");
    println!("  - 对任意 API 能判断：这里该用闭包还是 fn 指针？");
    println!("  - 能用组合子重写掉 80% 以上的手写 for 循环");

    println!();
}
