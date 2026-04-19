# 6. Unsized Coercion（再访）

> - **所属章节**：第 13 章 · Coercion in Rust
> - **代码位置**：`chapters/chapter13/src/topic_06_unsized_coercion.rs`
> - **上一篇**：[5. Trait Object 强转](./5-Trait对象强转.md)
> - **下一篇**：[7. 泛型与 Coercion](./7-泛型与Coercion.md)

---

## Unsized Coercion 的完整形式

在 Coercion 的大框架下，Unsized Coercion 是其中一类：把 Sized 类型的引用/Box 转成 DST 的胖指针。

常见形式：

| 原类型 | 目标类型 | 发生条件 |
|-------|---------|--------|
| `&[T; N]` | `&[T]` | 数组 → 切片 |
| `Box<[T; N]>` | `Box<[T]>` | 同上，堆上 |
| `&T`（T: Trait）| `&dyn Trait` | 具体类型 → 对象 |
| `Box<T>`（T: Trait）| `Box<dyn Trait>` | 同上 |

---

## 运行示例

```rust
fn sum(s: &[i32]) -> i32 { s.iter().sum() }

let arr = [1, 2, 3, 4, 5];
let v = vec![6, 7, 8];
let boxed: Box<[i32; 3]> = Box::new([10, 20, 30]);
let boxed_slice: Box<[i32]> = boxed; // 强转为 Box<[i32]>

println!("{}", sum(&arr));           // &[i32; 5] → &[i32]
println!("{}", sum(&v));             // &Vec<i32> → &[i32]（通过 Deref）
println!("{:?}", boxed_slice);       // Box<[i32]> 可以用
```

---

## 下一步

- 继续阅读：[7. 泛型与 Coercion](./7-泛型与Coercion.md)
