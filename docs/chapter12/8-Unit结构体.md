# 8. Unit 结构体与类型状态机

> - **所属章节**：第 12 章 · Understanding Size in Rust
> - **Cargo package**：`chapter12`
> - **运行方式**：`cargo run -p chapter12`
> - **代码位置**：`chapters/chapter12/src/topic_08_zero_sized_types_unit_structs.rs`
> - **Lab**：`chapters/chapter12/src/lab.rs`
> - **上一篇**：[7. Unit 类型 `()`](./7-Unit类型.md)
> - **下一篇**：[9. `PhantomData<T>`](./9-PhantomData.md)
> - **关键词**：unit struct、marker struct、type state、类型状态机、ZST、`PhantomData`、编译期保证

---

## 这一节解决什么问题

有时候，你需要用类型来**区分状态**，而不是用字段值。例如：

- `Connection<Disconnected>` 和 `Connection<Connected>` 是不同类型，前者没有 `send()` 方法，后者才有
- 这样，"在未连接的连接上发送数据"就变成了**编译期错误**，而不是运行期 panic

这套模式叫**类型状态机（Typestate Pattern）**，实现它的关键工具之一就是 **Unit 结构体**：`struct Connected;` 这样的零字段、零大小的结构体。

Unit 结构体的核心价值：
- 大小为 0，不增加任何运行期开销
- 每个 unit struct 是**唯一的类型**，可以作为类型参数区分状态
- 可以实现 trait，参与 trait 机制
- 可以和 `PhantomData` 配合做更精细的类型标记

---

## 一分钟结论

- Unit 结构体 `struct Marker;` 大小为 **0 字节**，只有一个值（它自己）
- 两个不同的 unit struct（如 `struct A;` 和 `struct B;`）是**不同类型**，这是区分状态的关键
- Unit struct 可以实现 trait，可以作为泛型参数
- 类型状态机：把"允许的操作"通过 `impl<State> Type<State>` 限制到特定状态
- 类型状态机把很多运行时 panic 变成了**编译期错误**，这是 Rust 零成本抽象的典型案例

---

## 与其他语言对比

| 语言 | 如何实现"只有在某个状态下才能调用某个方法" |
|-----|----------------------------------|
| Java | 运行时检查，`if (state != CONNECTED) throw new IllegalStateException()` |
| C++ | 运行时检查，`assert(connected)` |
| Go | 运行时检查，`if !c.connected { return ErrNotConnected }` |
| Rust | **编译期**：不同状态是不同类型，错误调用直接编译失败 |

Rust 通过类型状态机把"这个方法不应该在这个状态下调用"从运行期 panic 变成编译期错误——这就是"让非法状态不可表示（make illegal states unrepresentable）"的工程实践。

---

## 核心概念与心智模型

### Unit 结构体的本质

```
普通结构体：有字段，存数据，有大小
struct Person {
    name: String,  // 24 字节
    age: u32,      // 4 字节
}  // 总大小 ≈ 28 字节（含对齐）

Unit 结构体：无字段，不存数据，大小为 0
struct Disconnected;   // 0 字节
struct Connected;      // 0 字节
struct Authenticated;  // 0 字节

// 但它们是三个不同的类型！
// 就像不同颜色的空信封：信封里什么都没有，但颜色（类型名）本身携带了信息
```

### 类型状态机的结构

```rust
// 状态标记（unit structs）
struct Disconnected;   // ← 状态 A
struct Connected;      // ← 状态 B
struct Authenticated;  // ← 状态 C

// 带状态参数的主结构体
struct Connection<State> {
    host: String,
    _state: std::marker::PhantomData<State>,
}

// 为不同状态实现不同方法
impl Connection<Disconnected> {
    fn new(host: String) -> Self { ... }
    fn connect(self) -> Connection<Connected> { ... } // 状态转换
}

impl Connection<Connected> {
    fn authenticate(self, token: &str) -> Connection<Authenticated> { ... }
    fn disconnect(self) -> Connection<Disconnected> { ... }
    // 注意：没有 send()，必须先 authenticate
}

impl Connection<Authenticated> {
    fn send(&self, data: &[u8]) { ... }  // 只有认证后才能 send
    fn receive(&mut self) -> Vec<u8> { ... }
}

// 结果：
let conn = Connection::new("localhost".into());
// conn.send(&[1, 2, 3]); // ❌ 编译错误！Disconnected 状态没有 send
let conn = conn.connect();
// conn.send(&[1, 2, 3]); // ❌ 编译错误！Connected 状态没有 send
let conn = conn.authenticate("token");
conn.send(&[1, 2, 3]); // ✅ 只有 Authenticated 状态才有 send
```

---

## 详细原理

### 1. Unit 结构体的基本用法

```rust
use std::mem::size_of;

struct Empty;         // unit struct
struct AlsoEmpty {}   // 等价写法（空的具名字段 struct）
struct Unit();        // 等价写法（空的元组 struct）

println!("Empty     = {} bytes", size_of::<Empty>());     // 0
println!("AlsoEmpty = {} bytes", size_of::<AlsoEmpty>()); // 0
println!("Unit      = {} bytes", size_of::<Unit>());      // 0
```

