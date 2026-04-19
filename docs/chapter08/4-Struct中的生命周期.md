# 4. Struct 中的生命周期

> - **所属章节**：第 8 章 · Memory Management Features
> - **Cargo package**：`chapter08`
> - **运行方式**：`cargo run -p chapter08`
> - **代码位置**：`chapters/chapter08/src/topic_04_lifetimes_in_structs.rs`
> - **上一篇**：[3. 生命周期省略规则](./3-生命周期省略.md)
> - **下一篇**：[5. Box 智能指针](./5-Box智能指针.md)
> - **关键词**：struct lifetime、`struct Foo<'a>`、引用字段、借用的 struct

---

## 这一节解决什么问题

当 struct 的字段是引用时，编译器需要知道：这个 struct 实例的生命周期和它借用的数据之间是什么关系？

```rust
struct Excerpt {
    part: &str,  // ❌ 编译错误！需要生命周期标注
}
```

如果没有生命周期标注，编译器无法验证 `Excerpt` 实例是否比它借用的数据活得更长。

---

## 一分钟结论

- 包含引用字段的 struct 必须声明生命周期参数：`struct Foo<'a>`
- 含义：`Foo<'a>` 的实例不能比 `'a` 活得更长
- `impl<'a>` 块需要重复声明生命周期参数
- 方法可以用 `'a` 或引入新的生命周期参数
- 通常 struct 拥有自己的数据更简单（`String` 而不是 `&str`）；借用只在性能关键路径上使用

---

## 详细原理

### 1. 基础语法

```rust
// struct 借用了 'a 生命周期的字符串数据
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

// impl 块也需要声明 'a
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 { 3 }

    // 省略规则 3：返回值生命周期 = &self
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {announcement}");
        self.part
    }
}
```

### 2. 生命周期约束

```rust
let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence;

{
    let i = novel.find('.').unwrap_or(novel.len());
    first_sentence = &novel[..i];
}

let excerpt = Excerpt { part: first_sentence };
println!("{:?}", excerpt);
// ✅ excerpt.part 指向 novel，novel 仍然有效
```

### 3. 多个生命周期参数

```rust
struct MultiRef<'a, 'b> {
    primary: &'a str,
    secondary: &'b str,
}

impl<'a, 'b> MultiRef<'a, 'b> {
    fn primary(&self) -> &'a str { self.primary }
    fn secondary(&self) -> &'b str { self.secondary }
}
```

---

## 完整运行示例

```rust
#[derive(Debug)]
struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn first_word(&self) -> &'a str {
        self.text.split_whitespace().next().unwrap_or("")
    }

    fn words_longer_than(&self, n: usize) -> Vec<&'a str> {
        self.text.split_whitespace()
            .filter(|w| w.len() > n)
            .collect()
    }
}

fn main() {
    let text = String::from("the quick brown fox jumps over the lazy dog");
    let analyzer = TextAnalyzer::new(&text);

    println!("文章: '{}'", analyzer.text);
    println!("单词数: {}", analyzer.word_count());
    println!("第一个词: '{}'", analyzer.first_word());
    println!("长于3字符: {:?}", analyzer.words_longer_than(3));

    // analyzer 和 text 的生命周期绑定：text 必须活得不短于 analyzer
    drop(analyzer); // analyzer 先 drop 也没问题
    println!("text 仍然有效: '{text}'");
}
```

---

## 何时用 `&'a T` 字段 vs 拥有型字段

```
使用引用字段（&'a T）：
  ✅ 避免克隆大型数据（性能关键路径）
  ✅ 只需要临时借用，不需要拥有
  ⚠️ 生命周期约束使代码更复杂

使用拥有型字段（T，如 String）：
  ✅ 没有生命周期约束
  ✅ struct 可以独立存在，不依赖外部数据
  ⚠️ 需要克隆数据（有复制成本）

原则：优先使用拥有型，只在需要优化时考虑引用字段
```

---

## 下一步

- 继续阅读：[5. Box 智能指针](./5-Box智能指针.md)
- 回到目录：[第 8 章：内存管理特性](./README.md)
