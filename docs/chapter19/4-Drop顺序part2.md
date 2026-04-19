# 4. Drop 顺序（Part 2）：struct 字段

> - **所属章节**：第 19 章 · Dropcheck
> - **代码位置**：`chapters/chapter19/src/topic_04_drop_order_part_2.rs`
> - **上一篇**：[3. Drop 顺序（part 1）](./3-Drop顺序part1.md)
> - **下一篇**：[5. Drop 顺序与自引用](./5-Drop顺序与自引用.md)

---

## struct 字段：按声明顺序 drop

```rust
struct Tracker(&'static str);
impl Drop for Tracker {
    fn drop(&mut self) { println!("drop field: {}", self.0); }
}

struct Holder {
    first: Tracker,   // 声明顺序: first, second, third
    second: Tracker,
    third: Tracker,
}

let _h = Holder {
    first: Tracker("first"),
    second: Tracker("second"),
    third: Tracker("third"),
};
// drop 顺序: first → second → third（声明顺序，不是 LIFO！）
```

注意：**局部变量**是 LIFO（逆序），而 **struct 字段**是按声明顺序 drop。

---

## 下一步

- 继续阅读：[5. Drop 顺序与自引用](./5-Drop顺序与自引用.md)
