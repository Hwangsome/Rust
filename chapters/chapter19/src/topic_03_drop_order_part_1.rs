//! Drop 顺序 part 1：同一作用域内，**后声明的先 drop**（栈式 LIFO）。

struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

pub fn run() {
    println!("== Drop Order (part 1) ==");
    let _a = Announce("A (declared first)");
    let _b = Announce("B");
    let _c = Announce("C (declared last)");
    println!("  now leaving scope; expect C → B → A");
    println!();
}
