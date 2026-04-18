//! 双向链表（第 2 部分）：头删 + 遍历。
//!
//! 与单向链表相比，双向链表的"删一个头节点"要多做一件事：
//!
//! 1. 把旧的 head 从 `Rc<RefCell<Node>>` 里取出来（`.take()`）
//! 2. 把旧 head 的 `next` 作为**新的 head**
//! 3. **关键一步**：把新 head 的 `prev` 置空（否则它还指向已经不存在的前驱，形成悬挂引用的语义错误）
//!
//! 遍历时每一步都要 `.borrow()` 解开 `RefCell`，取 `.next.clone()`，继续前进。
//! 这就是为什么双向结构比单向链表多了明显的状态维护成本——也是工程上很多场景更愿意用 `VecDeque`
//! 或者索引式自定义结构的原因。

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

    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            let value = old_head.borrow().element;
            let next = old_head.borrow_mut().next.take();

            match next {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }

            value
        })
    }

    fn values(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.head.clone();

        while let Some(node) = current {
            values.push(node.borrow().element);
            current = node.borrow().next.clone();
        }

        values
    }
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Doubly Linked List (Part 2) ==");

    let mut list = DoublyLinkedList::new();
    list.push_front(30);
    list.push_front(32);
    list.push_front(34);
    list.push_front(36);

    println!("before pop_front => {:?}", list.values());
    println!("pop_front => {:?}", list.pop_front());
    println!("after pop_front => {:?}", list.values());
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Doubly Link List (Part 2)
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

    // Case: 1
    // -----------------------
    //         Head        Tail
    // None <-- 1 --> 2 --> 3 --> None
    // None     1 <-- 2 <-- 3     None
    // -----------------------

    // Case: 1 (After Removal)
    // -----------------------
    //       Head  Tail
    // None <-- 2 --> 3 --> None
    // None     2 <-- 3     None
    // -----------------------

    // Case: 2
    // -----------------------
    //       Head
    //       Tail
    // None <-- 1 --> None
    // -----------------------

    // Case: 2 (After Removal)
    // -----------------------
    //       Head = None
    //       Tail = None
    // -----------------------

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so we can not remove");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}
fn main() {
    let mut list1 = Doubly_Linklist::new();

    list1.add(30);
    list1.add(32);
    list1.add(34);
    list1.add(36);
    list1.print();

    list1.remove();
    println!("After Removal");
    list1.print();
}

/*
------------------------------------------------------------------------------------------------
Concept / Topic         | Explanation
------------------------|-----------------------------------------------------------------------
Removing from front     | Following scenarios must be considered when removing the head node.
                        | If the list has multiple nodes, the next of head becomes the new head.
                        | Additionally, the previous pointer of the new head is updated to None.
                        | If the list contains only one node, both head and tail become None.
                        | Finally, if the list empty, we simply returns none

Using take() and map()  | The take() method temporarily removes the head and replaces it with None.
                        | The removed head is processed using the map() function.
                        | The closure inside map transforms the old head into the new head pointer.

Traversing and Printing | A print method is implemented to traverse the list from the head node.
                        | A traversal pointer iterates while the node reference is not None.
                        | The borrow() function is used to temporarily access the node.
                        | During each iteration, the element is printed and traversal moves to next node.
--------------------------------------------------------------------------------------------------
*/
"###;
