//! 所有权基础：Copy / move / clone / 借用（只读）/ 作用域 / shadowing
//!
//! 这一节要回答初学者最容易卡住的一组问题：
//!
//! - `let s2 = s1` 之后，为什么有的类型可以继续用 `s1`，有的不行？
//! - “值被释放”是什么时候发生的？
//! - `let s = s;`（同名 shadowing）是不是 move？
//! - 字符串字面量 `"hi"` 和 `String::from("hi")` 在所有权层面有什么区别？
//!
//! 运行后建议按这个顺序观察：
//! 1. Copy 类型：赋值之后，原值仍可用。
//! 2. 堆分配类型（`String`、`Vec<T>`）：赋值之后默认发生 move，原绑定失效。
//! 3. 想得到“两份独立数据”——用 `clone`。
//! 4. 只想多一个只读视角——用 `&T` 借用。
//! 4b. **`Vec` 不是 `Copy`，但 `&Vec` 这种不可变引用类型是 `Copy`**：`let r2 = r1` 复制的是引用。
//! 5. 作用域结束时，值会被 drop（释放）。
//! 6. Shadowing 和 move 的区别。
//! 7. 字符串字面量 `&'static str` 为什么不会 move。
//!
//! 和下一节的关系：本节只在变量赋值层面讨论，
//! 下一节 `topic_02_ownership_in_functions` 会把同样的规则搬到函数调用边界上。

/// 1) `Copy` 类型：赋值生成**第二份彼此独立的值**，原绑定仍可用（不是 move）。
///
/// ## 心智模型（先抓住这句）
///
/// - **`Copy` 赋值**：像“抄一张小纸条”，`x` 还在，`y` 是另一张写着同样数字的纸条。
/// - **`move` 赋值**（典型如 `String`）：像“把唯一钥匙交给对方”，旧名字不再代表那份资源。
///
/// ## `Copy` 在 Rust 里到底保证什么？
///
/// `Copy` **不是**在承诺“一定发生在栈上”。它承诺的是更机械的一件事：
///
/// > 允许用 **按位拷贝（`memcpy` 语义）** 复制出第二份值，并且两份都合法、
/// > 不会隐式复制/共享同一块需要 `drop` 的堆资源，从而避免双重释放等问题。
///
/// 对本节的 `i32` / `(i32, bool)` 这类局部变量：实现上通常就是把几个机器字再写一份到当前栈帧里，
/// 所以口语说“栈上复制”往往**能对上直觉**；但把 `Copy` **等同于**“栈上复制”会踩坑——例如 `&T`
/// 也是 `Copy`，复制的是**引用这个胖/瘦指针本身**，不是把 `T` 的堆内容再拷贝一遍。
///
/// ## 常见 `Copy` / 非 `Copy`（帮助你对照后面几段 demo）
///
/// - `Copy`：`i32`、`bool`、`char`、浮点、引用 `&T`（当成员全 `Copy` 时，元组/数组也可 `Copy`）
/// - **不是** `Copy`：`String`、`Vec<T>`、`Box<T>` 等“持有堆资源并负责释放”的类型
fn demonstrate_copy_types() {
    println!("-- (1) i32 等 Copy 类型：赋值不是 move --");

    // `i32` 是 `Copy`：`let y = x` 不会把 `x` “搬走”，而是再生成一个同样为 15 的 `i32`。
    // 这两份值在语言规则上彼此独立：改 `y` 不会影响 `x`（本例没改，只是强调语义）。
    let x: i32 = 15;
    let y = x; // 按位复制 `x` 的比特模式到 `y`（不调用 `Clone::clone()`）

    // 观察点：`x` 与 `y` 同时可用 —— 这就是“不是 move”的直接证据。
    // 若 `i32` 不是 `Copy`，`let y = x` 会像 `String` 那样让 `x` 失效（但 `i32` 不会）。
    println!("Copy 类型赋值后原值仍可用: x = {x}, y = {y}");

    // -------------------------------------------------------------------------
    // 地址打印：别把它理解成“两个指针指向堆里的 15”
    // -------------------------------------------------------------------------
    //
    // `addr_of!(x)` / `addr_of!(y)` 打印的是：**栈上这两个局部变量槽位自身的地址**。
    // 对 `i32` 来说，值 `15` 通常就**内联**存放在槽位里的 4 个字节中，并没有“堆上另有一份 15”。
    //
    // 栈帧（示意；真实地址每次运行都会变；`x`/`y` 的相对排布也可能因优化略有不同）：
    //
    //      高地址 ↑
    //   ┌───────────────────────────────┐
    //   │  ... 调用者栈 ...               │
    //   ├───────────────────────────────┤
    //   │  y : [15, 0, 0, 0]  (4 bytes)   │  ← `addr_of!(y)` 指向这一格的开头
    //   ├───────────────────────────────┤
    //   │  x : [15, 0, 0, 0]  (4 bytes)   │  ← `addr_of!(x)` 指向这一格的开头
    //   ├───────────────────────────────┤
    //   │  ... 其它局部量 / 保存的寄存器 ... │
    //   └───────────────────────────────┘
    //      低地址 ↓
    //
    // 对比 `String`（后面 demo 会讲）：栈上更像“指针+长度+容量”，字符字节在堆。
    //
    // 下面两行输出：
    // - `{:p}`：两个槽位地址，通常不同（两份独立存储的证据）
    // - `to_ne_bytes()`：槽位里装着的比特（本例两份应相同，因为都是 15）
    println!(
        "两份 i32 的存放地址: &x = {:p}, &y = {:p}",
        std::ptr::addr_of!(x),
        std::ptr::addr_of!(y)
    );
    println!(
        "两份 i32 的字节表示（本机字节序）: x = {:?}, y = {:?}",
        x.to_ne_bytes(),
        y.to_ne_bytes()
    );

    // 元组/数组的 `Copy` 是“结构性”的：所有字段都能按位安全复制时，整体也 `Copy`。
    // 这里 `(i32, bool)` 两个成员都 `Copy`，因此整段元组布局被一次性按位复制到 `pair_copy`。
    let pair: (i32, bool) = (7, true);
    let pair_copy = pair; // 等价于分别复制两个 `Copy` 字段，但整体是一条拷贝指令的直觉
    println!("Copy 元组赋值后原值仍可用: pair = {pair:?}, pair_copy = {pair_copy:?}");
}

