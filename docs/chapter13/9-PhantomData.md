# 9. `PhantomData<T>`：让类型系统知道你和 T 的关系

> - **所属章节**：第 13 章 · Understanding Size in Rust
> - **Cargo package**：`chapter13`
> - **运行方式**：`cargo run -p chapter13`
> - **代码位置**：`chapters/chapter13/src/topic_09_zero_sized_types_phantom_data.rs`
> - **Lab**：`chapters/chapter13/src/lab.rs`
> - **上一篇**：[8. Unit 结构体与类型状态机](./8-Unit结构体.md)
> - **下一篇**：本章最后一篇
> - **关键词**：`PhantomData`、标记数据、生命周期绑定、`Send`/`Sync` 控制、协变/逆变、drop check、ZST

---

## 这一节解决什么问题

有时候你写了一个结构体，它的字段里**没有直接持有** `T`，但逻辑上它的行为"像是持有 T"：

```rust
struct MyVec<T> {
    ptr: *mut T,  // 裸指针，不走所有权系统
    len: usize,
    cap: usize,
}
```

这里的 `ptr` 是裸指针，编译器认为 `MyVec<T>` 跟 `T` 的类型信息没有关系——不会为它推断 `Send`/`Sync`，不会检查生命周期，不会做 drop check。这是危险的。

`PhantomData<T>` 就是为了解决这个问题：**在不真正持有 T 的情况下，告诉编译器"我逻辑上和 T 有关"**。

PhantomData 的三大用途：
1. **绑定生命周期**：让 struct 和某个生命周期参数关联
2. **影响 `Send`/`Sync` 自动推断**：通过持有 `PhantomData<Rc<()>>` 让 struct 变成 `!Send`
3. **影响协变/逆变**：`PhantomData<T>` 是协变，`PhantomData<*mut T>` 是不变
4. **drop check**：告诉 Rust drop 顺序验证器"我会 drop T"

---

## 一分钟结论

- `PhantomData<T>` 大小：**0 字节**（ZST，零大小类型）
- 功能：让编译器知道 struct 逻辑上"持有"或"关联"某个类型/生命周期
- 主要用途：
  - `PhantomData<T>`：表示"我持有 T"（协变，影响 drop check）
  - `PhantomData<*const T>`：表示"我和 T 相关，但不 own 它"（不变，不影响 Send/Sync）
  - `PhantomData<*mut T>`：表示"我可以写 T"（不变）
  - `PhantomData<fn() -> T>`：逆变（极少使用）
  - `PhantomData<Rc<()>>`：让 struct 变成 `!Send + !Sync`
- 绝大多数业务代码不需要手写 `PhantomData`，它是 unsafe/library 代码的工具

---

## 与其他语言对比

| 语言 | 如何在不存储数据的情况下关联类型信息 |
|-----|--------------------------------|
| C / C++ | 模板参数可以存在于类型但不影响内存，但编译器不像 Rust 那样检查 Send/Sync |
| Java | 泛型擦除，运行时没有类型信息 |
| Haskell | `newtype` 可以包一层不占额外内存的类型 |
| Rust | `PhantomData<T>`：显式告诉编译器类型信息，零运行期成本 |

---

## 核心概念与心智模型

### PhantomData 的本质：给编译器的"声明"

```rust
use std::marker::PhantomData;

// 没有 PhantomData 时：
struct BadIterator<T> {
    ptr: *const T,
    len: usize,
    pos: usize,
}
// 问题：编译器不知道 BadIterator<T> 和 T 的关系
// - 不会自动推断 Send/Sync
// - 不会检查 T 的生命周期
// - 可能出现悬垂指针

// 有 PhantomData 时：
struct GoodIterator<T> {
    ptr: *const T,
    len: usize,
    pos: usize,
    _marker: PhantomData<T>,  // ← 告诉编译器：我逻辑上持有 T
}
// 编译器现在会：
// - 如果 T: Send，则 GoodIterator<T>: Send
// - 如果 T: Sync，则 GoodIterator<T>: Sync
// - 在 drop check 时检查 T 的生命周期
```

### 三种不同的 PhantomData 语义

```
PhantomData<T>           →  "我 own 一个 T"（协变，Send/Sync 随 T）
PhantomData<&'a T>       →  "我持有对 T 的不可变引用"（协变，生命周期 'a）
PhantomData<&'a mut T>   →  "我持有对 T 的可变引用"（不变，生命周期 'a）
PhantomData<*const T>    →  "我有指向 T 的裸指针"（不变，!Send!Sync）
PhantomData<*mut T>      →  "我有可写指针到 T"（不变，!Send!Sync）
PhantomData<fn(T) -> U>  →  关于协变逆变的高级用法（极少用）
PhantomData<Rc<()>>      →  "让我的类型变成 !Send!Sync"
```

---

## 详细原理

### 1. 生命周期绑定

最常见的使用场景：struct 逻辑上借用了某段数据，但字段是裸指针，需要手动绑定生命周期。

