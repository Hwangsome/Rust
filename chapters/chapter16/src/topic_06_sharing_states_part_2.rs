//! 共享可变状态（part 2）：`RwLock`、`Atomic*`、死锁。
//!
//! `Mutex<T>` 虽然简单，但**同时只有一个读者或一个写者**。读多写少的场景下它会拖慢吞吐。
//!
//! 更细粒度的选择：
//!
//! - **`RwLock<T>`**：允许"多个读者 **或** 一个写者"。读多写少时吞吐高
//! - **`AtomicUsize` / `AtomicI32` ...**：对**基本类型**的无锁原子操作（计数器、flag）
//!
//! **死锁**：两个线程各自持有一个锁、又想拿对方的锁——经典教训："**始终以相同顺序**获取多个锁"。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

pub fn run() {
    println!("== Sharing States (part 2): RwLock / Atomic ==");

    println!("-- (1) RwLock：多读者 / 一个写者 --");
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 启动 3 个读线程
    let mut readers = Vec::new();
    for i in 0..3 {
        let d = Arc::clone(&data);
        readers.push(thread::spawn(move || {
            let guard = d.read().unwrap();
            println!("  reader {i} sees {guard:?}");
        }));
    }
    for r in readers { r.join().ok(); }

    // 单独一个写线程
    {
        let d = Arc::clone(&data);
        let w = thread::spawn(move || {
            let mut g = d.write().unwrap();
            g.push(4);
            println!("  writer pushed 4, now {g:?}");
        });
        w.join().ok();
    }
    println!();

    println!("-- (2) Atomic：无锁计数 --");
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                c.fetch_add(1, Ordering::Relaxed); // 无需锁
            }
        }));
    }
    for h in handles { h.join().ok(); }
    println!("  atomic counter = {}", counter.load(Ordering::Relaxed));
    println!();

    println!("-- (3) 死锁提示 --");
    println!("  多把锁时永远按相同顺序获取；");
    println!("  持锁时间要短；");
    println!("  不在持锁时调用可能再拿锁的代码");
    println!();
}
