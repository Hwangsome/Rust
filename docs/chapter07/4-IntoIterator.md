# 4. IntoIterator：`for` 循环背后的机制

> - **所属章节**：第 7 章 · Functional Programming Aspects
> - **Cargo package**：`chapter07`
> - **运行方式**：`cargo run -p chapter07`
> - **代码位置**：`chapters/chapter07/src/topic_04_into_iter.rs`
> - **上一篇**：[3. 迭代器](./3-迭代器.md)
> - **下一篇**：[5. 遍历集合](./5-遍历集合.md)
> - **关键词**：`IntoIterator`、`into_iter()`、`for` 循环脱糖、所有权转移

---

## 这一节解决什么问题

`for x in collection` 为什么能工作？`collection` 不是迭代器，但 `for` 循环能用它——这背后是 `IntoIterator` trait 在做转换。

`IntoIterator` 回答了"如何把这个类型变成迭代器"的问题，而 `Iterator` 回答"如何从迭代器里取下一个元素"。

---

## 一分钟结论

- `for x in collection` 等价于 `for x in IntoIterator::into_iter(collection)`
- `Vec<T>` 实现了三种 `IntoIterator`：`Vec<T>`（消耗）、`&Vec<T>`（只读借用）、`&mut Vec<T>`（可变借用）
- `for x in v`：消耗 `v`，x 是 `T`
- `for x in &v`：借用 `v`，x 是 `&T`
- `for x in &mut v`：可变借用 `v`，x 是 `&mut T`
- 自定义类型实现 `IntoIterator` 可以让它支持 `for` 循环

---

## 与其他语言对比

| 语言 | `for` 循环背后的机制 |
|-----|-----------------|
| Java | `Iterable<T>` 接口的 `iterator()` 方法 |
| Python | `__iter__` 和 `__next__` 协议 |
| Go | 内置 `range`，无自定义扩展 |
| C++ | `begin()` / `end()` 方法 |
| Rust | `IntoIterator` trait 的 `into_iter()` 方法 |

---

## 详细原理

### for 循环的脱糖过程

```rust
let v = vec![1, 2, 3];

// 你写的
for x in &v {
    println!("{x}");
}

// 编译器理解为（脱糖后）
{
    let mut iter = (&v).into_iter();  // &Vec<i32> → 调用 into_iter()
    loop {
        match iter.next() {
            Some(x) => {
                println!("{x}");
            }
            None => break,
        }
    }
}
```

### 三种 for 写法的语义

```rust
let v = vec![String::from("a"), String::from("b")];

// 消耗：for x in v
for x in v {  // v 被 move 进 into_iter()
    // x: String（拥有所有权）
}
// println!("{v:?}"); // ❌ v 已被消耗

let v = vec![String::from("a"), String::from("b")];

// 只读借用：for x in &v
for x in &v {  // (&v).into_iter()
    // x: &String（只读引用）
}
println!("{v:?}"); // ✅ v 仍然有效

// 可变借用：for x in &mut v
let mut v = vec![1, 2, 3];
for x in &mut v {  // (&mut v).into_iter()
    // x: &mut i32
    *x *= 2;  // 可以修改
}
println!("{v:?}"); // [2, 4, 6]，v 仍然有效
```

### 自定义 IntoIterator

```rust
struct NumberRange {
    start: i32,
    end: i32,
}

struct NumberRangeIter {
    current: i32,
    end: i32,
}

impl Iterator for NumberRangeIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.current < self.end {
            let val = self.current;
            self.current += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl IntoIterator for NumberRange {
    type Item = i32;
    type IntoIter = NumberRangeIter;  // 关联类型：迭代器的具体类型

    fn into_iter(self) -> NumberRangeIter {
        NumberRangeIter { current: self.start, end: self.end }
    }
}

// 现在 NumberRange 支持 for 循环
for n in NumberRange { start: 1, end: 6 } {
    print!("{n} ");  // 1 2 3 4 5
}
```

---

## 完整运行示例

```rust
// 自定义集合
struct Grid {
    data: Vec<Vec<i32>>,
}

struct GridIter {
    data: Vec<Vec<i32>>,
    row: usize,
    col: usize,
}

impl Iterator for GridIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        while self.row < self.data.len() {
            if self.col < self.data[self.row].len() {
                let val = self.data[self.row][self.col];
                self.col += 1;
                return Some(val);
            } else {
                self.row += 1;
                self.col = 0;
            }
        }
        None
    }
}

impl IntoIterator for Grid {
    type Item = i32;
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { data: self.data, row: 0, col: 0 }
    }
}

fn main() {
    println!("=== for 循环三种写法 ===");
    let data = vec![10, 20, 30];

    // 1. 消耗
    let data_move = data.clone();
    for x in data_move {
        print!("{x}(owned) ");
    }
    println!();

    // 2. 只读借用
    for x in &data {
        print!("{x}(&) ");
    }
    println!();
    println!("data 仍然有效: {data:?}");

    // 3. 可变借用
    let mut data_mut = data.clone();
    for x in &mut data_mut {
        *x += 100;
    }
    println!("修改后: {data_mut:?}");
    println!();

    println!("=== 自定义 IntoIterator ===");
    let grid = Grid {
        data: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    };

    print!("所有元素: ");
    for x in grid {
        print!("{x} ");
    }
    println!();

    // 配合迭代器方法
    let grid2 = Grid {
        data: vec![vec![1, 2, 3], vec![4, 5, 6]],
    };
    let sum: i32 = grid2.into_iter().filter(|x| x % 2 == 0).sum();
    println!("偶数之和: {sum}");
}
```

---

## 注意点与陷阱

### 陷阱：数组的 `into_iter()` 和 `iter()` 行为不同

```rust
let arr = [1, 2, 3];

// Rust 2021 之前：arr.into_iter() 产出 &i32（和 arr.iter() 一样）
// Rust 2021 之后：arr.into_iter() 产出 i32（按值，消耗数组）
for x in arr.into_iter() {
    // x 是 i32（Rust 2021 后，数组实现了真正的 IntoIterator）
    println!("{x}");
}

// 要明确借用遍历：
for x in arr.iter() {  // 或者 for x in &arr
    // x 是 &i32
    println!("{x}");
}
```

---

## 下一步

- 继续阅读：[5. 遍历集合](./5-遍历集合.md)
- 回到目录：[第 7 章：函数式编程](./README.md)
