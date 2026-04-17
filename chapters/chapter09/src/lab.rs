// 这一章的练习重点是：把“数据结构形状”和“所有权模型”一起设计。
// 如果只想数据结构接口、不想节点之间谁拥有谁，Rust 很快就会把问题暴露出来。
pub fn run() {
    println!("== Lab ==");
    println!("1. 为单向链表补一个 len() 或 is_empty() 方法");
    println!("2. 试着为单向链表添加 peek()，返回头节点值的引用或拷贝");
    println!("3. 为双向链表补一个 tail_value() 或 push_back()");
    println!("4. 画出双向链表里 head / tail / next / prev 的关系图");
    println!("5. 把 parent 从 Rc 改成 Weak，观察引用计数语义为什么更合理");
    println!();
}
