// 这一章的练习重点是：不要把生命周期和智能指针分开背。
// 更好的做法是始终问自己三个问题：谁拥有值、值活多久、谁能修改它。
pub fn run() {
    println!("== Lab ==");
    println!("1. 写一个返回较长字符串切片的函数，并给出生命周期标注");
    println!("2. 写一个持有切片引用的 struct，并实现一个读取方法");
    println!("3. 用 Box 定义一个最小递归 enum");
    println!("4. 用 Rc 共享一段字符串，并打印 strong_count");
    println!("5. 用 RefCell 在不可变绑定下修改内部 Vec");
    println!("6. 尝试组合 Rc<RefCell<T>>，理解它解决的两个问题分别是什么");
    println!();
}
