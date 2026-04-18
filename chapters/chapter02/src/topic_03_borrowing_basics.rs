//! 借用基础：`&T` 与 `&mut T` 的“组合规则”和“使用区间”。
//!
//! 这一节只讨论借用规则本身，不把函数签名拉进来。核心要掌握三件事：
//!
//! 1) **多个不可变借用可以并存**：因为大家都只是读。
//! 2) **可变借用必须独占**：同一时刻只能有一个 `&mut`，而且不能和 `&` 同时存在。
//! 3) **规则看的是“使用区间”，不是“物理先后顺序”**：
//!    - 只要不可变借用最后一次使用之后，才出现可变借用，就合法
//!    - 这个机制叫 NLL（Non-Lexical Lifetimes，非词法生命周期）
//!
//! 另外本节还会演示：
//! - 切片借用（`&vec[..]`）与索引借用（`&vec[0]`）
//! - 再借用（reborrow）：从 `&mut T` 借一个 `&T` 出来临时只读
//!
//! 和下一节的关系：下一节会把这些规则放进函数签名里再看一遍。

/// 1) 多个 `&T` 可以并存：读者不会互相破坏状态。
fn demonstrate_multiple_shared_borrows() {
    println!("-- (1) 多个 &T 可以同时存在 --");

    let values = vec![4, 5, 6];

    let r1 = &values;
    let r2 = &values;
    let r3 = &values;

    // 三个只读引用同时活跃，都能读取，Rust 完全允许。
    println!("r1 = {r1:?}, r2 = {r2:?}, r3 = {r3:?}");
}

/// 2) `&mut T` 必须独占：同一时刻，不能再有第二个活跃借用（不论可变还是不可变）。
///
/// 这里我们通过两个**紧挨着的小作用域**来证明：
/// - 在同一个作用域里，可变借用 `ref3` 是唯一的活跃借用
/// - 离开作用域后，原变量才能继续被其他借用访问
fn demonstrate_exclusive_mutable_borrow() {
    println!("-- (2) &mut T 必须独占 --");

    let mut values = vec![4, 5, 6];

    {
        let ref_mut = &mut values;
        ref_mut.push(7);
        println!("exclusive mutable borrow pushed 7: {ref_mut:?}");

        // 在 `ref_mut` 仍在使用的期间：
        // - 不能再创建 `&values`
        // - 也不能再创建第二个 `&mut values`
        // 取消下面任意一行都会触发 E0499 / E0502：
        //
        // let alias_ro = &values;
        // let alias_rw = &mut values;
    }

    // 离开上面的小作用域后，可变借用已经结束，这里可以重新创建只读借用。
    let ref_ro = &values;
    println!("immutable borrow after mut borrow ended: {ref_ro:?}");
}

/// 3) NLL：借用的“有效区间”到**最后一次使用**为止，不是到变量定义的大括号结束为止。
///
/// 这个例子没有手写小作用域，但依然合法：
/// 因为 `r1`、`r2` 在 `println!` 这行之后就再也没有被使用，
/// 编译器判定它们的借用“已经结束”，于是允许后面出现 `&mut values`。
///
/// 这是 Rust 2018 之后引入的重要特性；
/// 很多老教程还停留在“借用作用域 = 词法作用域”，这已经过时了。
fn demonstrate_non_lexical_lifetimes() {
    println!("-- (3) NLL: 借用只到最后一次使用为止 --");

    let mut values = vec![10, 20, 30];

    let r1 = &values;
    let r2 = &values;
    println!("two shared borrows last used here: {r1:?}, {r2:?}");
    // r1 / r2 之后再也没被用到，NLL 判定它们的借用已经结束。

    let r_mut = &mut values;
    r_mut.push(40);
    println!("mutable borrow became possible right after: {r_mut:?}");
}

/// 4) 多个 `&mut` 在**不重叠的区间**里依然可以出现。
///
/// 和上一个例子同源：Rust 的规则不是“一个变量只能被 `&mut` 一次”，
/// 而是“同一时刻只能有一个活跃的 `&mut`”。
fn demonstrate_multiple_mutable_in_sequence() {
    println!("-- (4) 多次 &mut，只要区间不重叠 --");

    let mut values = vec![1, 2, 3];

    {
        let first = &mut values;
        first.push(4);
        println!("first mut borrow: {first:?}");
    }

    {
        let second = &mut values;
        second.push(5);
        println!("second mut borrow: {second:?}");
    }

    println!("两次 &mut 完全合法，因为它们不曾同时活跃。");
}

/// 5) 切片借用：`&values[..]` 得到一个指向原数据的“视图”，底层数据仍由 `values` 拥有。
///
/// 这个知识点在第 4 章 `&[T]` 参数设计里会反复用到，在这里先建立直觉。
fn demonstrate_slice_borrow() {
    println!("-- (5) 切片借用：视图而不是复制 --");

    let values = vec![10, 20, 30, 40, 50];

    let whole: &[i32] = &values[..]; // 整个切片
    let head: &[i32] = &values[..2]; // 前两个
    let tail: &[i32] = &values[3..]; // 最后两个

    println!("whole = {whole:?}, head = {head:?}, tail = {tail:?}");

    // 所有切片都只是借用，`values` 仍然是唯一所有者。
    println!("原 Vec 的所有者仍是 values 本身: {values:?}");
}

/// 6) 再借用（reborrow）：从一个 `&mut T` 里“借一条只读线”出来用一下，再还回去。
///
/// 这个写法 99% 的初学者写不出来，但看过之后会发现标准库大量方法都是这样设计的。
/// 核心直觉：**只要再借用在原 `&mut` 被下次使用之前就结束，就合法**。
fn demonstrate_reborrow() {
    println!("-- (6) 再借用：从 &mut 里临时切出一条 &T --");

    let mut values = vec![1, 2, 3];
    let outer = &mut values;

    // 从 `outer` 再借一条只读引用（显式写法是 `&*outer`，这里依赖自动 reborrow 规则）。
    let snapshot: &Vec<i32> = &*outer;
    println!("reborrowed snapshot: {snapshot:?}");
    // `snapshot` 在这一行之后就不再被使用，它的借用结束了。

    // 于是我们可以继续通过 `outer` 修改底层数据。
    outer.push(4);
    println!("after push via outer: {outer:?}");
}

pub fn run() {
    println!("== Borrowing Basics ==");

    demonstrate_multiple_shared_borrows();
    println!();

    demonstrate_exclusive_mutable_borrow();
    println!();

    demonstrate_non_lexical_lifetimes();
    println!();

    demonstrate_multiple_mutable_in_sequence();
    println!();

    demonstrate_slice_borrow();
    println!();

    demonstrate_reborrow();
    println!();

    println!("规则总结：同一时刻 可以有 多个 &T，或者 一个 &mut T，但不能同时混用；判据是“区间是否重叠”。");
    println!();
}
