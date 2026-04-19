# 1. Struct 基础：自定义数据类型

> - **所属章节**：第 3 章 · Custom and Library Provided Types
> - **Cargo package**：`chapter03`
> - **运行方式**：`cargo run -p chapter03`
> - **代码位置**：`chapters/chapter03/src/topic_01_structs_basics.rs`
> - **上一篇**：本章第一篇
> - **下一篇**：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
> - **关键词**：`struct`、字段、具名字段、元组 struct、单元 struct、`#[derive]`、字段简写、更新语法

---

## 这一节解决什么问题

你需要把"用户名、邮箱、是否活跃"这三个相关数据打包成一个整体，而不是用三个独立变量传来传去。

`struct` 是 Rust 创建自定义类型的基础工具——把相关字段组合在一起，赋予整体一个有意义的名字。

---

## 一分钟结论

- **具名字段 struct**：最常用，字段有名字，通过 `.name` 访问
- **元组 struct**：字段无名，按位置访问（`.0`、`.1`），适合强类型别名
- **单元 struct**：无字段，大小为 0，用于 Marker 或状态标记
- `#[derive(Debug)]` 让 struct 能被 `{:?}` 打印（否则不能打印！）
- **字段简写**：变量名和字段名相同时可省略（`User { name, ..}`）
- **更新语法**：`User { active: false, ..other }` 复用剩余字段

---

## 与其他语言对比


| 语言         | 等价概念                        | 特点                         |
| ---------- | --------------------------- | -------------------------- |
| Java       | `class`（去掉方法）               | 总是在堆上（对象引用）                |
| Pythonself | `@dataclass` / `namedtuple` | 动态类型                       |
| C          | `struct`                    | 无方法，无封装                    |
| Rust       | `struct`                    | 栈上分配（默认），方法在 `impl` 块里，强类型 |


---

## 三种 struct 的内存布局

```
具名字段 struct User：
  ┌────────────────────────────────────────────────────┐
  │  username: String (24字节: ptr+len+cap)            │
  │  email: String (24字节)                            │
  │  active: bool (1字节 + 7字节填充)                  │
  └────────────────────────────────────────────────────┘
  总大小：56字节（包含对齐填充）

元组 struct Meters(f64)：
  ┌──────────────────┐
  │  0: f64 (8字节)  │
  └──────────────────┘
  总大小：8字节

单元 struct Marker：
  （零字节）
```

---

## 详细原理

### 1. 具名字段 struct

```rust
// 定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建（必须填写所有字段）
let user1 = User {
    email: String::from("alice@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

// 访问
println!("{}", user1.username);  // alice
println!("{}", user1.email);     // alice@example.com

// 修改（需要 mut）
let mut user2 = User {
    email: String::from("bob@example.com"),
    username: String::from("bob"),
    active: true,
    sign_in_count: 0,
};
user2.email = String::from("bob_new@example.com");
```