### 2. Unit struct 作为 trait 实现者

```rust
trait Area {
    fn area() -> f64;
}

struct Circle;
struct Square;

impl Area for Circle {
    fn area() -> f64 { std::f64::consts::PI } // 单位圆面积
}

impl Area for Square {
    fn area() -> f64 { 1.0 } // 单位正方形面积
}

// 通过类型来选择实现，而不是通过值
println!("Circle area: {}", <Circle as Area>::area());
println!("Square area: {}", <Square as Area>::area());
```

### 3. 完整的类型状态机实现

```rust
use std::marker::PhantomData;

// 状态标记
struct Locked;
struct Unlocked;

// 带状态的保险箱
struct Safe<State> {
    content: String,
    _state: PhantomData<State>,
}

impl Safe<Locked> {
    pub fn new(content: String) -> Self {
        Safe { content, _state: PhantomData }
    }

    // 开锁：消费 Locked 状态，产生 Unlocked 状态
    pub fn unlock(self, pin: &str) -> Result<Safe<Unlocked>, Safe<Locked>> {
        if pin == "1234" {
            Ok(Safe { content: self.content, _state: PhantomData })
        } else {
            Err(self) // 密码错误，归还 Locked 状态的 Safe
        }
    }
    // 注意：Safe<Locked> 没有 read() 方法！
}

impl Safe<Unlocked> {
    // 读取内容
    pub fn read(&self) -> &str {
        &self.content
    }

    // 上锁：消费 Unlocked 状态，产生 Locked 状态
    pub fn lock(self) -> Safe<Locked> {
        Safe { content: self.content, _state: PhantomData }
    }
}

fn demo_safe() {
    let locked_safe = Safe::new("秘密文件".to_string());

    // 不能在锁定状态下读取：
    // locked_safe.read(); // ❌ 编译错误！Safe<Locked> 没有 read 方法

    match locked_safe.unlock("0000") {
        Err(still_locked) => {
            println!("密码错误，保险箱仍然锁着");
            // 我们仍然持有 still_locked: Safe<Locked>
            match still_locked.unlock("1234") {
                Ok(unlocked) => {
                    println!("成功解锁！内容: {}", unlocked.read());
                    let _re_locked = unlocked.lock();
                    // _re_locked.read(); // ❌ 重新锁定后不能读
                }
                Err(_) => println!("还是错了"),
            }
        }
        Ok(_) => unreachable!(),
    }
}
```

---

## 完整运行示例

```rust
use std::marker::PhantomData;
use std::mem::size_of;

// === Part 1: Unit struct 基础 ===
struct Empty;
#[derive(Debug)] struct Red;
#[derive(Debug)] struct Green;
#[derive(Debug)] struct Blue;

// === Part 2: 类型状态机（简化版 TCP 连接）===
struct Closed;
struct Open;
struct Bound;

struct TcpSocket<State> {
    port: Option<u16>,
    _state: PhantomData<State>,
}

impl TcpSocket<Closed> {
    fn new() -> Self {
        println!("  创建新 Socket（Closed 状态）");
        TcpSocket { port: None, _state: PhantomData }
    }

    fn open(self) -> TcpSocket<Open> {
        println!("  Socket 打开（Closed → Open）");
        TcpSocket { port: self.port, _state: PhantomData }
    }
}

impl TcpSocket<Open> {
    fn bind(self, port: u16) -> TcpSocket<Bound> {
        println!("  Socket 绑定到端口 {}（Open → Bound）", port);
        TcpSocket { port: Some(port), _state: PhantomData }
    }

    fn close(self) -> TcpSocket<Closed> {
        println!("  Socket 关闭（Open → Closed）");
        TcpSocket { port: None, _state: PhantomData }
    }
    // 注意：Open 状态没有 listen() 或 accept()
}

impl TcpSocket<Bound> {
    fn listen(&self) {
        println!("  开始监听端口 {:?}", self.port);
    }
}

fn main() {
    println!("=== Unit Struct 大小 ===");
    println!("Empty = {} bytes", size_of::<Empty>());
    println!("Red   = {} bytes", size_of::<Red>());
    println!("Blue  = {} bytes", size_of::<Blue>());
    println!();

    println!("=== Unit Struct 作为类型区分 ===");
    // Red 和 Blue 是不同类型，即使都是零大小
    let r: Red = Red;
    let b: Blue = Blue;
    println!("r = {:?}, b = {:?}", r, b);
    // let _: Red = b; // ❌ 编译错误！Blue 不是 Red
    println!();

    println!("=== 类型状态机：TcpSocket ===");
    let socket = TcpSocket::<Closed>::new();
    // socket.bind(8080); // ❌ 编译错误！Closed 状态没有 bind()

    let open_socket = socket.open();
    // open_socket.listen(); // ❌ 编译错误！Open 状态没有 listen()

    let bound_socket = open_socket.bind(8080);
    bound_socket.listen(); // ✅ 只有 Bound 状态才能监听

    println!();
    println!("=== 类型状态机的内存开销 ===");
    println!("TcpSocket<Closed> = {} bytes", size_of::<TcpSocket<Closed>>());
    println!("TcpSocket<Open>   = {} bytes", size_of::<TcpSocket<Open>>());
    println!("TcpSocket<Bound>  = {} bytes", size_of::<TcpSocket<Bound>>());
    println!("（只有 Option<u16> 本身的大小，PhantomData 为 0）");
}
```

