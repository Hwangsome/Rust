# 9. Rc<RefCell<T>>：共享可变状态的标准模式

> - **所属章节**：第 9 章 · Memory Management Features
> - **Cargo package**：`chapter09`
> - **运行方式**：`cargo run -p chapter09`
> - **代码位置**：`chapters/chapter09/src/topic_09_refcell_example.rs`
> - **上一篇**：[8. RefCell 内部可变性](./8-RefCell.md)
> - **下一篇**：本章最后一篇
> - **关键词**：`Rc<RefCell<T>>`、共享可变状态、多所有者+可变、GUI 模式

---

## 这一节解决什么问题

两个独立的问题：

1. `Rc<T>`：多个所有者，但只读
2. `RefCell<T>`：单个所有者，但可以通过 `&T` 修改内部

组合在一起 `Rc<RefCell<T>>`：**多个所有者，且每个都可以修改内部数据**。

这是单线程环境下"共享可变状态"的标准 Rust 解法。

---

## 一分钟结论

- `Rc<RefCell<T>>` = 多所有者（Rc）+ 内部可变性（RefCell）
- 每个 clone 的 Rc 都能通过 `.borrow_mut()` 修改内部
- 修改立刻对所有持有者可见（共享同一块内存）
- 注意循环引用！用 `Weak<T>` 打破
- 多线程场景换成 `Arc<Mutex<T>>`

---

## 完整运行示例

```rust
use std::cell::RefCell;
use std::rc::Rc;

// 多个组件共享同一份应用状态
#[derive(Debug)]
struct AppState {
    user: Option<String>,
    messages: Vec<String>,
    notification_count: u32,
}

impl AppState {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(AppState {
            user: None,
            messages: Vec::new(),
            notification_count: 0,
        }))
    }
}

struct LoginComponent {
    state: Rc<RefCell<AppState>>,
}

impl LoginComponent {
    fn login(&self, username: &str) {
        let mut state = self.state.borrow_mut();
        state.user = Some(username.to_string());
        state.messages.push(format!("{username} 登录了"));
        state.notification_count += 1;
        println!("[Login] {} 已登录", username);
    }
}

struct NotificationComponent {
    state: Rc<RefCell<AppState>>,
}

impl NotificationComponent {
    fn show_notifications(&self) {
        let state = self.state.borrow();
        println!("[通知] 未读 {} 条", state.notification_count);
        for msg in &state.messages {
            println!("  - {msg}");
        }
    }

    fn clear_notifications(&self) {
        let mut state = self.state.borrow_mut();
        state.notification_count = 0;
        println!("[通知] 已清空");
    }
}

fn main() {
    let state = AppState::new();

    // 多个组件共享同一份状态
    let login = LoginComponent { state: Rc::clone(&state) };
    let notifications = NotificationComponent { state: Rc::clone(&state) };

    // 引用计数：3（state + login.state + notifications.state）
    println!("强引用数: {}", Rc::strong_count(&state));

    // 组件之间通过共享状态通信
    notifications.show_notifications();
    login.login("Alice");
    login.login("Bob");
    notifications.show_notifications();
    notifications.clear_notifications();
    notifications.show_notifications();
    println!();

    println!("=== 对比：多线程用 Arc<Mutex<T>> ===");
    use std::sync::{Arc, Mutex};

    let shared = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&shared);
        std::thread::spawn(move || {
            let mut guard = data.lock().unwrap();
            guard.push(i * 10);
        })
    }).collect();

    for h in handles { h.join().ok(); }
    println!("Arc<Mutex<_>> 最终: {:?}", shared.lock().unwrap());
}
```

---

## 选型指南

```
单线程共享可变状态：
  Rc<RefCell<T>>         → 最通用
  Rc<Cell<T>>            → T: Copy 时，更轻量
  Rc<T>（只读共享）

多线程共享可变状态：
  Arc<Mutex<T>>          → 独占写，通用
  Arc<RwLock<T>>         → 读多写少
  Arc<Atomic*>           → 简单计数/标志
```

---

## 下一步

第 9 章完成！你已经掌握了：
- 生命周期（具体、泛型、省略规则、struct 中的）
- 三种智能指针（Box、Rc、RefCell）及其组合

- 回到目录：[第 9 章：内存管理特性](./README.md)
- 下一章：[第 9 章：实现典型数据结构](../chapter09/README.md)