```rust
use std::marker::PhantomData;

// 自定义切片迭代器（简化版）
struct StrIterator<'a> {
    ptr: *const u8,
    end: *const u8,
    _lifetime: PhantomData<&'a str>, // ← 绑定生命周期 'a
}

impl<'a> StrIterator<'a> {
    fn new(s: &'a str) -> Self {
        let bytes = s.as_bytes();
        StrIterator {
            ptr: bytes.as_ptr(),
            end: unsafe { bytes.as_ptr().add(bytes.len()) },
            _lifetime: PhantomData,
        }
    }
}

// 没有 PhantomData<&'a str>，编译器不知道这个迭代器和 'a 的关系，
// 可能允许迭代器活过原始 &str 的生命周期，导致悬垂指针
```

### 2. 控制 `Send` 和 `Sync`

```rust
use std::marker::PhantomData;
use std::rc::Rc;

// 让 struct 变成 !Send + !Sync（即使它的字段都是 Send + Sync 的）
struct NotThreadSafe {
    data: i32,
    // Rc<()> 不是 Send，所以包含它的类型也不是 Send
    _not_send: PhantomData<Rc<()>>,
}

fn is_send<T: Send>() {}
// is_send::<NotThreadSafe>(); // ❌ 编译错误！NotThreadSafe 不是 Send

// 用途：当你的类型通过 unsafe 持有了只能在单线程使用的资源时，
// 用 PhantomData<Rc<()>> 来让编译器阻止跨线程传递
```

### 3. 强类型 ID（避免混用不同领域的 ID）

```rust
use std::marker::PhantomData;
use std::fmt;

struct Id<T> {
    value: u64,
    _phantom: PhantomData<*const T>, // 不实际持有 T，只是类型标记
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Id { value, _phantom: PhantomData }
    }
    fn value(&self) -> u64 { self.value }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct User;
struct Order;

type UserId = Id<User>;
type OrderId = Id<Order>;

fn get_user(id: UserId) -> String {
    format!("User {id}")
}

fn main() {
    let uid = UserId::new(42);
    let oid = OrderId::new(100);

    println!("{}", get_user(uid));    // ✅
    // get_user(oid);                 // ❌ 编译错误！OrderId 不是 UserId
}
```

### 4. drop check 的作用

```rust
use std::marker::PhantomData;

// 如果 PhantomData<T> 在 struct 里，drop check 会验证：
// 当 MyOwner<T> 被 drop 时，T 必须仍然活着（或者同时被 drop）
struct MyOwner<T> {
    data: *mut T,
    _owns: PhantomData<T>, // ← 告诉 drop check：我持有 T
}

impl<T> Drop for MyOwner<T> {
    fn drop(&mut self) {
        // 这里我们"知道" *self.data 仍然有效，因为 PhantomData<T> 保证了这一点
        unsafe { std::ptr::drop_in_place(self.data); }
    }
}
```

---

## 完整运行示例

```rust
use std::marker::PhantomData;
use std::mem::size_of;
use std::rc::Rc;

// ===== Part 1: 强类型 ID =====
struct User;
struct Product;

struct Id<T> {
    raw: u64,
    _phantom: PhantomData<*const T>,
}

impl<T> Id<T> {
    fn new(raw: u64) -> Self { Id { raw, _phantom: PhantomData } }
    fn raw(&self) -> u64 { self.raw }
}

type UserId = Id<User>;
type ProductId = Id<Product>;

fn fetch_user(id: UserId) -> String { format!("User#{}", id.raw()) }
fn fetch_product(id: ProductId) -> String { format!("Product#{}", id.raw()) }

// ===== Part 2: PhantomData 大小 =====
struct WithPhantom {
    data: i32,
    _p: PhantomData<String>, // 不存 String，大小仍是 i32 的大小
}

// ===== Part 3: !Send 演示 =====
struct SingleThreadOnly {
    value: i32,
    _not_send: PhantomData<Rc<()>>,
}

fn check_send<T: Send>(_: &T) { println!("  T is Send"); }

fn main() {
    // Part 1: 强类型 ID
    println!("=== 强类型 ID ===");
    let uid = UserId::new(42);
    let pid = ProductId::new(100);
    println!("  {}", fetch_user(uid));
    println!("  {}", fetch_product(pid));
    // fetch_user(pid); // ❌ 取消注释会编译失败
    println!();

    // Part 2: 零大小
    println!("=== PhantomData 大小 ===");
    println!("  PhantomData<String>   = {} bytes", size_of::<PhantomData<String>>());
    println!("  PhantomData<Vec<i32>> = {} bytes", size_of::<PhantomData<Vec<i32>>>());
    println!("  WithPhantom           = {} bytes (只有 i32 的大小)", size_of::<WithPhantom>());
    println!();

    // Part 3: !Send
    println!("=== PhantomData 控制 Send ===");
    let normal = 42_i32;
    check_send(&normal);

    let sto = SingleThreadOnly { value: 1, _not_send: PhantomData };
    // check_send(&sto); // ❌ 取消注释：`SingleThreadOnly` cannot be sent between threads
    println!("  SingleThreadOnly 的 check_send 被注释（它不是 Send）");
    let _ = sto;
    println!();
}
```

