# 2. Coercion 触发位置（Coercion Sites）

> - **所属章节**：第 14 章 · Coercion in Rust
> - **Cargo package**：`chapter15`
> - **代码位置**：`chapters/chapter15/src/topic_02_coercion_sites.rs`
> - **上一篇**：[1. Coercion 基础](./1-Coercion基础.md)
> - **下一篇**：[3. 引用强转](./3-引用强转.md)
> - **关键词**：coercion site、函数参数、let 标注、return、struct 字段

---

## Coercion 只在特定位置发生

不是所有地方都会自动触发 coercion，只有这些"coercion site"：

1. **函数/方法参数位置**：`f(&my_string)` 中的 `&my_string`
2. **`let` 带类型标注**：`let s: &str = &my_string;`
3. **`return` 表达式**：函数返回类型和表达式不完全一样时
4. **struct/enum 字段初始化**：字段类型和值类型不完全一样时
5. **方法调用接收者**：`my_string.len()` 自动 deref

---

## 完整运行示例

```rust
fn needs_str(s: &str) -> usize { s.len() }

struct Wrapper { text: String }

pub fn run() {
    println!("=== Coercion Sites ===");

    let s = String::from("hello");

    // 1. 函数参数
    println!("函数参数: {}", needs_str(&s));

    // 2. let 类型标注
    let as_str: &str = &s;
    println!("let 标注: {as_str}");

    // 3. struct 字段（Wrapper.text 是 String，可以从 &str 初始化吗？）
    // 注意：这里不是 coerce，因为 String 和 &str 不是 deref 关系
    // let w = Wrapper { text: "hi" }; // ❌ 必须 String::from 或 into()
    let w = Wrapper { text: s.clone() };
    println!("struct 字段: {}", w.text);

    // 4. 方法调用自动 deref
    let boxed = Box::new(String::from("world"));
    println!("自动 deref 调用: {}", boxed.to_uppercase()); // Box→String→str→len
}
```

---

## 不触发 Coercion 的情况

```rust
// 泛型参数位置不触发：
fn f<T: std::fmt::Debug>(x: T) { println!("{x:?}"); }
// f(&String::from("hi")); // T = &String，不会 coerce 成 &str
```

---

## 下一步

- 继续阅读：[3. 引用强转](./3-引用强转.md)
