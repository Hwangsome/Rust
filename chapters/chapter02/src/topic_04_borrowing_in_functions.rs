//! 函数中的借用：用签名表达“这个函数对数据的权限”。
//!
//! 上一节的规则放进函数签名后会有几个特别实用的惯用法：
//!
//! - `&[T]` 比 `&Vec<T>` 更通用：能同时接受 `Vec`、数组、`Box<[T]>` 等
//! - `&str` 比 `&String` 更通用：得益于 deref 强转，`&String` 会自动变成 `&str`
//! - 只读计算走 `&T`：`len`、`sum`、`find` 这类都属于典型“读者”
//! - 需要修改状态走 `&mut T`：调用方仍然拥有数据，函数只是“临时改一下”
//! - 返回引用需要生命周期知识，这里先不展开；本节只传入借用，不返回借用
//!
//! 建议读的时候问自己一个问题：
//! “如果我把某个参数从 `T` 改成 `&T`，调用方写起来会更自由，还是更受限？”
//! 在初学阶段，90% 的情况答案都是“更自由”，这也是为什么现实里的 Rust API 大量使用借用。

// ---------- 只读接口：能读就够了，别要 &mut ----------

/// 接收切片 `&[i32]`，这样数组和 `Vec` 都能直接传进来。
fn read_and_print(values: &[i32]) {
    println!("readonly values = {values:?}");
}

/// 求和：典型的只读计算，不需要修改输入。
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    // 这里 `values.iter()` 返回 `&i32` 的迭代器，`*v` 把引用取成值后累加。
    for v in values.iter() {
        total += *v;
    }
    total
}

/// 查找：返回 `Option<usize>`，让调用方自己决定“没找到”怎么办。
fn index_of(values: &[i32], target: i32) -> Option<usize> {
    for (i, v) in values.iter().enumerate() {
        if *v == target {
            return Some(i);
        }
    }
    None
}

// ---------- 可变接口：只有真要改的时候才用 &mut ----------

/// 往 `Vec` 末尾追加一个元素；调用方仍然拥有这个 `Vec`。
///
/// 注意签名是 `&mut Vec<i32>` 而不是 `&mut [i32]`：
/// 因为我们要 `push`，需要修改底层容量，`&mut [T]` 做不到（切片长度固定）。
fn append(values: &mut Vec<i32>, item: i32) {
    values.push(item);
}

/// 对切片里的每个元素原地翻倍。
///
/// 这里用 `&mut [i32]` 就够了，因为“翻倍”不改变长度，只改元素值。
fn double_each(values: &mut [i32]) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

// ---------- 字符串家族：&str 是“最窄的入口”，应该优先用 ----------

/// 打印字符串长度。
///
/// 签名用 `&str` 而不是 `&String`，好处：
/// - 字符串字面量 `"hello"` 可以直接传进来
/// - `&String` 会自动 deref 成 `&str`
/// - 将来换成 `&'a str`、`Cow<str>`、`Box<str>` 的字面量视图也不用改 API
fn print_len(s: &str) {
    println!("text = {s:?}, len = {}", s.len());
}

/// 把一段文本追加到 `String` 里。
///
/// 这里两个参数分别对应“目标缓冲区”（`&mut String`）和“要追加的文本”（`&str`）——
/// 这个组合会在后面的章节反复看到。
fn append_text(target: &mut String, extra: &str) {
    target.push_str(extra);
}

pub fn run() {
    println!("== Borrowing In Functions ==");

    // ---- 1) 只读接口：&[T] 天然通用 ----
    let numbers = vec![1, 2, 3, 4, 5];
    read_and_print(&numbers);
    println!("sum = {}", sum(&numbers));

    match index_of(&numbers, 3) {
        Some(idx) => println!("found 3 at index {idx}"),
        None => println!("3 not found"),
    }

    // 同一个函数也能直接接数组——这是 &[T] 的红利。
    let array = [10, 20, 30];
    read_and_print(&array);
    println!("array sum = {}", sum(&array));

    // 注意：这几次调用里，`numbers` 和 `array` 始终是各自作用域的所有者。
    println!("after readonly calls, numbers 仍然可用: {numbers:?}");

    // ---- 2) 可变接口：&mut T / &mut [T] ----
    let mut scores = vec![85, 90, 95];
    append(&mut scores, 100);
    println!("after append: scores = {scores:?}");

    double_each(&mut scores);
    println!("after double_each: scores = {scores:?}");

    // ---- 3) 字符串：&str 是最通用的只读入口 ----
    let owned = String::from("hello");
    print_len(&owned); // &String 自动 deref 成 &str
    print_len("literal"); // &'static str 直接满足 &str

    let mut buf = String::from("Greeting: ");
    append_text(&mut buf, &owned); // &String -> &str
    append_text(&mut buf, ", world!"); // 字面量 -> &str
    println!("buf = {buf}");

    // ---- 设计总结 ----
    println!("接口设计原则：能只读就只读；能接切片就别绑定 Vec；能接 &str 就别绑定 &String。");
    println!();
}
