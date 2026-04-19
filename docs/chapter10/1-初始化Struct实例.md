# 1. 初始化 Struct 实例的各种方式

> - **所属章节**：第 10 章 · Structuring Projects
> - **Cargo package**：`chapter10`
> - **运行方式**：`cargo run -p chapter10`
> - **代码位置**：`chapters/chapter10/src/topic_01_initializing_struct_instances.rs`
> - **上一篇**：本章第一篇
> - **下一篇**：[2. Builder 模式](./2-Builder模式.md)
> - **关键词**：`new()`、`Default`、`#[derive(Default)]`、字段简写、更新语法、校验构造

---

## 一分钟结论

Rust 的 struct 初始化有四种常见方式：

1. **直接字面量**：`MyStruct { field1: v1, field2: v2 }`
2. **关联函数 `new()`**：约定俗成的构造函数，可以带校验
3. **`#[derive(Default)]` + `T::default()`**：所有字段用类型默认值
4. **更新语法 `..base`**：基于已有实例，只覆盖部分字段

---

## 何时用哪种

```
字段少且所有字段值都已知 → 字面量
需要校验或有复杂初始化逻辑 → new() → Result<Self, E>
字段多，大部分用默认值 → Default + 更新语法
字段很多且可选 → Builder 模式（下一篇）
```

---

## 详细原理

### `new()` 带校验

```rust
#[derive(Debug)]
struct Email(String);

impl Email {
    fn new(s: &str) -> Result<Self, String> {
        if !s.contains('@') || s.len() < 5 {
            return Err(format!("无效邮箱: {s}"));
        }
        Ok(Email(s.to_string()))
    }
}

let email = Email::new("user@example.com").expect("邮箱格式错误");
```

### `Default` + 更新语法

```rust
#[derive(Debug, Default)]
struct ServerConfig {
    host: String,         // Default: ""
    port: u16,            // Default: 0
    max_conns: u32,       // Default: 0
    debug: bool,          // Default: false
    timeout_ms: u64,      // Default: 0
}

// 只改 host 和 port，其余用默认值
let config = ServerConfig {
    host: "localhost".into(),
    port: 8080,
    ..ServerConfig::default()
};
```

### 手动实现 Default

```rust
#[derive(Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    pool_size: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 5432,  // PostgreSQL 默认端口
            pool_size: 10,
        }
    }
}
```

---

## 完整运行示例

```rust
pub fn run() {
    // 1. 字面量
    let point = Point { x: 1.0, y: 2.0 };
    println!("字面量: {:?}", point);

    // 2. new() 带校验
    match User::new("alice", "alice@example.com") {
        Ok(u) => println!("用户: {:?}", u),
        Err(e) => println!("错误: {e}"),
    }

    // 3. Default
    let default_config = ServerConfig::default();
    println!("默认配置: {:?}", default_config);

    // 4. 更新语法
    let custom_config = ServerConfig {
        host: "production.example.com".into(),
        port: 443,
        ..ServerConfig::default()
    };
    println!("自定义配置: {:?}", custom_config);
}
```

---

## 下一步

- 继续阅读：[2. Builder 模式](./2-Builder模式.md)
- 回到目录：[第 10 章：结构化项目](./README.md)
