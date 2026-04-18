# 5. 共享状态 (part 1)：`Arc<Mutex<T>>`

多线程共享可变状态的标准组合：
- `Arc<T>`：原子引用计数，多线程共享所有权
- `Mutex<T>`：同时只允许一个线程可变访问

## 对应代码

- [topic_05_sharing_states_part_1.rs](../../chapters/chapter14/src/topic_05_sharing_states_part_1.rs)

## 对照

| 场景 | 单线程 | 多线程 |
|-----|-------|-------|
| 多所有者 + 可变共享 | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |

## 关键习惯

- `.lock().unwrap()` —— 教学阶段能接受，生产代码考虑 `?` + 自定义错误
- 持锁时间要短；避免 await 时持锁
