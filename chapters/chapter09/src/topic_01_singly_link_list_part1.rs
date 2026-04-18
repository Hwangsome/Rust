//! 单向链表（第 1 部分）：搭起递归的数据结构。
//!
//! 为什么需要 `Box`？因为 `Node` 的 `next` 字段指向**另一个 `Node`**，
//! 如果直接写 `next: Node`，编译器要计算 `Node` 的大小：需要 `Node` 的大小 = i32 + `Node` 的大小 = ...
//! —— 无限递归，无法确定。所以必须用 `Box<Node>` 把下一节点放到堆上，当前节点只保留一个指针大小。
//!
//! 整体形状：
//!
//! ```text
//! LinkedList { head } → Node{val, next} → Node{val, next} → None
//! ```
//!
//! `Option<Box<Node>>` 意思是："要么有下一个节点（指针在堆上），要么就是链表终点（None）"。
//! 这个类型别名我们起名 `Link` 让代码更好读。

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Link List (Part 1)
// -------------------------------------------

#[derive(Debug)]
struct Linklist {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;
fn main() {
    // let list = Node {
    //     element: 1,
    //     next: None,
    // };

    // let list = Node {
    //     element: 1,
    //     next: Some(Box::new(Node {
    //         element: 2,
    //         next: None,
    //     })),
    // };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: None,
    //     }),
    // };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: Some(Box::new(Node {
    //             element: 2,
    //             next: Some(Box::new(Node {
    //                 element: 3,
    //                 next: None,
    //             })),
    //         })),
    //     }),
    // };

    // let list = Linklist { head: None };

    let list = Linklist {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("{:?}", &list.head);
}


/* 
-----------------------------------------------------------------------------------------------------
Concept / Topic           | Explanation
--------------------------|--------------------------------------------------------------------------
Linked List Basics        | A linked list is used to organize and store data as a sequence of nodes.
                          | Each node contains a value and a reference pointing to the next node.
                          | The first node is called the head and the last node is called the tail.
                          | The tail node points to nothing, marking the end of the list.

Node Structure Design     | A node struct stores the element value and a link to next node as fields.
                          | The next field is defined as an optional pointer to another node.

Head Wrapper Structure    | It allows explicitly storing the starting or head node.
                          | By making the head field as Option<Node>, it allows for an empty list. 
                          
Improving Type Readability| Recursive pointer types using Option and Box can become difficult to read.
                          | A custom pointer type alias is introduced to simplify these types.
                          | This improves readability and keeps struct definitions cleaner.
-----------------------------------------------------------------------------------------------------
*/
"###;
