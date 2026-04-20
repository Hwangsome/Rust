# 6. Box 的典型用法与 Deref 强转

> - **所属章节**：第 9 章 · Memory Management Features
> - **Cargo package**：`chapter09`
> - **运行方式**：`cargo run -p chapter09`
> - **代码位置**：`chapters/chapter09/src/topic_06_box_pointer_usecases.rs`
> - **上一篇**：[5. Box<T> 智能指针](./5-Box智能指针.md)
> - **下一篇**：[7. Rc 智能指针](./7-Rc智能指针.md)
> - **关键词**：`Box<dyn Trait>`、Trait Object、`Box<dyn Error>`、插件模式、Deref coercion

---

## 这一节解决什么问题

上一篇讲了 `Box` 的基础（堆分配、递归类型）。这一篇专注工程场景：**在实际项目里，什么情况下会用到 `Box`？**

最常见的三个场景：

1. **`Box<dyn Trait>`**：存放不同具体类型的集合（插件系统、事件处理）
2. **`Box<dyn Error>`**：函数返回任意错误类型
3. **通过 Deref 透明访问**：`Box<T>` 可以像 `T` 一样直接调用方法

---

## 详细原理

### 1. `Box<dyn Trait>`：异构集合

```rust
trait Renderer {
    fn render(&self) -> String;
    fn priority(&self) -> u8 { 0 }
}

struct HtmlRenderer;
struct JsonRenderer { indent: usize }
struct PlainTextRenderer { prefix: String }

impl Renderer for HtmlRenderer {
    fn render(&self) -> String { "<html>content</html>".into() }
    fn priority(&self) -> u8 { 10 }
}

impl Renderer for JsonRenderer {
    fn render(&self) -> String {
        format!("{{\n{}\"content\": \"data\"\n}}", " ".repeat(self.indent))
    }
}

impl Renderer for PlainTextRenderer {
    fn render(&self) -> String { format!("{} content", self.prefix) }
}

// 存放不同渲染器，统一接口
let mut renderers: Vec<Box<dyn Renderer>> = vec![
    Box::new(HtmlRenderer),
    Box::new(JsonRenderer { indent: 2 }),
    Box::new(PlainTextRenderer { prefix: "[TXT]".into() }),
];

// 按优先级排序
renderers.sort_by_key(|r| std::cmp::Reverse(r.priority()));

for renderer in &renderers {
    println!("{}", renderer.render());
}
```

### 2. `Box<dyn Error>`：统一错误类型

```rust
use std::num::ParseIntError;

fn parse_config(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("配置不能为空".into());  // &str → Box<dyn Error>
    }
    let n = trimmed.parse::<i32>()?;  // ParseIntError → Box<dyn Error>
    if n < 0 {
        return Err(format!("值 {} 不能为负", n).into());
    }
    Ok(n)
}

for input in ["42", "  -5  ", "abc", "", "100"] {
    match parse_config(input) {
        Ok(n) => println!("✅ {input:?} → {n}"),
        Err(e) => println!("❌ {input:?} → {e}"),
    }
}
```

### 3. Deref 强转：`Box<T>` 像 `T` 一样使用

```rust
let boxed_str: Box<str> = "hello".into();
let boxed_string: Box<String> = Box::new(String::from("world"));

// Box<T> 通过 Deref trait 自动解引用
fn accept_str(s: &str) { println!("{s}"); }

accept_str(&boxed_str);     // Box<str> → &str
accept_str(&boxed_string);  // Box<String> → &String → &str（两层）
println!("{}", boxed_string.to_uppercase()); // 方法调用自动解引用
```

---

## 完整运行示例

```rust
// 插件系统示例
trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct UpperPlugin;
struct ReversePlugin;
struct RepeatPlugin { times: usize }

impl Plugin for UpperPlugin {
    fn name(&self) -> &str { "upper" }
    fn execute(&self, input: &str) -> String { input.to_uppercase() }
}

impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn execute(&self, input: &str) -> String { input.chars().rev().collect() }
}

impl Plugin for RepeatPlugin {
    fn name(&self) -> &str { "repeat" }
    fn execute(&self, input: &str) -> String { input.repeat(self.times) }
}

struct PluginPipeline {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginPipeline {
    fn new() -> Self { PluginPipeline { plugins: Vec::new() } }

    fn add(&mut self, plugin: impl Plugin + Send + Sync + 'static) {
        self.plugins.push(Box::new(plugin));
    }

    fn run(&self, input: &str) -> String {
        self.plugins.iter().fold(input.to_string(), |acc, p| p.execute(&acc))
    }
}

fn main() {
    let mut pipeline = PluginPipeline::new();
    pipeline.add(UpperPlugin);
    pipeline.add(RepeatPlugin { times: 2 });
    pipeline.add(ReversePlugin);

    let result = pipeline.run("hello");
    println!("结果: {result}");  // "HELLOHELLO" 然后翻转
    println!();

    // Box<dyn Error>
    println!("=== Box<dyn Error> ===");
    fn may_fail(n: i32) -> Result<i32, Box<dyn std::error::Error>> {
        if n == 0 { return Err("除以零".into()); }
        Ok(100 / n)
    }

    for n in [5, 0, -2] {
        match may_fail(n) {
            Ok(v) => println!("  {n} → {v}"),
            Err(e) => println!("  {n} → 错误: {e}"),
        }
    }
}
```

---

## 性能说明

`Box<dyn Trait>` 每次方法调用比泛型多一次 vtable 查找（~2-4ns）。对大多数应用来说可以忽略。只有在极其热的路径上才需要考虑用枚举替代。

---

## 下一步

- 继续阅读：[7. Rc 智能指针](./7-Rc智能指针.md)
- 回到目录：[第 9 章：内存管理特性](./README.md)