**预期输出**：

```text
=== Unit Struct 大小 ===
Empty = 0 bytes
Red   = 0 bytes
Blue  = 0 bytes

=== Unit Struct 作为类型区分 ===
r = Red, b = Blue

=== 类型状态机：TcpSocket ===
  创建新 Socket（Closed 状态）
  Socket 打开（Closed → Open）
  Socket 绑定到端口 8080（Open → Bound）
  开始监听端口 Some(8080)

=== 类型状态机的内存开销 ===
TcpSocket<Closed> = 2 bytes  （Option<u16> 的大小）
TcpSocket<Open>   = 2 bytes
TcpSocket<Bound>  = 2 bytes
（只有 Option<u16> 本身的大小，PhantomData 为 0）
```

---

## 编译器错误分析

### ❌ 在错误的状态下调用方法

```rust
let socket = TcpSocket::<Closed>::new();
socket.listen(); // ❌ 在 Closed 状态下调用只有 Bound 才有的方法
```

```text
error[E0599]: no method named `listen` found for struct `TcpSocket<Closed>`
              in the current scope
  |
  | socket.listen();
  |        ^^^^^^
  |
  = note: the method was found for `TcpSocket<Bound>` but not for `TcpSocket<Closed>`
```

**这正是类型状态机的价值**：不需要运行就知道错了。

---

## 实际工程场景

### 1. Builder 模式中的类型验证

```rust
struct Unset;
struct Set<T>(T);

struct RequestBuilder<Url, Method> {
    url: Url,
    method: Method,
}

impl RequestBuilder<Unset, Unset> {
    fn new() -> Self {
        RequestBuilder { url: Unset, method: Unset }
    }
}

impl<M> RequestBuilder<Unset, M> {
    fn url(self, url: String) -> RequestBuilder<Set<String>, M> {
        RequestBuilder { url: Set(url), method: self.method }
    }
}

impl<U> RequestBuilder<U, Unset> {
    fn method(self, m: String) -> RequestBuilder<U, Set<String>> {
        RequestBuilder { url: self.url, method: Set(m) }
    }
}

impl RequestBuilder<Set<String>, Set<String>> {
    fn build(self) -> String {
        format!("{} {}", self.method.0, self.url.0)
    }
    // 只有 url 和 method 都设置后，才有 build() 方法
}

fn main() {
    let req = RequestBuilder::new()
        .url("http://example.com".into())
        .method("GET".into())
        .build();
    println!("{}", req);

    // RequestBuilder::new().build(); // ❌ 编译错误！url 和 method 还是 Unset
}
```

### 2. 网络协议实现

实际的网络库（如 `tokio`、`hyper`）内部大量使用类型状态机来保证协议顺序正确。

---

## 注意点与陷阱

### 陷阱 1：Unit struct 可以被 pattern match，但只有一种模式

```rust
struct Locked;
match some_locked {
    Locked => println!("it's locked"),
    // 不需要 _ 或其他模式，因为 Locked 只有一个值
}
```

### 陷阱 2：类型状态机的方法消费 `self`（不是借用）

```rust
let open = socket.open(); // socket 被消费，open 是新状态
// socket.close(); // ❌ socket 已经 move 走了
```

这是设计上的刻意选择——状态转换后旧状态就不存在了，防止用旧状态继续操作。

### 陷阱 3：类型状态机在很多状态时代码量会增加

如果状态很多，impl 块会很多，代码量增加显著。这时候可以考虑：
- 只对最关键的不变量用类型状态
- 用 enum + 运行时检查处理次要状态

---

## 我的理解与记忆方法

**类比**：

> Unit struct 就像"颜色编码的徽章"——徽章本身是纸片，没有重量，但不同颜色代表不同权限（类型）。安保系统（编译器）会检查你的徽章颜色，不对的颜色不让你进某个区域（调用某个方法）。

**决策树：什么时候用类型状态机**：

```
这个操作如果在错误的状态下调用：
  运行时会 panic？
    是 → 考虑用类型状态机把它变成编译期错误
    否 → 不需要
```

**零成本**：状态标记（unit struct）大小为 0，`PhantomData` 大小为 0，类型状态机**完全不增加运行期开销**。

---

## 下一步

下一篇讲 `PhantomData<T>`——它和 unit struct 的关系非常密切，是实现类型状态机时不可缺少的工具。

- 继续阅读：[9. `PhantomData<T>`](./9-PhantomData.md)
- 回到目录：[第 12 章：Understanding Size in Rust](./README.md)
- 官方参考：[Rust Design Patterns - Type State](https://rust-unofficial.github.io/patterns/patterns/behavioural/typestate.html)
- 延伸阅读：[The Typestate Pattern in Rust - Will Crichton](http://willcrichton.net/rust-api-type-patterns/typestate.html)
