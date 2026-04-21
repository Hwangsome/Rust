# 8. Coercion 的传递性

> - **所属章节**：第 14 章 · Coercion in Rust
> - **代码位置**：`chapters/chapter15/src/topic_08_transitivity_in_coercion.rs`
> - **上一篇**：[7. 泛型与 Coercion](./7-泛型与Coercion.md)
> - **下一篇**：[9. LUB 强转](./9-LUB强转.md)

---

## Coercion 可以链式发生

```rust
fn accepts_str(s: &str) { println!("{s}"); }

let boxed = Box::new(String::from("hello"));
// Box<String> → String（Deref）→ str（Deref）
// &Box<String> → &String → &str（三层，全自动！）
accepts_str(&boxed);

let v = vec![1, 2, 3];
let boxed_vec = Box::new(v);
fn sum(s: &[i32]) -> i32 { s.iter().sum() }
// Box<Vec<i32>> → Vec<i32> → [i32]
sum(&boxed_vec); // ✅
```

---

## 规律

链式 coercion 的规则：**每一步都必须是合法的单步 coercion**。  
不能跨越两层"壳"同时转换。

---

## 下一步

- 继续阅读：[9. LUB 强转](./9-LUB强转.md)
