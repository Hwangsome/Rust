// 这一章的练习应该尽量亲手制造“不能编译”的场景。
// 对所有权和借用来说，编译器报错本身就是学习材料的一部分。
pub fn run() {
    println!("== Lab ==");
    println!("1. 写一个 move 之后原值不能再使用的 String 例子");
    println!("2. 写一个 take ownership 的函数，再写一个 give ownership 的函数");
    println!("3. 试着同时创建不可变借用和可变借用，观察编译器提示");
    println!("4. 写一个接收 &Vec<T> 的函数，再写一个接收 &mut Vec<T> 的函数");
    println!("5. 定义一个引用并手动解引用，观察 * 的作用");
    println!();
}
