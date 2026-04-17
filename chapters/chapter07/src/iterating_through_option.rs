// 这个文件演示 Option 也能参与迭代流水线。
// 运行时要观察：`Some` 会像“长度为 1 的小集合”，`None` 会像“空集合”。
// 这让很多“可选值”场景可以自然接到迭代器组合子后面。
pub fn run() {
    println!("== Iterating Through Option ==");

    let mut products = vec!["keyboard", "mouse"];
    products.extend(Some("monitor"));
    products.extend(None::<&str>);
    println!("extend with Option => {:?}", products);

    let tags = [Some("rust"), None, Some("iterator")];
    let flattened: Vec<&str> = tags.into_iter().flatten().collect();
    println!("flatten Option items => {:?}", flattened);
    println!();
}
