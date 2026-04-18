# 3. 消息传递 (part 1)：`mpsc` channel 基础

"不要通过共享内存来通信，要通过通信来共享内存。"

## 对应代码

- [topic_03_message_passing_through_channels_part_1.rs](../../chapters/chapter14/src/topic_03_message_passing_through_channels_part_1.rs)

## 核心 API

```rust
let (tx, rx) = mpsc::channel::<T>();
tx.send(v)?;       // 生产者
for v in rx {...}  // 消费者
```

所有 tx 被 drop 后，`for v in rx` 自然结束。
