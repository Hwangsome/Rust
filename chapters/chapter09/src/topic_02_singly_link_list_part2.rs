// 这个文件在链表结构上补最小操作：头插和头删。
// 运行时要观察：`take()` 会把 `self.head` 暂时替换成 `None`，从而安全地把旧头拿出来。
// 这是链表实现里非常常见的所有权技巧。
type Link = Option<Box<Node>>;

#[derive(Debug, Default)]
struct LinkedList {
    head: Link,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Link,
}

impl LinkedList {
    fn new() -> Self {
        Self::default()
    }

    fn push_front(&mut self, element: i32) {
        let previous_head = self.head.take();
        self.head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            old_head.element
        })
    }

    fn values(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = &self.head;

        while let Some(node) = current {
            values.push(node.element);
            current = &node.next;
        }

        values
    }
}

pub fn run() {
    println!("== Singly Linked List (Part 2) ==");

    let mut list = LinkedList::new();
    list.push_front(5);
    list.push_front(7);
    list.push_front(10);

    println!("after push_front => {:?}", list.values());
    println!("pop_front => {:?}", list.pop_front());
    println!("after pop_front => {:?}", list.values());
    println!();
}
