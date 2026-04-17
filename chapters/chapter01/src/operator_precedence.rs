// 这一节只做一件事：让读者看到“默认优先级”和“括号强制分组”会导致不同结果。
pub fn run() {
    println!("== Operator Precedence ==");

    let default_order = 2 + 3 * 4;
    let grouped_order = (2 + 3) * 4;

    println!("2 + 3 * 4 = {default_order}");
    println!("(2 + 3) * 4 = {grouped_order}");
    println!("乘除通常先于加减，括号可以显式改写求值顺序。");
    println!();
}
