# 1. Coercion 基础与 Deref 强转

> - **所属章节**：第 14 章 · Coercion in Rust
> - **Cargo package**：`chapter15`
> - **运行方式**：`cargo run -p chapter15`
> - **代码位置**：`chapters/chapter15/src/topic_01_coercion_basics_deref_coercion.rs`
> - **上一篇**：本章第一篇
> - **下一篇**：[2. Coercion 触发位置](./2-Coercion触发位置.md)
> - **关键词**：coercion、隐式转换、Deref、`&String → &str`、`&Vec<T> → &[T]`

---

## 什么是 Coercion

Coercion（强转/转型）是编译器在特定位置**自动**把一个类型转换为另一个兼容类型，无需显式 `as`。

最常见的是 **Deref coercion**：

```
&String  →  &str      （String: Deref<Target=str>）
&Vec<T>  →  &[T]      （Vec<T>: Deref<Target=[T]>）
&Box<T>  →  &T        （Box<T>: Deref<Target=T>）
&Rc<T>   →  &T
&Arc<T>  →  &T
```

---

## 为什么有 Coercion

如果没有 Deref coercion，你不得不写：

```rust
fn accept_str(s: &str) { println!("{s}"); }

let owned = String::from("hello");
accept_str(owned.as_str());  // 必须显式转换
accept_str(&owned);          // 有了 Deref coercion：自动！
```

---

## Deref Coercion 链式发生

```rust
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn accept_str(s: &str) { println!("{s}"); }

let mb = MyBox(String::from("hello"));
// &MyBox<String> → &String（通过 MyBox::Deref）
//               → &str   （通过 String::Deref）
// 两层，全自动！
accept_str(&mb);
```

---

## 完整运行示例

```rust
fn accept_str(s: &str) { println!("  str: {s}"); }
fn accept_slice(s: &[i32]) { println!("  slice: {s:?}"); }

pub fn run() {
    println!("=== Deref Coercion ===");

    let owned = String::from("hello");
    let boxed = Box::new(String::from("world"));
    let v = vec![1, 2, 3];

    accept_str("字面量");           // &'static str → &str（trivial）
    accept_str(&owned);             // &String → &str（Deref）
    accept_str(&boxed);             // &Box<String> → &String → &str（两层）

    accept_slice(&v);               // &Vec<i32> → &[i32]（Deref）
    accept_slice(&[10, 20, 30]);   // &[i32; 3] → &[i32]（coerce）
    println!();

    println!("=== 为什么这样设计 ===");
    println!("  API 设计更通用（&str 比 &String 接受更多类型）");
    println!("  调用方不需要手动 .as_str() / .as_slice()");
    println!("  智能指针（Box/Rc/Arc）使用起来和普通引用一样自然");
}
```

---

## 下一步

- 继续阅读：[2. Coercion 触发位置](./2-Coercion触发位置.md)
- 回到目录：[第 14 章：Coercion in Rust](./README.md)
