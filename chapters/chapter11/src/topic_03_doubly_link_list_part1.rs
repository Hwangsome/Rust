//! 双向链表（第 1 部分）：为什么必须用 `Rc<RefCell<Node>>`？
//!
//! 双向链表里，每个节点既被链表从头尾拥有，又被前后节点互相引用：
//!
//! ```text
//!  head ────→ A ⇄ B ⇄ C ←──── tail
//! ```
//!
//! 分析所有权：
//! - **多个所有者**：至少有 `A.next`、`B.prev`、`head` 同时"指着"节点 A
//!   → 需要 `Rc<Node>`
//! - **可变**：插入 / 删除时要改 `prev` / `next`
//!   → `Rc` 是只读的，需要 `RefCell` 让内部字段可变
//!
//! 于是节点类型定下来：`Rc<RefCell<Node>>`——这一组合正是上章介绍的"多所有者 + 内部可变"模式。
//!
//! 注意：严格实现的生产级双向链表里 `prev` 应该用 `Weak<RefCell<Node>>` 避免引用环（见 topic_05）。
//! 本教学实现为了简化用了 Rc——在极少数拆链场景才可能漏释放。

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Default)]
struct DoublyLinkedList {
    head: Link,
    tail: Link,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            element,
            next: None,
            prev: None,
        }))
    }
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self::default()
    }

    fn push_front(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
    }

    fn head_value(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.borrow().element)
    }

    fn tail_value(&self) -> Option<i32> {
        self.tail.as_ref().map(|node| node.borrow().element)
    }
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Doubly Linked List (Part 1) ==");

    let mut list = DoublyLinkedList::new();
    list.push_front(30);
    list.push_front(32);
    list.push_front(34);

    println!(
        "head = {:?}, tail = {:?}",
        list.head_value(),
        list.tail_value()
    );
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Doubly Link List (Part 1)
// -------------------------------------------

use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Doubly_Linklist {
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
    prev: pointer,
}

type pointer = Option<Rc<RefCell<Node>>>;

impl Doubly_Linklist {
    fn new() -> Self {
        Doubly_Linklist {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}
fn main() {}


/* 
---------------------------------------------------------------------------------------------------------
Concept / Topic     | Explanation
--------------------|------------------------------------------------------------------------------------
Doubly Linked List  | It allows navigation in both forward and backward directions.
                    | Each node contains two pointers: one to the next node and one to the previous node.
                    | The list maintains explicit references to both the head and the tail nodes.

Multiple Ownership  | The doubly link list requires multiple ownership with mutability access. 
with Mutability     | The Rc smart pointer allow multiple owners of the same data. 
                    | However, we it does not provide mutablity access.  
                    | Wrapping nodes inside Rc<RefCell<Node>> allows multiple owners to mutate the Node.

Node definition     | The node structure is extended to include both next and previous pointers.
                    | The wrapper list structure now stores both head and tail references.

Adding a Node       | If the list is empty, the new node becomes both the head and the tail.
at the Front        | If the list already has a head, the previous of old head is updated.
                    | Additionally, the next of new node is set to the old head and the head is updated.                 
-----------------------------------------------------------------------------------------------------------
*/
"###;
