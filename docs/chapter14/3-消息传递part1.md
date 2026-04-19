# 3. 消息传递（Part 1）：mpsc channel 基础

> - **所属章节**：第 14 章 · Concurrency
> - **代码位置**：`chapters/chapter14/src/topic_03_message_passing_through_channels_part_1.rs`
> - **上一篇**：[2. 线程中的所有权](./2-线程中的所有权.md)
> - **下一篇**：[4. 消息传递（part 2）](./4-消息传递part2.md)
> - **关键词**：`mpsc`、channel、`Sender`、`Receiver`、`send`、`recv`

---

## "通过通信来共享内存"

Rust 的标准库提供 `std::sync::mpsc`（多生产者单消费者）channel：

```
mpsc = Multiple Producer, Single Consumer
       多生产者,       单消费者

tx (Sender)  ──→  channel buffer  ──→  rx (Receiver)
     可以 clone        消息按序传递         只能有一个
```

---

## 基础 API

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<String>();

// 生产者线程
thread::spawn(move || {
    tx.send("hello".to_string()).unwrap();
    tx.send("world".to_string()).unwrap();
    // tx drop 后，rx 收到 channel closed 信号
});

// 消费者（主线程）
for msg in rx {  // rx 作为迭代器，自动等待直到 channel 关闭
    println!("收到: {msg}");
}
```

---

## 完整运行示例

```rust
use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("=== mpsc channel ===");
    let (tx, rx) = mpsc::channel::<i32>();

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        println!("  生产者完成");
    });

    let mut received = Vec::new();
    for msg in rx {
        received.push(msg);
    }

    producer.join().unwrap();
    println!("  消费者收到: {received:?}");
    println!("  总和: {}", received.iter().sum::<i32>());
}
```

---

## 下一步

- 继续阅读：[4. 消息传递（part 2）](./4-消息传递part2.md)
