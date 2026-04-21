//! 消息传递（part 1）：`std::sync::mpsc` 多生产者-单消费者通道。
//!
//! "不要通过共享内存来通信，要通过通信来共享内存"（来自 Go，也被 Rust 广泛采纳）。
//!
//! 基础 API：
//!
//! ```ignore
//! use std::sync::mpsc;
//! let (tx, rx) = mpsc::channel::<T>();
//! tx.send(value)?;              // 生产者
//! for v in rx { ... }          // 消费者（或 rx.recv()）
//! ```

use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("== Channels (part 1): mpsc basics ==");

    let (tx, rx) = mpsc::channel::<String>();

    // 生产者线程
    let producer = thread::spawn(move || {
        for word in ["hello", "from", "thread"] {
            tx.send(word.to_string()).ok();
        }
        // tx drop 后，对应的 rx 会得到 channel closed 的信号
    });

    // 消费者（主线程）用 for 循环消费，直到 tx 被全部 drop
    for msg in rx {
        println!("  received: {msg}");
    }

    producer.join().ok();
    println!();
}
