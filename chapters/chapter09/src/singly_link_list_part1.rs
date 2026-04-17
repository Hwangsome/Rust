// 这个文件只做一件事：把“单向链表的递归结构”搭起来。
// 运行时要观察：节点里必须用 `Option<Box<Node>>` 指向下一个节点，否则类型大小无法确定。
// 这一步还不追求操作方法，只先看清数据长什么样。
type Link = Option<Box<Node>>;

#[derive(Debug)]
struct LinkedList {
    head: Link,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Link,
}

impl LinkedList {
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
    println!("== Singly Linked List (Part 1) ==");

    let list = LinkedList {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("list head => {:?}", list.head);
    println!("list values => {:?}", list.values());
    println!();
}
