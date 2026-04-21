# 2. Trait 关联常量（Associated Constants）

> - **所属章节**：第 8 章 · Project Structure
> - **Cargo package**：`chapter08`
> - **运行方式**：`cargo run -p chapter08`
> - **代码位置**：`chapters/chapter08/src/topic_associated_consts.rs`
> - **上一篇**：[1. 文件式模块](./1-文件式模块.md)
> - **下一篇**：[1. 闭包](../chapter09/1-闭包.md)（第 9 章）

---

## 这一节解决什么问题

有时候一个 trait 的每个实现者都需要提供一个**固定的配置值**——不是类型，也不是运行时方法，而是一个编译期就能确定的常量（如税率、精度、最大容量）。

用关联常量（associated constant）可以把这个值声明在 trait 里，强制每个实现者填入，并且默认方法可以直接通过 `Self::CONST` 引用它，不需要把值作为参数传来传去。

---

## 一分钟结论

- `const NAME: Type;` 放在 trait 体内即为关联常量声明，实现者**必须**提供具体值
- 默认方法通过 `Self::NAME` 访问，调用方通过 `T::NAME` 或 `<T as Trait>::NAME` 访问
- 关联常量是**编译期**值，与实例无关，不需要 `&self`
- trait 也可以给关联常量提供**默认值**（`const RATE: f64 = 0.0;`），实现者可选覆盖

---

## 代码结构

```rust
pub trait Taxable {
    const TAX_RATE: f64;           // 关联常量声明（无默认值，必须实现）

    fn price(&self) -> f64;        // 普通抽象方法

    fn tax(&self) -> f64 {         // 默认方法：引用 Self::TAX_RATE
        self.price() * Self::TAX_RATE
    }
    fn total(&self) -> f64 {
        self.price() + self.tax()
    }
}
```

三种住宿各自实现不同税率：

| 类型 | `TAX_RATE` | 说明 |
|------|-----------|------|
| `LuxuryHotelRoom` | `0.15` | 豪华酒店，奢侈品税 15% |
| `VacationRental`  | `0.10` | 民宿短租，服务税 10%   |
| `HostelBed`       | `0.05` | 青旅床位，优惠税率 5%  |

---

## 三种访问关联常量的方式

```rust
// 1. 通过具体类型路径（最常见）
let rate = LuxuryHotelRoom::TAX_RATE;

// 2. 通过泛型参数（在泛型函数内）
fn show_rate<T: Taxable>() {
    println!("{}", T::TAX_RATE);
}

// 3. 完全限定路径（消歧义）
let rate = <LuxuryHotelRoom as Taxable>::TAX_RATE;
```

---

## 关联常量 vs 关联方法的取舍

| | 关联常量 | 关联方法 |
|---|---|---|
| 何时确定 | 编译期 | 运行时 |
| 能否依赖实例字段 | 否 | 是 |
| 能否提供默认值 | 是 | 是 |
| 适用场景 | 类型级固定配置 | 实例级动态计算 |

若税率可以按实例不同（如会员折扣），改为 `fn tax_rate(&self) -> f64` 更合适；若税率是类型级别的固定规则，`const TAX_RATE: f64` 语义更准确，且编译器可内联优化。
