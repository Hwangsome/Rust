# 7. 泛型与 Coercion：不会自动发生

> - **所属章节**：第 14 章 · Coercion in Rust
> - **代码位置**：`chapters/chapter15/src/topic_07_coercion_in_generics.rs`
> - **上一篇**：[6. Unsized 强转（再访）](./6-unsized强转再访.md)
> - **下一篇**：[8. Coercion 传递性](./8-Coercion传递性.md)

---

## 关键区别

Coercion **不会**在泛型参数的具体类型推断时发生：

```rust
fn foo<T: std::fmt::Debug>(x: T) { println!("{x:?}"); }

let arr: [i32; 3] = [1, 2, 3];
foo(&arr);  // T = &[i32; 3]，不会自动变成 &[i32]
// 但 &arr 实现了 Debug，所以可以打印

// 想让 T = &[i32]，必须显式：
foo(&arr[..]);         // 显式切片
foo(arr.as_slice());   // 或这个方法
```

---

## 用 AsRef / Borrow 接受更多类型

```rust
fn process<T: AsRef<[i32]>>(data: T) {
    let slice: &[i32] = data.as_ref();
    println!("{slice:?}");
}

process([1, 2, 3]);            // [i32; 3]: AsRef<[i32]>
process(vec![4, 5, 6]);       // Vec<i32>: AsRef<[i32]>
process(&[7, 8, 9] as &[i32]); // &[i32]: AsRef<[i32]>
```

---

## 下一步

- 继续阅读：[8. Coercion 传递性](./8-Coercion传递性.md)
