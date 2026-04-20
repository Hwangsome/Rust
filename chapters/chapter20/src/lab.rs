pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：写 `impl Drop for T {{ ... }}` 记录 drop 时机，观察栈 LIFO");
    println!("▷ 练习 2：给 struct 各字段加 Drop，观察按声明顺序 drop");
    println!("▷ 练习 3：尝试写自引用 struct，看 E0597 / E0515");
    println!("▷ 练习 4：用 mem::forget / Box::leak 制造一个合法但永不 drop 的值");
    println!("▷ 练习 5：用 catch_unwind 捕获子作用域的 panic");
    println!();
}
