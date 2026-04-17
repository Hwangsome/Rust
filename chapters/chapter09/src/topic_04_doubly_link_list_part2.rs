// 这个文件继续补双向链表的头删和遍历。
// 运行时要观察：删除头节点时，不仅要移动 head，还要把新头的 prev 清空。
// 这就是双向结构比单向结构多出来的状态维护成本。
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