/// 2) `String` 的默认赋值是 move：只有新所有者可用。
///
/// 这是问题 “为什么 `s1 = String::from("hello"); let s2 = s1; println!("{s1}")` 会报 E0382”
/// 的最小复刻版本。
fn demonstrate_string_move() {
    println!("-- (2) String 的默认赋值是 move --");

    let s1 = String::from("hello");
    let s2 = s1;


    // 这里要观察的是：`s2` 拥有堆上的 `"hello"`，`s1` 这个绑定已经被编译器标记为失效。
    println!("move 后仍可用的是新所有者: s2 = {s2}");

    // 若取消下面这行的注释，会触发 E0382（典型提示：
    // `use of moved value: s1` 或 `borrow of moved value: s1`）：
    //
    // 含义是：`s1` 所持有的堆缓冲区的“所有权记号”已经交给 `s2` 了。
    // Rust 为了避免同一份堆内存被两个变量误当作各自负责的资源（double free），
    // 选择让旧绑定直接失效，而不是做一次隐式浅拷贝。
    //
    // println!("s1 = {s1}");
}

/// 3) `Vec<T>` 的赋值也是 move——和 `String` 是同一条规则。
///
/// 这里要打破一个常见误区：不要觉得只有 `String` 才会 move。
/// 凡是“自己持有堆上资源”的类型，默认都走 move 语义。
fn demonstrate_vec_move() {
    println!("-- (3) Vec<T> 的赋值也是 move --");

    let v1 = vec![1, 2, 3];
    let v2 = v1;

    println!("move 后仍可用的是新所有者: v2 = {v2:?}");

    // 取消下面这行注释同样会报 E0382：
    // println!("v1 = {v1:?}");
}