### 2. 字段简写（当变量名和字段名相同）

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // 等同于 email: email
        username,   // 等同于 username: username
        active: true,
        sign_in_count: 0,
    }
}
```

### 3. 结构体更新语法

```rust
let user1 = User {
    email: String::from("alice@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

// 基于 user1 创建新用户，只更改部分字段
let user2 = User {
    email: String::from("alice2@example.com"),  // 覆盖
    ..user1  // 其余字段来自 user1（注意：String 字段会被 move！）
};
// user1.email 现在无效（被 move），但 user1.active 和 sign_in_count 是 Copy 的
```

### 4. 元组 struct（强类型别名）

```rust
struct Meters(f64);
struct Feet(f64);

fn convert_to_meters(f: Feet) -> Meters {
    Meters(f.0 * 0.3048)
}

let height = Feet(6.0);
let metric = convert_to_meters(height);
println!("{:.2} 米", metric.0);

// 强类型的价值：
let m: Meters = Meters(1.0);
// let f: Feet = m; // ❌ 类型不同，编译错误！防止单位混用
```

### 5. 单元 struct

```rust
struct AlwaysEqual;

// 实现 trait 但不需要数据
trait Report {
    fn generate(&self) -> String;
}

impl Report for AlwaysEqual {
    fn generate(&self) -> String { "标准报告".to_string() }
}

let marker = AlwaysEqual;
println!("{}", marker.generate());
println!("大小: {} 字节", std::mem::size_of::<AlwaysEqual>()); // 0
```

### 6. `#[derive(Debug)]`：是什么、为什么、怎么样

调试打印是日常开发里最高频的需求之一。Rust 不会给自定义 `struct` 自动生成「怎么打印」的规则，所以要么**派生** `Debug`，要么**手写** `impl Debug`。

#### 是什么

- **`Debug`** 是标准库里的一个 **trait**，约定类型可以用「程序员向」的格式格式化出来。
- **`#[derive(Debug)]`** 是**派生宏**：在编译期为你的 `struct` **自动生成**一份 `impl Debug for YourType { ... }`，字段按默认规则递归打印。
- 与 **`Display`** 不同：`Display` 没有默认派生，且用于 **`{}`** 的「用户向」展示；**`Debug` 用于 `{:?}` 和 `{:#?}`**。

#### 为什么需要它

- `println!("{}", user)` 要求类型实现 **`Display`**，自定义 `struct` 默认**没有**。
- `println!("{:?}", user)` 要求类型实现 **`Debug`**，同样默认**没有**。
- 加上 `#[derive(Debug)]` 后，才能直接 **`println!("{:?}", instance)`** 做日志、断言失败信息、快速 REPL 式排查，而不必先手写一堆 `format!`。

#### 怎么样用

**1）写在 `struct` 上一行（元组 struct、单元 struct 同理）**

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);   // 一行：Point { x: 1, y: 2 }
println!("{:#?}", p);  // 多行缩进，可读性更好
```

**2）字段类型的约束**

派生 `Debug` 时，**每个字段的类型也必须实现 `Debug`**（递归）。因此常见组合是：内层标准类型都没问题；若某字段是你自定义的别的 `struct`，那个类型往往也要 `#[derive(Debug)]`。

**3）和 `Clone` / `PartialEq` 等叠在一起**

```rust
#[derive(Debug, Clone, PartialEq)]
struct Label(String);
```

多个 trait 写在同一个 `derive(...)` 里，用逗号分隔；顺序一般无所谓（极少数宏展开顺序坑极少遇到）。

**4）不想用派生时**

可以 **`impl std::fmt::Debug for MyType { ... }` 手写**（控制打印哪些字段、脱敏等）。练习阶段几乎总是 **`derive` 够用**。

#### 常见报错（心里有个数）

- 去掉 `#[derive(Debug)]` 仍写 `println!("{:?}", p)` → 编译器提示 **`Debug` is not satisfied**。
- 想用 **`{}`** 打印自定义 struct → 需要 **`Display`**（或改成 **`{:?}`**）。

---

## 完整运行示例

```rust
use std::mem::size_of;

#[derive(Debug, Clone, PartialEq)]
struct Article {
    title: String,
    author: String,
    word_count: u32,
    published: bool,
}

impl Article {
    fn summary(&self) -> String {
        format!("《{}》by {} ({} 字)", self.title, self.author, self.word_count)
    }
}

// 元组 struct
#[derive(Debug)]
struct Celsius(f64);
#[derive(Debug)]
struct Fahrenheit(f64);

impl Celsius {
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    println!("=== 具名字段 struct ===");
    let article1 = Article {
        title: "Rust 入门".into(),
        author: "Alice".into(),
        word_count: 1500,
        published: true,
    };

    println!("{:?}", article1);
    println!("{}", article1.summary());

    // 更新语法：基于 article1 创建副本
    let article2 = Article {
        title: "Rust 进阶".into(),
        word_count: 3000,
        ..article1.clone()  // 用 clone 避免 move
    };
    println!("{}", article2.summary());
    println!();

    println!("=== 元组 struct（强类型单位）===");
    let room_temp = Celsius(22.5);
    let body_temp = Celsius(37.0);
    println!("室温: {:?} → {:?}", room_temp, room_temp.to_fahrenheit());
    println!("体温: {:?} → {:?}", body_temp, body_temp.to_fahrenheit());
    println!();

    println!("=== struct 大小 ===");
    println!("Article: {} 字节", size_of::<Article>());
    println!("Celsius: {} 字节", size_of::<Celsius>());
}
```

---

## 注意点与陷阱

### 陷阱 1：struct 没有 `#[derive(Debug)]` 就不能打印

（原理与用法展开见上文 **「### 6. `#[derive(Debug)]`」**。）

```rust
struct Point { x: i32, y: i32 }
// println!("{:?}", Point { x: 1, y: 2 }); // ❌

#[derive(Debug)]
struct Point2 { x: i32, y: i32 }
println!("{:?}", Point2 { x: 1, y: 2 }); // ✅
```

### 陷阱 2：更新语法会 move 非 Copy 字段

```rust
let u1 = User { username: "alice".into(), email: "alice@example.com".into(), .. };
let u2 = User { email: "new@example.com".into(), ..u1 };
// u1.username 已被 move，u1 整体不能再用（如果后续还用到的话）
```

---

## 下一步

- 继续阅读：[2. 为 Struct 添加功能](./2-为Struct添加功能.md)
- 文中 `&self` / `self` 想系统搞懂：[11. `self` 与 `Self` 专题](./11-self与Self.md)
- 回到目录：[第 3 章：自定义类型](./README.md)
- 官方参考：[The Rust Book - Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

