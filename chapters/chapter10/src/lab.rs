//! 第 9 章练习：手写数据结构时，**先设计所有权，再动数据指针**。

pub fn run() {
    println!("== Lab ==");

    println!("▷ 练习 1：给单向链表加 len() / is_empty()");
    println!("  - 遍历 self.head 的 Option<Box<Node>> 链，累加节点数");

    println!();

    println!("▷ 练习 2：给单向链表加 peek() 和 peek_mut()");
    println!("  - peek 返回 Option<&i32>（只读）");
    println!("  - peek_mut 返回 Option<&mut i32>（可修改第一个元素）");

    println!();

    println!("▷ 练习 3：把 pop_front 改成 pop_back");
    println!("  - 提示：需要走到倒数第二个节点，把它的 next 清掉，返回最后一个");

    println!();

    println!("▷ 练习 4：双向链表的 push_back / pop_back");
    println!("  - 维护 tail 指针");
    println!("  - 删除尾节点时，注意处理新尾的 next 字段");

    println!();

    println!("▷ 练习 5：prev 改成 Weak<RefCell<Node>>");
    println!("  - 把双向链表里的 prev: Option<Rc<RefCell<Node>>> 改成 Weak");
    println!("  - 观察删除节点时内存释放的时机差异（strong_count 归 0）");

    println!();

    println!("▷ 练习 6：树结构");
    println!("  - 写 `struct TreeNode {{ value, children: Vec<Rc<RefCell<TreeNode>>>, parent: Weak<...> }}`");
    println!("  - 实现从一个节点向上找到根");
    println!("  - 实现前序遍历打印所有节点");

    println!();

    println!("▷ 练习 7：不借助 Rc<RefCell<T>> 的数据结构");
    println!("  - 用 Vec<Node> + 索引实现一个“仿链表”（arena 风格）");
    println!("  - 对比：代码复杂度是升了还是降了？");

    println!();

    println!("完成标准：");
    println!("  - 看到一个链式数据结构能立即判断：用 Box / Rc / Weak 哪种组合");
    println!("  - 能解释为什么 pop_front 需要 .take()");
    println!("  - 能说清 parent 为什么要用 Weak 而不是 Rc");

    println!();
}