/// 4) `clone`：显式深拷贝，得到两个彼此独立的所有者。
///
/// 这里要强调：`clone` 不是“免费的编译通过按钮”——
/// 它通常会做一次堆分配和内容复制，成本不小。
fn demonstrate_clone_for_heap_types() {
    println!("-- (4) 需要两份独立数据：用 clone --");

    let s1 = String::from("world");
    let s2 = s1.clone();

    // -------------------------------------------------------------------------
    // 常见误区：`clone()` 之后是不是“两个栈地址指向**同一个**堆上的 String”？
    //
    // **不是。** `String` 在栈上只是一小块元数据（指针/长度/容量，具体布局别死记），
    // `clone()` 会**再分配一块堆缓冲区**，把字符内容复制过去，于是：
    //
    // - 栈上：`s1`、`s2` 两个槽位地址不同（`addr_of!`）
    // - 堆上：也有**两份** UTF-8 缓冲（`as_ptr()` 通常不同；内容一开始相同）
    //
    // 只有“浅拷贝指针、两个 `String` 指向同一块堆”才会共享堆缓冲，但那是**未定义行为/
    // 违背 `String` 不变量**的画风；标准库的 `clone()` 做的是合法深拷贝。
    //
    // 示意（逻辑结构，非精确内存布局）：
    //
    //   栈                         堆
    //   ┌──────────────┐           ┌─────────────┐
    //   │ s1: ptr,len..│──────────▶│ "world"     │
    //   └──────────────┘           └─────────────┘
    //   ┌──────────────┐           ┌─────────────┐
    //   │ s2: ptr,len..│──────────▶│ "world"拷贝 │
    //   └──────────────┘           └─────────────┘
    //
    println!(
        "两个 String **栈槽**地址（元数据存放处）: &s1 = {:p}, &s2 = {:p}",
        std::ptr::addr_of!(s1),
        std::ptr::addr_of!(s2)
    );
    println!(
        "两个 String **堆缓冲**起始地址（UTF-8 字节）: s1.as_ptr() = {:p}, s2.as_ptr() = {:p}",
        s1.as_ptr(),
        s2.as_ptr()
    );

    // 这里要观察的是：s1 和 s2 是两份完全独立的 String，修改其中一个不会影响另一个。
    let mut s2 = s2;
    s2.push_str("!");
    println!("clone 后两个 String 互不影响: s1 = {s1}, s2 = {s2}");
}

/// 5) 只读借用 `&T`：不转移所有权，也能让另一个名字“看到同一份数据”。
///
/// 这是“避免 clone”最常见的办法：当你只想读的时候，就借一下，别真的复制。
fn demonstrate_shared_immutable_borrow() {
    println!("-- (5) 只想共享只读视图：用 &T --");

    let greeting = String::from("hello");
    let view1 = &greeting;
    let view2 = &greeting;

    // -------------------------------------------------------------------------
    // 和 `(4) clone` 对照：这里**没有**第二份堆缓冲
    // -------------------------------------------------------------------------
    //
    // - `greeting`：唯一所有者；堆上只有一份 `"hello"` 的 UTF-8 字节。
    // - `view1` / `view2`：类型都是 `&String`（对同一个 `greeting` 的**只读借用**）。
    //
    // `&T` 本身是 `Copy`：你可以把 `view2` 理解成“又抄了一份**引用值**（地址）”，
    // 但抄的是“指向同一份 `greeting` 的地址”，不是把 `String` 在堆里再复制一份。
    //
    // 示意（逻辑结构）：
    //
    //        栈                              堆
    //   ┌────────────────┐                ┌──────────────┐
    //   │ greeting       │──ptr,len,cap──▶│ hello\0...   │
    //   └────────────────┘                └──────────────┘
    //          ▲           ▲
    //          │           └──────────────┐
    //   ┌──────┴─────┐              ┌────┴─────┐
    //   │ view1: &String              │ view2: &String
    //   └────────────┘              └──────────┘
    //   （两个“只读入口”，通常指向**同一个** `greeting` 槽位 / 同一路径读到同一块堆）
    //
    // 下面打印帮你验证两件事：
    // - `view1 as *const String` 与 `view2 as *const String`：通常相同（都指向 `greeting` 这份 `String`）
    // - `as_ptr()`：堆缓冲起始地址，`greeting` 与 `view1` 通常相同（同一个底层 UTF-8）
    println!("借用不改变所有者: greeting = {greeting}, view1 = {view1}, view2 = {view2}");
    println!(
        "两个 &String 指向的 **String 头**地址: greeting= {:p},view1 = {:p}, view2 = {:p}",
        std::ptr::addr_of!(greeting),
        view1 as *const String,
        view2 as *const String
    );
    println!(
        "UTF-8 **堆缓冲**起始地址: greeting.as_ptr() = {:p}, view1.as_ptr() = {:p}",
        greeting.as_ptr(),
        view1.as_ptr()
    );
}

