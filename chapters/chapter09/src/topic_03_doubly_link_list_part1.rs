// 这个文件演示双向链表为什么会立刻引出 `Rc<RefCell<T>>`。
// 运行时要观察：同一个节点既要被链表拥有，又要被前后节点互相指向，还要允许修改 next / prev。
// 这三个要求叠在一起，正是 Rc 和 RefCell 组合登场的原因。
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
