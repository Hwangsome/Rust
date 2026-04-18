//! 第 6 章练习：让泛型、trait、trait bound、关联类型**联动**起来。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：泛型 struct Pair<T>");
    println!("  - 写 `struct Pair<T> {{ first: T, second: T }}`");
    println!("  - 写 `impl<T: PartialOrd> Pair<T> {{ fn max(&self) -> &T }}`");
    println!("  - 用 `Pair::new(3, 7)` 和 `Pair::new(\"a\", \"c\")` 分别测试");
    println!();

    println!("▷ 练习 2：两种写法下的 trait 参数");
    println!("  - 写 `fn print_shape(s: &impl Shape)`");
    println!("  - 改写成 `fn print_shape<T: Shape>(s: &T)`");
    println!("  - 再改写成 `fn print_shape<T>(s: &T) where T: Shape`");
    println!("  - 对比三种写法在什么场景下最自然");
    println!();

    println!("▷ 练习 3：默认方法 vs 覆盖");
    println!("  - 给 trait 加一个 `describe(&self) -> String` 的默认实现");
    println!("  - 让一个 impl 保留默认，另一个 impl 覆盖，观察差异");
    println!();

    println!("▷ 练习 4：Vec<Box<dyn Trait>>");
    println!("  - 定义 trait Animal 含 `fn speak(&self) -> String`");
    println!("  - Dog / Cat / Cow 各实现一份");
    println!("  - 把它们装进 `Vec<Box<dyn Animal>>` 遍历调用 speak");
    println!();

    println!("▷ 练习 5：object-safe vs not");
    println!("  - 给 trait 添加 `fn clone_self(&self) -> Self`");
    println!("  - 尝试把它做成 `dyn Trait`，观察编译错误 E0038");
    println!("  - 把 `-> Self` 改成 `-> Box<dyn Trait>`，让它变回 object-safe");
    println!();

    println!("▷ 练习 6：super trait");
    println!("  - 定义 `trait Drawable: Shape + std::fmt::Debug {{}}`");
    println!("  - 给一个 `impl Drawable for Rectangle {{}}`");
    println!("  - 写一个 `fn render<T: Drawable>(t: &T)` 同时用到 Shape 与 Debug");
    println!();

    println!("▷ 练习 7：关联类型");
    println!("  - 定义 `trait Container {{ type Item; fn peek(&self) -> Option<&Self::Item>; }}`");
    println!("  - 给 `Vec<T>` 和 `VecDeque<T>` 手写这个 trait 的实现");
    println!("  - 写一个泛型函数 `fn first_item<C: Container>(c: &C) -> Option<&C::Item>`");
    println!();

    println!("▷ 练习 8：关联类型 vs 泛型参数的取舍");
    println!("  - 同一个主题用两种方式各写一遍（比如 Container 或 Counter）");
    println!("  - 看哪种在调用端更少噪声");
    println!();

    println!("▷ 练习 9：derive 多个 trait");
    println!("  - 为一个 struct derive Debug + Clone + PartialEq + Default + Hash");
    println!("  - 放进 HashMap<MyStruct, i32> 做 key");
    println!();

    println!("完成标准：");
    println!("  - 能流畅在 3 种等价写法（<T: Trait> / where / impl Trait）间切换");
    println!("  - 能说出 dyn Trait 的运行期代价和 object-safe 的判据");
    println!("  - 面对一个 API 能立即判断：这里该用关联类型还是泛型参数？");
    println!();
}
