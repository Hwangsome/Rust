# 4. 消息传递（Part 2）：多生产者与同步 channel

> - **所属章节**：第 15 章 · Concurrency
> - **代码位置**：`chapters/chapter15/src/topic_04_message_passing_through_channels_part_2.rs`
> - **上一篇**：[3. 消息传递（part 1）](./3-消息传递part1.md)
> - **下一篇**：[5. 共享状态（part 1）](./5-共享状态part1.md)
> - **关键词**：`tx.clone()`、多生产者、`sync_channel`、有界 channel、背压

---

## 多生产者：`tx.clone()`

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<(usize, i32)>();

// 3 个生产者线程
let handles: Vec<_> = (0..3).map(|id| {
    let tx_clone = tx.clone(); // 每个线程有自己的 Sender
    thread::spawn(move || {
        for i in 0..3 {
            tx_clone.send((id, i * 10)).unwrap();
        }
    })
}).collect();

drop(tx); // 必须 drop 原 tx，否则 rx 永远不会知道 channel 已关闭

for msg in rx {
    println!("收到: {:?}", msg);
}

for h in handles { h.join().unwrap(); }
```

---

## 同步 channel（有界缓冲）

```rust
// sync_channel(n)：最多缓存 n 条消息
// 缓冲满时 send() 会阻塞，自然产生背压
let (tx, rx) = mpsc::sync_channel::<i32>(2);

thread::spawn(move || {
    for i in 1..=5 {
        println!("发送 {i}");
        tx.send(i).unwrap(); // 当缓冲满时，这里阻塞
    }
});

for msg in rx {
    std::thread::sleep(std::time::Duration::from_millis(50));
    println!("处理 {msg}");
}
```

---

## 下一步

- 继续阅读：[5. 共享状态（part 1）](./5-共享状态part1.md)
