//! 函数调用形成的所有权边界。
//!
//! 上一节把所有权放在变量赋值层面看，这一节把同一套规则挪到函数参数与返回值上。
//!
//! 关键观点（务必先在脑子里建立）：
//! - **参数按值接收** ⇒ 可能发生 move（对非 Copy 类型），调用方失去所有权
//! - **参数按 `&T` 接收** ⇒ 借用，调用方仍持有所有权
//! - **参数按 `&mut T` 接收** ⇒ 独占借用，函数可以改，调用结束后调用方继续持有
//! - **返回值** ⇒ 把所有权交回调用方
//! - `Copy` 类型在函数边界上没有 move 问题（按位复制）
//!
//! 建议阅读顺序：
//! 1) 先看 `takes_ownership`（函数吃掉值）
//! 2) 再看 `gives_ownership`（工厂函数）
//! 3) 再看 `takes_and_gives_ownership`（吃进去再吐回来）
//! 4) 再看 `returns_pair`（用元组一次归还多个所有权）
//! 5) 然后对比 `borrows_vec`（用借用替代“吃进去再吐回来”这种笨拙的写法）
//! 6) 最后看 Copy 类型在函数边界上的行为

// ---------- 非 Copy 类型：按值接收就是拿走所有权 ----------

/// 按值接收 `Vec<i32>`：函数结束时 `vec` 被 drop，调用方再也不能用原变量。
fn takes_ownership(vec: Vec<i32>) {
    println!("function took ownership of vec: {vec:?}");
    // 注意：函数结束 `}` 时，`vec` 这个局部绑定离开作用域，堆上的 Vec 会被释放。
}

/// 工厂函数：在函数内部创建值，再把所有权交给调用方。
fn gives_ownership() -> Vec<i32> {
    let produced = vec![4, 5, 6];
    // 这里的“返回”其实是把 `produced` 的所有权 move 给调用方。
    produced
}

/// 经典的“take and give back”模式：
/// - 先把所有权交给函数
/// - 函数修改后再通过返回值交还
///
/// 实战里如果你发现自己写出这种签名，多半说明你应该改用 `&mut T`——详见 `borrows_vec_mutably`。
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

/// 用元组一次返回多个所有者。
///
/// 这是早期 Rust 教程常用的写法：既要函数改完数据，又要把一些“副产品”一起带回去。
/// 虽然可行，但通常不是第一选择——借用往往更自然。
fn returns_pair(mut vec: Vec<i32>) -> (Vec<i32>, usize) {
    vec.push(99);
    let len = vec.len();
    (vec, len)
}

// ---------- 借用对照：同样的目的，不拿走所有权 ----------

/// 只读借用：只想读取，不想消费调用方的数据。
///
/// 这里特意用 `&[i32]` 而不是 `&Vec<i32>`——函数只需要“能当切片读”的能力，
/// 于是接口面变得更通用（数组、`Vec`、`Box<[i32]>` 都能传进来）。
fn borrows_vec_readonly(vec: &[i32]) {
    let sum: i32 = vec.iter().sum();
    println!("readonly borrow: vec = {vec:?}, sum = {sum}");
}

/// 可变借用：允许函数修改调用方的数据，但不拿走所有权。
///
/// 对比上面的 `takes_and_gives_ownership`——同样是“改一下再还”，
/// 但这里不用在返回值上折腾，调用方写起来也更干净。
fn borrows_vec_mutably(vec: &mut Vec<i32>) {
    vec.push(42);
}

// ---------- Copy 类型：函数边界上没有 move 问题 ----------

/// 按值接收 `i32`：这是按位复制，调用方原值仍可用。
///
/// 这和 `String`/`Vec` 完全不同——初学者很容易把两类类型混成一套心智模型。
fn takes_i32_by_value(value: i32) -> i32 {
    value * 2
}

pub fn run() {
    println!("== Ownership In Functions ==");

    // ---- 1) 按值接收：函数吃掉值 ----
    let vec_a = vec![1, 2, 3];
    // 这里故意不 clone。调用后 `vec_a` 就不能再用了——这正是 move 的证据。
    takes_ownership(vec_a);
    // println!("{vec_a:?}"); // ← 取消注释会触发 E0382

    // ---- 2) 工厂函数：返回值把所有权交给调用方 ----
    let vec_b = gives_ownership();
    println!("gives_ownership() returned: {vec_b:?}");

    // ---- 3) 吃进去再吐回来：能跑，但不是好设计 ----
    // 注意看我们怎么“被迫”重新绑定：因为 `vec_b` 被 move 进函数了，
    // 想继续使用就只能接住返回值，再起一个新名字。
    let vec_c = takes_and_gives_ownership(vec_b);
    println!("takes_and_gives_ownership => {vec_c:?}");

    // ---- 4) 用元组一次返回多个所有者 ----
    let (vec_d, len_d) = returns_pair(vec_c);
    println!("returns_pair => vec = {vec_d:?}, len = {len_d}");

    // ---- 5) 借用替代“吃进去再吐回来”：更自然的写法 ----
    //
    // 注意我们不再需要反复重绑定。`vec_e` 从头到尾都是调用方的。
    let mut vec_e = vec![7, 8, 9];
    borrows_vec_readonly(&vec_e); // &Vec<i32> 会自动 deref 成 &[i32]
    borrows_vec_mutably(&mut vec_e); // 函数里对 vec_e 追加了一个元素
    println!("after borrows_vec_mutably: vec_e = {vec_e:?}");

    // 同一个函数既能接 `Vec`，也能接数组——这是选用 `&[T]` 的红利。
    let array = [100, 200, 300];
    borrows_vec_readonly(&array);

    // ---- 6) Copy 类型跨函数边界 ----
    //
    // `i32` 按值传参是按位复制，调用方的原绑定仍然可用。
    let n = 21;
    let doubled = takes_i32_by_value(n);
    println!("Copy across fn boundary: n = {n}, doubled = {doubled}");

    // ---- 规则总结 ----
    println!("签名即契约: `T` 可能吃掉所有权; `&T` 只读; `&mut T` 可改但不拿走。");
    println!();
}