/// 5b) 补充：`let r2 = r1`，若 `r1` 的类型是 **`&T`（不可变借用）**，赋值走的是 **`Copy`**
///
/// 新手最容易混的一句话：**“`Vec` 不能 Copy，所以借用也不能 Copy？”——不对。**
///
/// - **`Vec<i32>` 本身不是 `Copy`**：直接 `let v2 = v1` 会把 `Vec` **move** 走。
/// - **`&Vec<i32>` 作为类型实现了 `Copy`**：`let r2 = r1` 复制的是**引用这份小数据**（像多复印一张“门票”），
///   **不会**隐式去 `clone` 整个 `Vec`；堆上的那份 `Vec` 仍由 `vec` 唯一拥有。
///
/// 对照（只记结论即可，细节在借用章节展开）：**`&mut T` 不是 `Copy`**，不能靠赋值造出两个同时活跃的 `&mut`。
fn demonstrate_immutable_ref_is_copy() {
    println!("-- (5b) `&T` 引用值本身是 Copy：`let r2 = r1` 后 r1 仍可用 --");

    // -------------------------------------------------------------------------
    // 人话：`Vec` 不 Copy，但“指向 Vec 的那张门票”`&Vec` 是 Copy
    // -------------------------------------------------------------------------
    //
    // `let borrowed2 = borrowed1` 复制的是 **引用变量里存的那几个机器字**（指向谁的地址信息），
    // 不是把堆里的 `[1,2,3]` 再 clone 一份，也不是把 `vec` 这个所有者 move 走。
    //
    // ASCII（逻辑结构；`Vec` 头是 ptr/len/cap，别死记字段顺序）：
    //
    //      栈                                              堆
    //   ┌────────────────────────────┐                ┌─────────────┐
    //   │ vec: Vec 头 ─────────────────┼───────────────▶│ 1  2  3 ... │
    //   └────────────────────────────┘                └─────────────┘
    //            ▲                     ▲
    //            │    同一块地址       │
    //   ┌────────┴─────────┐   ┌───────┴──────────┐
    //   │ borrowed1: &Vec  │   │ borrowed2: &Vec  │
    //   │ （第一份引用值）  │   │ （第二份：Copy） │
    //   └──────────────────┘   └──────────────────┘
    //
    // 所以：`borrowed1 as *const Vec == borrowed2 as *const Vec`（都指向 `vec` 这份头）。
    // `vec` 仍是唯一所有者；两个借用只是两个**并列的只读入口**。
    //
    let vec = vec![1, 2, 3];
    let borrowed1: &Vec<i32> = &vec;
    // 下面这行**不是** move `Vec`，而是 `Copy` 这份 `&Vec<i32>`（“第二个名字拿同一张只读门票”）。
    let borrowed2 = borrowed1;

    println!("所有者 vec 仍可用: {vec:?}");
    println!("borrowed1 = {borrowed1:?}, borrowed2 = {borrowed2:?}");
    // `{:p}`：把引用**解码**成“指向的 `Vec` 头在哪儿”；Copy 后两个引用目标应相同。
    println!(
        "两个 &Vec 指向同一份 Vec 头（地址应相同）: {:p}, {:p}",
        borrowed1 as *const Vec<i32>,
        borrowed2 as *const Vec<i32>
    );
}

/// 6) 作用域与 drop：值在其所有者离开作用域时被释放。
///
/// Rust 没有 GC，资源回收靠“作用域 + 所有者”这一组静态规则。
/// 这个例子通过一对大括号构造一个内部作用域，演示：
/// - 在内部作用域里创建的 `String`，会在 `}` 时被自动释放
/// - 所以离开这个作用域后，就再也没法访问它
fn demonstrate_scope_and_drop() {
    println!("-- (6) 作用域结束时值会被 drop --");

    {
        let inner = String::from("temp");
        println!("进入内部作用域，inner = {inner}");
        // 这一行之后，内部作用域结束，`inner` 会被释放。
        // Rust 不需要你手动写 free / delete，编译期就能知道什么时候释放。
    }

    // 此时如果尝试使用 `inner`，会报“cannot find value `inner` in this scope”，
    // 因为作用域内的绑定名也一起消失了。
    println!("离开内部作用域后，inner 已经不存在了。");
}

/// 7) Shadowing（遮蔽）不是 move，而是“用同名新变量覆盖旧绑定”。
///
/// 这一节想明确区分：
/// - `let s = s;`（左侧没有新信息，只是重绑定）——对 `String` 来说这就是 move
/// - `let s = s.to_uppercase();`（左侧是新值）——创建新 `String`，旧 `String` 被 drop
/// - `let x = 5; let x = x + 1;`——对 Copy 类型而言是 shadowing，不是 move
fn demonstrate_shadowing_vs_move() {
    println!("-- (7) Shadowing 与 move 的对比 --");

    // 7a. Copy 类型的 shadowing：只是“换一个同名绑定”，类型甚至可以改变。
    let x = 5;
    let x = x + 1; // shadowing：新的 x 遮蔽旧的 x
    let x = format!("x-as-string-{x}"); // 类型从 i32 变成 String，也合法
    println!("Copy 类型 shadowing 可以连类型都换掉: x = {x}");

    // 7b. 持堆类型的 shadowing：旧值在新绑定建立后会被 drop。
    let s = String::from("lower");
    let s = s.to_uppercase(); // 旧 s 被消费（move 进 to_uppercase），返回新 String 覆盖同名
    println!("String 的 shadowing: s = {s}");
}

