# 4. 消息传递 (part 2)：多生产者 / sync_channel / try_recv

- `tx.clone()` 创建另一个发送端
- `mpsc::sync_channel(cap)` 给 channel 加缓冲上限——满了 send 阻塞
- `rx.try_recv()` 非阻塞接收

## 对应代码

- [topic_04_message_passing_through_channels_part_2.rs](../../chapters/chapter14/src/topic_04_message_passing_through_channels_part_2.rs)

## 典型场景

背压（back-pressure）：消费者慢时让生产者自然减速。
