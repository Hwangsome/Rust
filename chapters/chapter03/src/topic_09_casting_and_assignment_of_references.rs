//! 引用在赋值 / 转换时的行为：`&T` 是 `Copy`、`&mut T` 不是。
//!
//! 这一节要消除两个常见误解：
//! 1. "引用就是一块内存"——不是。引用本身是一个**值**，有自己的类型和行为
//! 2. "`&mut T` 和 `&T` 只是可变性不同"——它们的**赋值行为**也不同：
//!    - `&T` 实现了 `Copy`，所以 `let r2 = r1;` 只是复制这个引用
//!    - `&mut T` **不**实现 `Copy`，赋值会**转移**这个可变借用的所有权
//!
//! 本节还演示：
//! - **reborrow**（再借用）：从 `&mut T` 临时借一条 `&T` 出来用
//! - **deref coercion**（解引用强转）：`&String` 可以当 `&str` 传
//!
//! 这些概念在第 2 章已经打过基础，本节放在 chapter03 的末尾是为了把它们
//! 再放进"类型/方法"的语境里巩固一次。

pub fn run() {
    println!("== Casting And Assignment Of References ==");

    println!("-- (1) &T 是 Copy：赋值后两个引用都可用 --");
    let text = String::from("Rust");
    let ref1 = &text;
    let ref2 = ref1; // 复制这个引用，不是 move
    println!("两个 &String 都可用: ref1 = {ref1}, ref2 = {ref2}");
    println!();

    println!("-- (2) &mut T 不是 Copy：赋值会 move --");
    let mut data = 42;
    let mut_ref1 = &mut data;
    let mut_ref2 = mut_ref1; // 这里是 **move**，不是复制
    *mut_ref2 += 1;
    // println!("{mut_ref1}"); // ← 会 E0382，mut_ref1 已经 move 给 mut_ref2
    println!("通过 mut_ref2 改完之后 data = {}", *mut_ref2);
    println!();

    println!("-- (3) reborrow：从 &mut 临时借一条 &T --");
    let mut number = 100;
    let outer = &mut number;
    {
        // `&*outer` 从 outer 再借一条只读引用。
        let snapshot: &i32 = &*outer;
        println!("reborrow 得到的只读快照: {snapshot}");
        // snapshot 在这一行之后就不再被使用，NLL 判定它结束了。
    }
    // 只读快照结束后，outer 仍然可以写。
    *outer += 1;
    println!("reborrow 结束后通过 outer 写回: number = {}", *outer);
    println!();

    println!("-- (4) deref 强转: &String 可以当 &str --");
    let owned = String::from("hello");
    // 声明 `accepts_str` 在本模块里，让它接 &str。
    fn accepts_str(s: &str) -> usize {
        s.len()
    }
    // 调用时传 &owned，&String 会自动 deref 成 &str。
    println!("accepts_str(&owned) => {}", accepts_str(&owned));
    // 字面量本身就是 &str，照样可以传。
    println!("accepts_str(\"literal\") => {}", accepts_str("literal"));
    println!();

    println!("-- (5) 值相等 vs 地址相等 --");
    let a = String::from("same");
    let b = String::from("same");
    let ra = &a;
    let rb = &b;
    println!("ra == rb (Display 相等，内容比较): {}", ra == rb);
    println!("std::ptr::eq(ra, rb)            : {}", std::ptr::eq(ra, rb));
    println!();
}