/// 8) 字符串字面量 `&'static str` 为什么不会 move。
///
/// 这里想解决一个新手常见疑惑：
/// - `String::from("hi")` 得到的是 `String`，会 move
/// - 但 `"hi"` 本身是 `&str`（准确说是 `&'static str`），是“借用”
///
/// `&str` 是 **胖指针**（数据指针 + 长度），并且实现 **`Copy`**：
/// `let another = literal` 只是再复制一份“句柄”，不会把底层字节从谁手里“搬走”。
/// 字面量字节通常躺在**只读数据段**（随二进制一起加载），**不由某个变量负责 free**，
/// 因此不存在“把所有权从 literal 交给 another”这种 `String` 意义上的 move。
fn demonstrate_str_literal_is_not_string() {
    println!("-- (8) &str 字面量不会 move --");

    // -------------------------------------------------------------------------
    // 为什么 `let another = literal` 之后两个都还能用？
    // -------------------------------------------------------------------------
    //
    // 1) `literal` 的类型是 `&'static str`：对程序里某段 **UTF-8 字节** 的只读视图。
    // 2) 这段字节一般编进可执行文件的只读区（概念示意，别和虚拟地址一一较真）：
    //
    //        只读数据段（示意）                栈（示意）
    //   ┌───────────────────────┐        ┌─────────────────────────┐
    //   │  'h'  'i'  （字面量）   │        │ literal: ptr + len      │
    //   └───────────────────────┘        │ another: ptr + len(拷贝) │
    //            ▲  ▲                     └─────────────────────────┘
    //            │  └──────────────────────────┘
    //            └──────────────┘
    //            两个 `&str` 里保存的“指针+长度”可以相同：`&str` 是 Copy，不是 move 走底层。
    //
    // 3) 因为没有“唯一所有者要交出去”，也就不会出现 `String` 那种 E0382。
    //
    let literal = "hi"; // &'static str
    let another = literal; // 复制的是胖指针本身（ptr+len），不是把只读段“搬家”
    println!("两个 &str 都可用: literal = {literal}, another = {another}");
    println!(
        "两个 &str 指向的数据起始地址（通常相同）: literal.as_ptr() = {:p}, another.as_ptr() = {:p}",
        literal.as_ptr(),
        another.as_ptr()
    );

    // -------------------------------------------------------------------------
    // 对比：`String` 是“拥有堆缓冲”的类型
    // -------------------------------------------------------------------------
    //
    // `String::from("hi")` 会在堆里**新分配**一块缓冲区，把 `'h''i'` 拷进去；
    // 栈上的 `owned` 保存 ptr/len/cap，**要对这块堆内存负责 drop**。
    //
    //        栈                         堆（示意）
    //   ┌──────────────┐           ┌──────────────┐
    //   │ owned        │──ptr,len──▶│ hi + 容量...  │
    //   └──────────────┘           └──────────────┘
    //
    // `let moved = owned;` 把“负责释放的那份所有权”交给 `moved`，`owned` 失效（move）。
    //
    let owned = String::from(literal);
    println!(
        "String 堆缓冲起始地址（移动前）: owned.as_ptr() = {:p}",
        owned.as_ptr()
    );
    let moved = owned;
    println!(
        "String 版本 move 后新所有者: moved = {moved}, moved.as_ptr() = {:p}（同一块堆缓冲，只是所有者改名）",
        moved.as_ptr()
    );
    // println!("owned = {owned}"); // ← 如果取消注释，会触发 E0382
}

pub fn run() {
    println!("== Ownership Basics ==");

    // 顺序经过精心设计：先建立直觉（Copy），再看反直觉（move），
    // 然后给出两条常见解法（clone / &T），再补上作用域与 shadowing 这两条背景知识。
    demonstrate_copy_types();
    println!();

    demonstrate_string_move();
    println!();

    demonstrate_vec_move();
    println!();

    demonstrate_clone_for_heap_types();
    println!();

    demonstrate_shared_immutable_borrow();
    println!();

    demonstrate_immutable_ref_is_copy();
    println!();

    demonstrate_scope_and_drop();
    println!();

    demonstrate_shadowing_vs_move();
    println!();

    demonstrate_str_literal_is_not_string();
    println!();
}
