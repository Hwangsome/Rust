//! 不变（invariance）part 2：**`Cell<T>` / `RefCell<T>` 对 T 不变**。
//!
//! 原因同上：内部可变性意味着"隐式的 &mut 访问"——任何允许放宽的方向都可能出问题。
//!
//! 结论：**能写的容器通常是不变的**；纯只读的容器通常是协变的。
//!
//! ## 三种方差的一句话总结
//!
//! | 方差 | 典型 | 规则 |
//! |-----|-----|-----|
//! | 协变 | `&'a T`、`Box<T>`、`Vec<T>` | 长寿 → 短寿 OK |
//! | 逆变 | `fn(T)` 的参数 | 短寿 ← 长寿 OK（方向反过来）|
//! | 不变 | `&mut T`、`Cell<T>`、`UnsafeCell<T>` | 必须精确匹配 |

use std::cell::RefCell;

pub fn run() {
    println!("== Invariance (part 2): RefCell<T> ==");

    let cell = RefCell::new(vec![1, 2, 3]);
    cell.borrow_mut().push(4);
    println!("  cell = {:?}", cell.borrow());
    println!();

    println!("方差速记: 协变（只读）、逆变（函数参数）、不变（能写的容器）");
    println!();
}
