# 2. Builder 模式

> - **所属章节**：第 11 章 · Structuring Projects
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_02_builder_pattern.rs`
> - **上一篇**：[1. 初始化 Struct 实例](./1-初始化Struct实例.md)
> - **下一篇**：[3. 简化大型 Struct](./3-简化大型Struct.md)
> - **关键词**：Builder 模式、`-> Self`、链式调用、可选字段、分阶段构造

---

## 这一节解决什么问题

当 struct 有很多字段（比如 10+ 个），大部分是可选的，但某些组合必须同时存在——这时构造函数参数太多，顺序容易搞错，也不清晰。

Builder 模式解决：用链式方法调用逐步设置字段，最后调用 `build()` 构造最终结果。

---

## Builder 的两种设计风格

### 风格 A：消耗型（返回 Self）

```rust
builder.host("localhost")
       .port(8080)
       .build()
```

优点：链式调用自然。
缺点：每步都消耗 builder，不能复用中间状态。

### 风格 B：借用型（返回 &mut Self）

```rust
let mut b = Builder::new();
b.host("localhost").port(8080);
let result = b.build();
```

优点：可以多次调用或有条件设置字段。
缺点：需要 `let mut`，不那么流畅。

---

## 完整运行示例

```rust
#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
    timeout_ms: u64,
}

#[derive(Default)]
struct HttpRequestBuilder {
    url: Option<String>,
    method: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
    timeout_ms: u64,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self {
            method: "GET".into(),
            timeout_ms: 5000,
            ..Default::default()
        }
    }

    fn url(mut self, url: &str) -> Self {
        self.url = Some(url.into()); self
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.into(); self
    }

    fn header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.into(), val.into()); self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.into()); self
    }

    fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms; self
    }

    fn build(self) -> Result<HttpRequest, String> {
        let url = self.url.ok_or("URL 是必填项")?;
        Ok(HttpRequest {
            url, method: self.method,
            headers: self.headers, body: self.body,
            timeout_ms: self.timeout_ms,
        })
    }
}

pub fn run() {
    let request = HttpRequestBuilder::new()
        .url("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice"}"#)
        .timeout(10_000)
        .build()
        .expect("构建请求失败");

    println!("{:#?}", request);
}
```

---

## 下一步

- 继续阅读：[3. 简化大型 Struct](./3-简化大型Struct.md)
- 回到目录：[第 11 章：结构化项目](./README.md)
