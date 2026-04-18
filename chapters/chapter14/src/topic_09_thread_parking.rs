//! 线程停放（parking）：`thread::park` / `thread::Thread::unpark`。
//!
//! 比 channel / mutex 更底层的同步原语——让一个线程**主动暂停**，等另一个线程唤醒它。
//! 常用于实现自己的同步工具。

use std::thread;
use std::time::Duration;

pub fn run() {
    println!("== Thread Parking ==");

    let parked = thread::spawn(|| {
        println!("  [parked] will park now");
        thread::park(); // 阻塞直到被 unpark
        println!("  [parked] resumed after unpark");
    });

    thread::sleep(Duration::from_millis(30));

    println!("  [main] calling unpark on parked thread");
    parked.thread().unpark();

    parked.join().ok();
    println!();
}
