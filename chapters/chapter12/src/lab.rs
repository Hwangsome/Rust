//! 第 10 章练习：把"能初始化"升级为"API 好用 + 结构好维护"。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：带校验的 new → Result");
    println!("  - 写 `struct Email(String)`，`fn new(s: String) -> Result<Self, String>`");
    println!("  - 校验：必须包含 `@` 字符");
    println!("  - 加一个 `fn as_str(&self) -> &str` 作为只读 getter");

    println!();

    println!("▷ 练习 2：Default + 结构更新语法");
    println!("  - 给一个 7 字段的 struct 派生 Default");
    println!("  - 构造实例时只覆盖其中 2 个字段，其余用 `..Default::default()`");

    println!();

    println!("▷ 练习 3：手写 Builder");
    println!("  - 以 `HttpRequest` 为主题，必填字段：url");
    println!("  - 可选字段：method、headers、body、timeout");
    println!("  - 建一个 `HttpRequestBuilder`，每个可选字段一个链式 setter");
    println!("  - `build()` 返回 `Result<HttpRequest, String>`（校验 url 非空）");

    println!();

    println!("▷ 练习 4：结构拆分缓解借用冲突");
    println!("  - 写一个大 struct `Game {{ players: Vec<Player>, world: World }}`");
    println!("  - 给它写 `fn tick(&mut self)`，内部需要同时修改某个 player 和 world");
    println!("  - 观察 E0499；再把 players 和 world 拆成独立可变引用，让编译通过");

    println!();

    println!("▷ 练习 5：类型细分（newtype）");
    println!("  - 写 `struct Meters(f64); struct Feet(f64);`");
    println!("  - 写 `fn walk(distance: Meters)`，尝试传一个 `Feet`——观察 E0308 的保护");

    println!();

    println!("完成标准：");
    println!("  - 看到一个大 struct 能立刻问自己：这几个字段经常一起变吗？");
    println!("  - 知道 Builder 什么时候值得付出它的样板代码");
    println!("  - 知道怎么用 Default + `..x` 语法写出最干净的构造代码");

    println!();
}
