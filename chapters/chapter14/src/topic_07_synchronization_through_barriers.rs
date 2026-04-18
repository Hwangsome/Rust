//! 同步点：`std::sync::Barrier`——**等所有线程都到这里才继续**。
//!
//! 适合"分段并发：先各自做 A，等全部做完后一起做 B"。

use std::sync::{Arc, Barrier};
use std::thread;

pub fn run() {
    println!("== Barrier ==");

    let n = 3;
    let barrier = Arc::new(Barrier::new(n));
    let mut handles = Vec::new();

    for i in 0..n {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("  [thread {i}] phase-1 done, waiting at barrier");
            b.wait(); // 在这里等所有线程都到达
            println!("  [thread {i}] phase-2 starting");
        }));
    }

    for h in handles { h.join().ok(); }
    println!("  all threads finished phase 2");
    println!();
}
