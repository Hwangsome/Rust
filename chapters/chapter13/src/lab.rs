//! 第 12 章练习：把类型大小和 ZST 的技巧放到实战里。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：打印各种类型的大小");
    println!("  - 用 size_of 打印 i32 / (i32,i32) / [i32;3] / &[i32] / &str / &dyn Trait");
    println!("  - 猜答案，再跑验证");

    println!();

    println!("▷ 练习 2：?Sized 泛型");
    println!("  - 写 `fn show<T: ?Sized + Debug>(x: &T)`");
    println!("  - 试着传 &str、&[i32]、&i32，观察全部通过");

    println!();

    println!("▷ 练习 3：unsized coercion");
    println!("  - 写 `fn use_slice(s: &[i32])`，然后用 &[1,2,3] 传");
    println!("  - 观察 &[i32; 3] 自动变 &[i32]");

    println!();

    println!("▷ 练习 4：never 类型");
    println!("  - 写 `fn bail() -> ! {{ panic!(\"...\") }}`");
    println!("  - 在 if 里把它作为分支，观察类型推断");

    println!();

    println!("▷ 练习 5：Unit struct + 类型状态");
    println!("  - 仿照本节 Car<New/Delivered>，写 Connection<Disconnected/Connected>");
    println!("  - 只有 Connected 才有 send() 方法");

    println!();

    println!("▷ 练习 6：PhantomData");
    println!("  - 写 `struct Id<T>(u64, PhantomData<T>)`");
    println!("  - 用它做强类型的 UserId / OrderId 区分");

    println!();

    println!("完成标准：");
    println!("  - 能独立说清 Sized vs ?Sized 触发时机");
    println!("  - 能用 Unit struct 设计编译期状态机");
    println!("  - 知道 PhantomData 的 3 个用途（生命周期绑定 / drop check / 协变性）");

    println!();
}
