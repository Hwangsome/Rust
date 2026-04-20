//! 共享可变状态（part 1）：`Arc<Mutex<T>>`。
//!
//! "多个线程都要读写同一份状态"的标准组合：
//!
//! - **`Arc<T>`**：原子引用计数，让多个线程都能 **拥有** 同一个 T
//! - **`Mutex<T>`**：同时只允许一个线程持有可变访问，通过 `.lock()` 获取
//! - 两者组合：`Arc<Mutex<T>>` —— 多个线程共享可变状态的最常见模式
//!
//! 对照 chapter08 的 `Rc<RefCell<T>>`：
//! - 单线程：`Rc<RefCell<T>>`（非原子，更快）
//! - 多线程：`Arc<Mutex<T>>`（原子，线程安全）

use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("== Sharing States (part 1): Arc<Mutex<T>> ==");

    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles = Vec::new();
    for t in 0..5 {
        // 每个线程都 clone 一个 Arc——strong_count 增加
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // .lock() 返回 LockResult<MutexGuard<T>>；unwrap 避免复杂
                let mut guard = c.lock().unwrap();
                *guard += 1;
            }
            println!("  [thread {t}] done");
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().ok();
    }

    let final_value = *counter.lock().unwrap();
    println!("  final counter = {final_value}");
    println!("  Arc::strong_count = {}", Arc::strong_count(&counter));
    println!();
}
