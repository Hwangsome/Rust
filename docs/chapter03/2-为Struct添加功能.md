# 2. 为 Struct 添加功能

> 类型：**Study note**
> 关键词：`impl`、method、associated function、`self`
> 上一篇：[1. Struct 基础](./1-Struct基础.md)
> 下一篇：[3. Enum](./3-Enum.md)

## 一分钟结论

- `impl` 让 `struct` 不只是“装数据”，还能“带行为”
- `new()` 只是常见命名，不是关键字
- `&self` / `&mut self` / `self` 三种接收方式分别表达不同的所有权语义

## 接收者速查：`self` / `mut self` / `&self` / `&mut self`

下面四条与常见英文笔记同义（**`mut self` 别和 `&mut self` 混**）：

| 接收者 | 所有权 | 一句话（中文） | One line (English) |
|--------|--------|----------------|----------------------|
| **`self`** | 按值 **move** 给方法 | 调用后外层实例通常**不可用**；形参 `self` 默认**不是** `mut`，**改字段**往往改用 `mut self` | Immutable by-value: receiver **takes ownership**. |
| **`mut self`** | 仍是按值 **move** | 与 `self` 一样会消费调用方实例；区别是 **`self` 这个绑定是 `mut`**，可在返回前改字段再 `return self` | Mutable by-value: **takes ownership**, `self` binding is **mutable** inside the method. |
| **`&self`** | **不** move | 不可变借用，只读；可多次调用 | Immutable reference: **no ownership moved**, shared read. |
| **`&mut self`** | **不** move | 可变借用、独占写；外层需 `let mut x` | Mutable reference: **no ownership moved**, exclusive write. |

**代码对照**：`chapters/chapter03/src/lab.rs` 里 `TaylorSwiftSong` 的「**单曲发行流水线**」故事，`run()` 开头按固定顺序调用：`hand_over_demo_for_pressing`（`self`）→ `add_bonus_hidden_track`（`mut self`）→ `press_kit_quote`（`&self`）→ `engineer_snip_tail_silence`（`&mut self`）。

## 证据来源

- 对应模块：[topic_02_adding_functionality_to_structs.rs](../../chapters/chapter03/src/topic_02_adding_functionality_to_structs.rs)
- 四种接收者对照演示：[lab.rs](../../chapters/chapter03/src/lab.rs)（`TaylorSwiftSong`「单曲发行流水线」+ `run()` 开头故事段）
- 运行章节：`cargo run -p chapter03`

关键输出：

```text
Owner: ABC, Year: 2010, Fuel: 0, Price: 5000
selling price = 5123
Owner: ABC, Year: 2010, Fuel: 10.5, Price: 5000
```

## 扩展演示输出（当前代码已升级）

`topic_02_adding_functionality_to_structs.rs` 把 `impl` 块里所有四类函数都走一遍：关联函数（`new` / `used` / `monthly_insurance`）→ `&self` 只读查询 → `&mut self` 就地修改 → `self` 按值消费。还演示了"同一个类型可以有多个 `impl` 块"。

```text
-- (1) 用关联函数 new 构造 --
Owner: Alice, Year: 2010, Fuel: 0.0, Price: 5000

-- (2) &mut self 方法修改字段 --
Owner: Alice, Year: 2010, Fuel: 15.5, Price: 5000

-- (3) 关联函数被 &self 方法调用 --
selling_price = 5123

-- (5) self 方法消费实例 --
车辆 Alice 已售出，给新车主重新登记。
```

## 定义

`impl` 块是给类型增加方法和关联函数的地方。

## 作用

- 把“和这个类型强相关的行为”收拢到一起
- 用方法签名表达读取、修改、消费实例的语义
- 提升 API 可发现性和可维护性

## 原理

### 1. 关联函数不一定依赖实例

```rust
fn new(owner: String, year: u32, price: u32) -> Self
```

这种函数常写成 `Type::new(...)`，主要用来构造实例。

### 2. 方法接收者表达所有权语义

| 写法 | 含义 |
| --- | --- |
| `&self` | 只读借用实例 |
| `&mut self` | 可变借用实例 |
| `self` | 消费实例本身 |

当前示例中的：

- `display_car_info(&self)`：只读
- `refuel(&mut self, ...)`：修改
- `sell(self)`：拿走整个实例

### 3. `Self` 让实现更紧凑

在 `impl Car` 里，`Self` 就表示 `Car` 本身。

## 最小示例

```rust
struct Car {
    owner: String,
    fuel_level: f32,
}

impl Car {
    fn new(owner: String) -> Self {
        Self { owner, fuel_level: 0.0 }
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn owner(&self) -> &str {
        &self.owner
    }
}
```

## 注意点

### 1. `new` 只是约定，不是语法

你完全可以叫别的名字，但 `new` 最容易被读者理解。

### 2. `self`、`&self`、`&mut self` 差异非常大

这不是风格差异，而是所有权模型的一部分。

### 3. 消费型方法调用后，原实例可能就不能继续用了

像 `sell(self)` 这种方法会把实例拿走。

## 常见错误

### ❌ 错误 1：方法签名只按“能编过”写，不按语义写

正确问题应该是：这个方法到底要读、改，还是消费实例？

### ❌ 错误 2：把关联函数和方法混成一类

- 关联函数：`Type::new()`
- 方法：`value.method()`

### ❌ 错误 3：调用消耗型方法后还想继续用原值

这本质上还是所有权问题。

## 我的理解

- `struct` 是数据模型
- `impl` 是行为模型
- `self` 的不同写法，就是 Rust 把对象行为和所有权系统绑定在一起的方式

## 下一步

下一篇转向另一类自定义类型：`enum`。它解决的不是“多个字段”，而是“多个可能状态”。

- 继续阅读：[3. Enum](./3-Enum.md)
- 回到目录：[第 3 章：Custom and Library Provided](./README.md)
