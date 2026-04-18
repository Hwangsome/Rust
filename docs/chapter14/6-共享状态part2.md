# 6. 共享状态 (part 2)：`RwLock` + `Atomic*` + 死锁

## 对应代码

- [topic_06_sharing_states_part_2.rs](../../chapters/chapter14/src/topic_06_sharing_states_part_2.rs)

## 选型

| 场景 | 推荐 |
|-----|-----|
| 频繁读、少写 | `RwLock<T>` |
| 简单计数 / flag | `AtomicUsize` / `AtomicBool` |
| 任意可变状态 | `Mutex<T>` |

## 死锁经验

- 始终**以相同顺序**获取多把锁
- 持锁时间短
- 持锁时不要调用可能再拿锁的代码
