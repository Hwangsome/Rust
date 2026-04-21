# 3. Drop 顺序（Part 1）：局部变量

> - **所属章节**：第 20 章 · Dropcheck
> - **代码位置**：`chapters/chapter21/src/topic_03_drop_order_part_1.rs`
> - **上一篇**：[2. Drop 与部分 Move](./2-Drop与部分move.md)
> - **下一篇**：[4. Drop 顺序（part 2）](./4-Drop顺序part2.md)

---

## 局部变量：后声明先 drop（LIFO）

```rust
struct Tracker(&'static str);
impl Drop for Tracker {
    fn drop(&mut self) { println!("drop: {}", self.0); }
}

fn example() {
    let a = Tracker("a");  // 声明顺序: a, b, c
    let b = Tracker("b");
    let c = Tracker("c");
    println!("before drop");
} // drop 顺序: c, b, a（声明的逆序）

// 输出：
// before drop
// drop: c
// drop: b
// drop: a
```

---

## 显式 drop

```rust
let a = Tracker("a");
let b = Tracker("b");
drop(a);  // 立刻 drop a（不等到作用域结束）
// a 不能再用了
println!("b 仍然有效");
// b 在作用域结束时 drop
```

---

## 下一步

- 继续阅读：[4. Drop 顺序（part 2）](./4-Drop顺序part2.md)
