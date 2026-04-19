//! 借用基础：`&T` 与 `&mut T` 的“组合规则”和“使用区间”。
//!
//! 这一节只讨论借用规则本身，不把函数签名拉进来。核心要掌握三件事：
//!
//! 1) **多个不可变借用可以并存**：因为大家都只是读。
//! 2) **可变借用必须独占**：同一时刻只能有一个 `&mut`，而且不能和 `&` 同时存在。
//!
//! **口语锚点**：上面两条合起来，很像「**读读可以同时进行；一旦有写（`&mut`），就不能再和别的读/写并行**」——也就是常说的**多读共享、读写互斥**直觉（类比读写锁里的多读者 / 独占写者，便于记忆，别当成 OS 原语一一对应）。
//!
//! 3) **规则看的是“使用区间”，不是“物理先后顺序”**：
//!    - 只要不可变借用最后一次使用之后，才出现可变借用，就合法
//!    - 这个机制叫 NLL（Non-Lexical Lifetimes，非词法生命周期）
//!
//! 另外本节还会演示：
//! - 切片借用（`&vec[..]`）与索引借用（`&vec[0]`）
//! - 再借用（reborrow）：从 `&mut T` 借一个 `&T` 出来临时只读
//! - 词法块释放 `&mut`：内层 `{}` 结束，独占借用结束，外层所有者继续可用（与 NLL 对照）
//!
//! 和下一节的关系：下一节会把这些规则放进函数签名里再看一遍。
//!
//! 运行顺序上，先用 `(0)` 把符号钉死：`T` / `&T` / `&mut T` 与 `let mut` 的分工，再进入组合规则；末尾 `(7)` 用**词法块**对照 NLL。
//!
//! ## “借用就是借个地址去堆上读？”——只信一半
//!
//! - **一半对**：像 `String` / `Vec` / `Box<T>` 这种“栈上头 + 堆里肉”的类型，`&T` 往往沿着**地址链**最终读到堆（例如 `&Vec` 先指到栈上的 `Vec` 头，再通过头里的指针指到堆缓冲）。
//! - **另一半**：借用在语言里的**核心**是编译期检查的 **“临时访问权 + 能活多久（活跃区间）”**，不是把 `&` 等同于“堆指针”；`&i32` 可以**只在栈上**，`&str` 还可能指向**只读数据段**的字面量。
//! - **更好记**：借用 = **借一条路去用那份 `T`**；这条路最后穿到栈、堆还是静态区，由 **`T` 的布局**决定，别默认“一定有堆”。

/// 0) **对照表**：`T`（拥有） vs `&T`（只读借用） vs `&mut T`（可变借用）
///
/// 这一节专门回答一个常见口误：
///
/// > “`T` 是不可变引用吗？”
///
/// **不是。** 单独写 `T` 通常表示“值的类型/一份拥有”，引用类型要把 `&` 写出来：`&T` / `&mut T`。
///
/// 另外分清两个 `mut`：
///
/// - `let mut x`：`x` 这个**绑定**可变（允许 `x = ...`，也允许借出 `&mut x`）
/// - `&mut x`：借出**可变引用**（通过它改 `x` 里的数据，受独占规则约束）
fn demonstrate_owned_vs_borrow_vs_mut_borrow() {
    println!("-- (0) T vs &T vs &mut T（用代码钉概念） --");

    // -------------------------------------------------------------------------
    // A) `T`：拥有（owned）。`s` 的类型是 `String`，不是引用。
    // -------------------------------------------------------------------------
    let s: String = String::from("hi");
    // `s` 拥有堆上的 UTF-8 缓冲；离开作用域时由 `s` 负责释放（drop）。

    // -------------------------------------------------------------------------
    // B) `&T`：不可变借用。`r` 的类型是 `&String`（读作“对 String 的引用”）。
    // -------------------------------------------------------------------------
    let r: &String = &s;
    //            ^^^
    // `&s`：从 `s` **借**一个只读视角；所有权仍在 `s`，没有 move 走 `String`。
    //
    // 实现上 `r` 里会带“地址信息”，但别把它简化成“只借堆地址”：
    // - `r: &String` 先指向 **栈上的 `String` 头**（ptr/len/cap），再通过头里的指针去读 **堆** 上的 UTF-8。
    // - 规则层面你记住：**`&` = 只读许可**；堆不堆是 `String` 这种类型自己的事。

    // 可以同时存在多个 `&T`（共享只读）：
    let r2: &String = &s;
    println!("A) owned s = {s}, B) two shared borrows r = {r}, r2 = {r2}");

    // -------------------------------------------------------------------------
    // C) `&mut T`：可变借用。需要两件事同时成立：
    //   1) 原绑定写成 `let mut ...`（允许可变借用从它出发）
    //   2) 用 `&mut` 取引用（不是只写 `&`）
    // -------------------------------------------------------------------------
    let mut buf: String = String::from("hello");
    let m: &mut String = &mut buf;
    //                 ^^^^^^^^
    // `m` 的类型是 `&mut String`：通过 `m` 可以改 `buf` 指向的字符串内容。

    m.push_str(" world");
    println!("C) after push via &mut: buf = {buf}");

    // 下面三行如果取消注释，会编译失败（帮助你用错误信息反推规则）：
    //
    // let buf2 = String::from("x"); // 注意：这里没有 `mut`
    // let bad: &mut String = &mut buf2;
    //                        ^^^^^^^^^
    // E0596：`buf2` 不是 `mut` 绑定，不能借出 `&mut`（“cannot borrow as mutable”）。

    println!("记忆锚点：类型里出现 `&` / `&mut` 才是引用；单独的 `T` 多半是拥有。");
}

