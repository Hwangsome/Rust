# 8. RefCell<T>：运行时检查的内部可变性

> - **所属章节**：第 8 章 · Memory Management Features
> - **Cargo package**：`chapter08`
> - **运行方式**：`cargo run -p chapter08`
> - **代码位置**：`chapters/chapter08/src/topic_08_refcell.rs`
> - **上一篇**：[7. Rc 智能指针](./7-Rc智能指针.md)
> - **下一篇**：[9. RefCell 实际案例](./9-RefCell示例.md)
> - **关键词**：`RefCell<T>`、内部可变性、`borrow`、`borrow_mut`、运行时检查、E0502 vs panic

---

## 这一节解决什么问题

借用规则（同时只能有一个 `&mut T` 或多个 `&T`）在编译期被强制执行。但有些设计模式需要"**外部看起来不可变，内部可以改变**"——比如缓存、计数器、观察者模式。

`RefCell<T>` 把借用检查推迟到**运行时**：

- 你用 `&RefCell<T>` 引用（编译器看来是只读）
- 但可以在运行时获取 `&mut T`
- 如果违反规则，**运行时 panic**（而不是编译错误）

---

## 一分钟结论

- `RefCell::new(value)` 创建内部可变容器
- `.borrow()` → `Ref<T>`（只读借用，类似 `&T`）
- `.borrow_mut()` → `RefMut<T>`（可变借用，类似 `&mut T`）
- 借用规则仍然有效，但在运行时检查（违反时 panic）
- `Ref<T>` 和 `RefMut<T>` 是 RAII 守卫，drop 时自动释放借用
- 不能跨线程使用（`!Sync`）；跨线程用 `Mutex<T>`

---

## 与其他语言对比

| 语言 | "看起来不可变但内部可变"的实现 |
|-----|--------------------------|
| Java | `private` 字段 + `synchronized` 方法（没有借用系统）|
| C++ | `mutable` 关键字（编译器不检查）|
| Python | 惯例（`_private`），没有强制 |
| Rust | `RefCell<T>`（借用规则在运行时检查）|

---

## 核心概念与心智模型

```
编译时借用检查（普通 &mut）：
  let mut data = vec![1, 2, 3];
  let r = &data;       // ← 编译器记录"有只读借用"
  data.push(4);        // ← 编译器拒绝：有只读借用存在
  println!("{r:?}");

运行时借用检查（RefCell）：
  let data = RefCell::new(vec![1, 2, 3]);
  let r = data.borrow();    // ← 运行时：借用计数 +1（只读）
  data.borrow_mut();        // ← 运行时 panic！：已有只读借用
  // 如果你能保证不违反规则，这种模式很有用
```

---

## 详细原理

### 1. 基础用法

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// 只读借用
{
    let borrowed = data.borrow();
    println!("read: {:?}", *borrowed);
    // borrowed 在块结束时 drop，只读借用释放
}

// 可变借用（在只读借用结束后）
{
    let mut borrowed_mut = data.borrow_mut();
    borrowed_mut.push(4);
    println!("after push: {:?}", *borrowed_mut);
}

println!("final: {:?}", data.borrow());
```

### 2. 内部可变性模式

```rust
use std::cell::Cell; // 对 Copy 类型更高效的内部可变

struct CallCounter {
    count: Cell<u32>,  // 内部可变
    name: String,
}

impl CallCounter {
    fn new(name: &str) -> Self {
        CallCounter { count: Cell::new(0), name: name.to_string() }
    }

    // &self 方法（看起来只读），但内部可以修改 count
    fn call(&self) {
        self.count.set(self.count.get() + 1);
        println!("[{}] called {} times", self.name, self.count.get());
    }
}

let counter = CallCounter::new("API");
counter.call();
counter.call();
counter.call();
// counter 只需要 &（不可变引用），但内部状态在变化
```

### 3. 运行时 panic 的情况

```rust
use std::cell::RefCell;

let data = RefCell::new(42);

let borrow1 = data.borrow();  // 只读借用
// let borrow_mut = data.borrow_mut(); // ❌ 运行时 panic！
// thread 'main' panicked at 'already borrowed: BorrowMutError'

// 使用 try_borrow_mut 安全检查
match data.try_borrow_mut() {
    Ok(_) => println!("获取可变借用成功"),
    Err(e) => println!("获取失败: {e}"),  // 输出错误而非 panic
}
```

---

## 完整运行示例

```rust
use std::cell::RefCell;
use std::rc::Rc;

// 观察者模式：多个观察者共享同一个计数器
struct EventLog {
    events: RefCell<Vec<String>>,
}

impl EventLog {
    fn new() -> Rc<Self> {
        Rc::new(EventLog { events: RefCell::new(Vec::new()) })
    }

    fn log(&self, event: &str) {
        self.events.borrow_mut().push(event.to_string());
    }

    fn print_all(&self) {
        for (i, event) in self.events.borrow().iter().enumerate() {
            println!("  [{i}] {event}");
        }
    }
}

fn main() {
    let log = EventLog::new();
    let log2 = Rc::clone(&log);

    // 多个持有者都可以记录事件
    log.log("用户登录");
    log2.log("页面访问");
    log.log("购买商品");
    log2.log("用户登出");

    println!("=== 事件日志 ===");
    log.print_all();
    println!();

    println!("=== try_borrow_mut 安全模式 ===");
    let data = RefCell::new(100_i32);
    let _hold = data.borrow(); // 持有只读借用

    match data.try_borrow_mut() {
        Ok(mut v) => { *v *= 2; println!("成功修改"); }
        Err(_) => println!("当前有只读借用，无法可变借用"),
    }
    // _hold 在这里 drop，释放只读借用

    match data.try_borrow_mut() {
        Ok(mut v) => { *v *= 2; println!("现在可以修改了: {}", *v); }
        Err(_) => println!("失败"),
    }
    println!();

    println!("=== Cell<T>（Copy 类型的内部可变）===");
    use std::cell::Cell;

    let flag = Cell::new(false);
    println!("flag = {}", flag.get());
    flag.set(true);
    println!("flag = {}", flag.get());
    // Cell 比 RefCell 更轻量（无借用守卫），但只支持 Copy 类型
}
```

---

## RefCell vs Mutex

| 特性 | `RefCell<T>` | `Mutex<T>` |
|-----|------------|----------|
| 线程安全 | ❌（单线程）| ✅（多线程）|
| 检查方式 | 运行时（panic）| 运行时（阻塞/错误）|
| 开销 | 极小（借用计数）| 操作系统锁（较重）|
| 组合搭档 | `Rc<RefCell<T>>`（单线程）| `Arc<Mutex<T>>`（多线程）|

---

## 下一步

- 继续阅读：[9. RefCell 实际案例](./9-RefCell示例.md)
- 回到目录：[第 8 章：内存管理特性](./README.md)
