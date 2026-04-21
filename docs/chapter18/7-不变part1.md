# 7. 不变（Invariance）Part 1：`&mut T`

> - **所属章节**：第 17 章 · Variance
> - **代码位置**：`chapters/chapter18/src/topic_07_invariance_part_1.rs`
> - **上一篇**：[6. 逆变](./6-逆变.md)
> - **下一篇**：[8. 不变（part 2）](./8-不变part2.md)

---

## `&mut T` 是不变的

`&mut T` 既可以读 T 又可以写 T。如果它在 T 上是协变的，就会允许把短寿命写进长寿命容器（不安全）；如果是逆变的，就会允许把长寿命当短寿命写（也不安全）。所以 `&mut T` **必须在 T 上不变**。

```rust
fn f<'a, 'b: 'a>(x: &'a mut Vec<&'a str>, y: &'b str) {
    // 这里 'b 必须活得不短于 'a（因为我们要把 y 放进 x）
    x.push(y);
}

let mut v: Vec<&str> = vec![];
let s = String::from("hello");
f(&mut v, &s);
println!("{v:?}");
```

---

## 理解不变的关键

```
协变 + 可写 = 危险！
  如果 Vec<&'long T> 可以协变成 Vec<&'short T>，
  你就可以把短寿命引用 push 进长寿命 Vec，导致悬垂引用。

所以 &mut Vec<T> 对 T 必须不变。
```

---

## 下一步

- 继续阅读：[8. 不变（part 2）](./8-不变part2.md)