---

## 编译器错误分析

### ❌ 使用了强类型 ID 但传错类型

```rust
let product_id = ProductId::new(1);
fetch_user(product_id); // ❌
```

```text
error[E0308]: mismatched types
  |
  | fetch_user(product_id);
  |            ^^^^^^^^^^^ expected `Id<User>`, found `Id<Product>`
```

### ❌ `PhantomData<Rc<()>>` 导致跨线程传递失败

```rust
let sto = SingleThreadOnly { value: 1, _not_send: PhantomData };
std::thread::spawn(move || {
    // ❌ sto 包含 PhantomData<Rc<()>>，Rc 不是 Send
    println!("{}", sto.value);
});
```

```text
error[E0277]: `Rc<()>` cannot be sent between threads safely
  |
  | std::thread::spawn(move || { ... })
  | ^^^^^^^^^^^^^^^^^^
  = note: required because of the requirements on `PhantomData<Rc<()>>`
```

---

## 实际工程场景

### 标准库中的 PhantomData

```rust
// Vec<T> 内部（简化）
pub struct Vec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
pub struct RawVec<T, A: Allocator> {
    ptr: Unique<T>,  // 内含 PhantomData<T>
    cap: usize,
    alloc: A,
}

// Unique<T> 内部：
struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    _marker: PhantomData<T>, // 关键！告诉编译器 Vec 逻辑上 own T
}
// 如果没有这个 PhantomData<T>，Vec 对 T 的 Send/Sync 和 drop check 都会不正确
```

### 自定义 FFI 包装

```rust
use std::marker::PhantomData;

// 对 C 库指针的 Rust 安全包装
struct CHandle<T> {
    ptr: *mut libc::c_void,
    _phantom: PhantomData<T>,
}

impl<T> Drop for CHandle<T> {
    fn drop(&mut self) {
        unsafe { ffi::destroy_handle(self.ptr); }
    }
}
```

---

## 注意点与陷阱

### 陷阱 1：`PhantomData<T>` vs `PhantomData<*const T>` 的区别

```rust
// PhantomData<T>：协变，且影响 Send/Sync（T: Send 则 Struct: Send）
struct Owns<T>    { _p: PhantomData<T> }

// PhantomData<*const T>：不变，且 !Send!Sync（裸指针不是 Send）
struct Points<T>  { _p: PhantomData<*const T> }

// 选择哪个取决于你的语义：
// - 如果 struct 逻辑上 own T，用 PhantomData<T>
// - 如果 struct 只是指向 T（不负责 drop），用 PhantomData<*const T>
// - 如果不确定，用 PhantomData<*const T>（更保守）
```

### 陷阱 2：忘记加 `PhantomData` 导致 Send 推断不正确

```rust
struct UnsafeWrapper<T> {
    ptr: *mut T,
    // 忘了 _phantom: PhantomData<T>
}
// 现在 UnsafeWrapper<T> 是 Send（因为裸指针 *mut T 在 Rust 中是 !Send + !Sync）
// 等等，反过来：*mut T 不是 Send，所以 UnsafeWrapper 也不是 Send
// 但逻辑上，如果 T: Send，UnsafeWrapper 应该是 Send
// 加了 PhantomData<T> 之后，Send 的推断就正确了
```

---

## 我的理解与记忆方法

**核心直觉**：

> `PhantomData` 是给编译器看的"便利贴"——你告诉它"虽然我没有实际存 T，但我和 T 的关系就像我存了它一样"。编译器会相信你，并据此检查生命周期、Send/Sync 和 drop 顺序。

**选择哪种 PhantomData**：

```
我逻辑上 own T（负责 drop T）？
  是 → PhantomData<T>
  否 →
    我只是引用 T（只读）？
      是 → PhantomData<&'a T>（如果有生命周期）
           或 PhantomData<*const T>（如果是裸指针，且 !Send!Sync）
    我可以写 T？
      是 → PhantomData<&'a mut T> 或 PhantomData<*mut T>
    我只是需要 T 的类型信息（如 ID 系统）？
      → PhantomData<*const T>（最保守的选择）
```

---

## 下一步

第 13 章到这里已经完整走过了 Rust 类型系统中"大小"这个维度的所有关键概念：
- 什么是 Sized / DST / 胖指针
- `Sized` / `?Sized` 的约束规则
- Unsized Coercion 的触发规则
- ZST 的三种形态：`!`（Never）、`()`（Unit）、Unit struct
- `PhantomData` 在 unsafe 代码中的正确使用

- 回到目录：[第 13 章：Understanding Size in Rust](./README.md)
- 下一章：[第 13 章：Coercion in Rust](../chapter13/README.md)
- 官方参考：[std::marker::PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
- 延伸阅读：[The Rustonomicon - PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html)
