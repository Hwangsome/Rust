pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：给自己的 trait 加 `as_any() -> &dyn Any`，让下游能 downcast");
    println!("▷ 练习 2：用 Box<dyn Any>::downcast::<T>() 拿所有权版");
    println!("▷ 练习 3：用 TypeId::of::<T>() 做分支，而不取回具体值");
    println!();
}