/// 1) 多个 `&T` 可以并存：读者不会互相破坏状态。
fn demonstrate_multiple_shared_borrows() {
    println!("-- (1) 多个 &T 可以同时存在 --");

    let values = vec![4, 5, 6];

    // `r1: &Vec<_>`：引用目标仍是栈上的 `Vec` 头，再由头里指针指到堆元素；多个 `&` = 多条并行的只读入口。
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
///
/// ## 核心直觉（配合函数体里的 ASCII）
///
/// - 你手里已经有一条 **`&mut` 独占通道**（`outer`）。
/// - 有时需要先**只读扫一眼**底层数据：`&*outer` 从这条可变借用上**临时再借**出一条 `&`（只读）。
/// - 这条 `&` **从属于**原来的 `&mut`：它活着的时候，编译器会暂时限制你对 `outer` 的下一步使用；
///   一旦 `&` 在 NLL 意义下**最后一次使用结束**，独占权又回到 `outer`，才能 `push`。
///
/// ## `&*outer` 怎么读（拆符号）
///
/// - `*outer`：把 `&mut Vec` **解引用**成目标 `Vec`（仍在原处，不是 move）。
/// - `&*outer`：再对它取**共享不可变借用**，得到 `&Vec`，这叫 **reborrow**。
fn demonstrate_reborrow() {
    println!("-- (6) 再借用：从 &mut 里临时切出一条 &T --");

    // -------------------------------------------------------------------------
    // 时间线（横轴 = 代码顺序；纵轴 = “谁握着独占写权限”）
    // -------------------------------------------------------------------------
    //
    //   values          outer (&mut)              snapshot (&)
    //   所有者           独占写 lease               从 outer 派生的只读再借用
    //     │                 │                          │
    //     │    ┌────────────┴────────────┐            │
    //     │    │ 这段区间：mut 仍归 outer，  │            │
    //     │    │ 但短暂插进一条只读旁路     ├────────────┘
    //     │    └────────────┬────────────┘
    //     │                 │
    //     │    println!(snapshot)  ←── snapshot 最后一次使用；之后旁路关闭
    //     │                 │
    //     │    outer.push   ←── 独占写通道恢复给 outer
    //     ▼                 ▼
    //
    // ASCII（逻辑结构）：
    //
    //        栈 / 所有者                         堆
    //   ┌──────────────────────┐            ┌─────────────┐
    //   │ values: Vec 头       │───────────▶│ 1 2 3 ...   │
    //   │  0x1053a9960         │            │             │
    //   └──────────────────────┘            └─────────────┘
    //            ▲
    //            │ `outer: &mut Vec`（独占可变借用：改的是 `values` 背后同一块数据）
    //   ┌────────┴────────┐
    //   │outer 0x1053a9960│
    //   └────────┬────────┘
    //            │ 再借用：`snapshot: &Vec` 指向**同一块** Vec 头，但只能读
    //   ┌────────┴────────┐
    //   │ snapshot        │  ← `let snapshot = &*outer`
    //   └─────────────────┘
    //
    // 误区纠正：再借用**不是** clone 一份新 Vec；只是多一个**只读别名**，寿命受 NLL 约束。
    //
    let mut values = vec![1, 2, 3];
    // 在建立 `&mut` 之前先打印 `values`：一旦 `let outer = &mut values` 生效，就不能再 `addr_of!(values)`（会与 `&mut` 重叠）。
    println!(
        "（借出 &mut 前）`values` 的 Vec 头地址 = {:p}，堆元素起址 = {:p}",
        std::ptr::addr_of!(values),
        values.as_ptr()
    );

    let outer = &mut values;
    // `outer` 本身在栈上有个槽；它保存的“指向目标”应等于上一行的 Vec 头地址。
    println!(
        "（借出 &mut 后）`outer` 绑定槽位 = {:p}，`outer` 指向的 Vec 头 = {:p}，堆元素起址 = {:p}",
        std::ptr::addr_of!(outer),
        outer as *const Vec<i32>,
        outer.as_ptr()
    );

    // `&*outer` 是什么意思？和 `&outer` 差在哪？
    //
    // - 已知 `outer` 的类型是 `&mut Vec<i32>`（指向 `values` 里那份 `Vec` 的**可变借用**）。
    // - `*outer`：对可变引用**解引用**，得到“背后的那个 `Vec`”（在原位，不是 move）。
    // - `&*outer`：再对这个 `Vec` 取**不可变借用** → 类型是 `&Vec<i32>`，即**对数据的只读视图**（reborrow）。
    //
    // 若写成 `&outer`，含义完全不同：`outer` 这个**局部变量**（它自己就是 `&mut Vec`）再取一层引用，
    // 得到的是 `&&mut Vec<i32>` —— 指向的是“栈上的那条可变借用”，不是“借用目标里的 `Vec` 头”。
    // 因此读数据一般用 `&*outer` / `outer.deref()` 思路，而不是 `&outer`。
    //
    // 调用 `outer.len()` 等方法时，编译器也常自动插入类似的再借用，原理同源。
    let snapshot: &Vec<i32> = &*outer;
    println!("reborrowed snapshot: {snapshot:?}");
    println!(
        "再借用指向的 Vec 头地址（应与 `&*outer` 一致）: snapshot={:p}, &*outer={:p}",
        snapshot as *const Vec<i32>,
        &*outer as *const Vec<i32>
    );
    // 含 `snapshot` 的 `println!` 全部执行完后，`snapshot` 的活跃区间结束；NLL 关闭只读旁路，`outer` 可继续 `push`。
    //
    // 若在使用 `snapshot` 的同一区间里又对 `outer` 做 `push`（读写区间重叠），常见报错：E0499 / E0502。

    outer.push(4);
    println!("after push via outer: {outer:?}");

    // 上面这行 `println!` 用掉了 `outer` 的**最后一次**使用 → NLL 认为 **`&mut values` 这段借用结束**。
    // 重点：`outer` 从来就不是“另一份 Vec”，只是 **`values` 的可变别名**；所以 push 改的是同一块堆数据。
    // 借用结束后，再用所有者名字 `values` 读，应看到与 `outer` 里一致的内容 `[1,2,3,4]`。
    println!("`outer` 的借用结束后，所有者 `values` 读到的仍是同一份数据: {values:?}");
}

/// 7) **词法块**结束即释放借用：内层 `&mut` 结束后，外层又能自由用 `vec`
///
/// 和 NLL（按“最后一次使用”结束借用）不同，这里靠的是**显式大括号作用域**：
/// `borrowed` 只在内层块里存活，块一结束，**独占可变借用**随之结束，
/// 外层的 `vec` 不再被“租约”挡住，可以继续 `println!` / 再借 `&` / `&mut`。
fn demonstrate_mut_borrow_released_by_block_scope() {
    println!("-- (7) 内层 `}}` 结束 → `&mut` 借用释放，所有者 `vec` 继续可用 --");

    let mut vec = vec![1, 2, 3];
    {
        // 仅在 `{}` 内：`borrowed` 独占 `vec`；此区间里不能再写 `&vec` / 第二个 `&mut vec`。
        let borrowed = &mut vec;
        borrowed.push(4);
        println!("块内通过 &mut 修改后: {borrowed:?}");
    }
    // 离开块后，`borrowed` 消失，**独占期结束**；下面用的是所有者 `vec` 本身，合法。
    println!("块外所有者视角（应含 push 进去的 4）: vec = {vec:?}");
}

pub fn run() {
    println!("== Borrowing Basics ==");

    demonstrate_owned_vs_borrow_vs_mut_borrow();
    println!();

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

    demonstrate_mut_borrow_released_by_block_scope();
    println!();


    println!("规则总结：同一时刻 可以有 多个 &T，或者 一个 &mut T，但不能同时混用；判据是“区间是否重叠”。");
    println!();
}
