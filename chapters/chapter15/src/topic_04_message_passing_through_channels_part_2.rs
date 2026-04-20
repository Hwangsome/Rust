//! 消息传递（part 2）：多生产者 / 同步通道 / try_recv。
//!
//! - `tx.clone()` → 创建**另一个发送端**。所有 tx 全部被 drop 后 rx 收到关闭信号
//! - `mpsc::sync_channel(n)`：带有缓冲区容量的**同步通道**（缓冲满时 send 会阻塞）
//! - `rx.try_recv()`：非阻塞接收，立刻返回 `Err(TryRecvError)` 当前没消息

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("== Channels (part 2): multi-producer ==");

    println!("-- (1) tx.clone() 多生产者 --");
    let (tx, rx) = mpsc::channel::<(u32, String)>();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    drop(tx); // 原 tx drop 掉，只留两个 clone

    let p1 = thread::spawn(move || {
        for i in 1..=3 {
            tx1.send((1, format!("p1-{i}"))).ok();
        }
    });
    let p2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send((2, format!("p2-{i}"))).ok();
        }
    });

    p1.join().ok();
    p2.join().ok();

    // 收集所有消息（此时所有 tx 已 drop，rx 的 for 会自然结束）
    let mut msgs: Vec<_> = rx.into_iter().collect();
    msgs.sort(); // 稳定顺序便于观察
    for m in &msgs {
        println!("  got {m:?}");
    }
    println!();

    println!("-- (2) sync_channel 带缓冲 --");
    let (stx, srx) = mpsc::sync_channel::<i32>(2); // 缓冲 2 个
    let sp = thread::spawn(move || {
        for i in 1..=5 {
            stx.send(i).ok();
            println!("  sent {i}");
        }
    });
    thread::sleep(Duration::from_millis(10));
    for v in srx {
        println!("  recv {v}");
    }
    sp.join().ok();
    println!();
}
