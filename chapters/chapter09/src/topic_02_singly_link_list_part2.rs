//! 单向链表（第 2 部分）：头插 `push_front` 与头删 `pop_front`。
//!
//! 实现链表时最容易踩的坑是**所有权转移**：改 `self.head` 时，老的 head 必须"先拿出来"再"放新值"。
//! `Option::take()` 就是给这种"临时取出、替换为 None"场景准备的：
//!
//! ```text
//! self.head.take() ≡ std::mem::replace(&mut self.head, None)
//! ```
//!
//! 这让我们能够安全地从 `Option<Box<Node>>` 里夺走旧节点，放进新节点的 next 字段，再把新节点赋回 head。
//! 这个模式在标准库 `VecDeque`、`BinaryHeap` 内部也反复使用。

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Link List (Part 2)
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

impl Linklist {
    fn new() -> Linklist {
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     Some(previous_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //         self.head = new_node;
        //     }
        // }

        // fn take<T>(dest: &mut T) -> T
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}
fn main() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    //println!("List: {:?}", list);
    list.print();
    println!("{}", list.remove().unwrap());
}


/* 
---------------------------------------------------------------------------------------------------------
Concept / Topic         | Explanation
------------------------|--------------------------------------------------------------------------------
Constructor (new)       | A constructor method new() is defined to initialize an empty linked list.
                        | It returns a LinkList instance with the head field set to None.

Adding Elements (Push)  | This method will add a new node at the start of the list. 
                        | The newly added node becomes the head and its next field points to previous head.

Using take()            | Directly moving value out of self.head is not allowed with a mutable reference.
                        | The take() method replaces the head with None and returns the previous value.
                        | This temporarily removes the head while keeping the list structure valid.

Removing Elements (Pop) | This method deletes the element at the beginning of the list.
                        | The head is removed using take(), and the next of head becomes the new head.
                        | The removed node’s element is returned.

Printing the List       | A print method traverses the list starting from the head reference.
                        | Iteration continues while the traversal pointer is not None.
                        | The as_ref() method is used to access node contents without moving ownership.
---------------------------------------------------------------------------------------------------------
*/
"###;
