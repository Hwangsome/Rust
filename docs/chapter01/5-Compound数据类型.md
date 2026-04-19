# 5. Compound 数据类型：元组与数组

> - **所属章节**：第 1 章 · Quick Startup
> - **Cargo package**：`chapter01`
> - **运行方式**：`cargo run -p chapter01`
> - **代码位置**：`chapters/chapter01/src/topic_04_compound_data_types.rs`
> - **上一篇**：[4. Primitive 数据类型](./4-Primitive数据类型.md)
> - **下一篇**：[6. 函数与代码块](./6-函数与代码块.md)
> - **关键词**：tuple、array、切片、解构、`[T; N]`、`&[T]`

---

## 这一节解决什么问题

有时你需要把几个值组合在一起，但又不想定义一个 struct。Rust 提供了两种内置的"打包"方式：

- **元组（tuple）**：固定数量、不同类型的值组合
- **数组（array）**：固定数量、相同类型的值序列

这两种都是**栈上分配的固定大小**类型——和 `Vec`（堆上动态数组）不同。

---

## 一分钟结论

- **元组**：`(T1, T2, T3)` — 不同类型，按位置 `.0`/`.1`/`.2` 访问，支持解构
- **数组**：`[T; N]` — 相同类型，固定长度（N 是类型的一部分！），按下标 `[i]` 访问
- `[T; 3]` 和 `[T; 4]` 是**不同类型**（长度不同）
- `&[T]`（切片）是数组或 Vec 的视图，长度运行时才知道（DST）
- 数组越界访问在 debug 模式会 panic（不是静默的未定义行为）

---

## 与其他语言对比

| 特性 | Python | Java | Rust tuple | Rust array |
|-----|--------|------|-----------|-----------|
| 固定长度 | 否（list 可变）| 否（ArrayList 可变）| 是 | 是 |
| 不同类型 | 是（list 可混）| 否（同类型）| **是** | 否（同类型）|
| 栈/堆 | 堆 | 堆 | **栈** | **栈** |
| 越界 | 运行时异常 | 运行时异常 | - | **Debug: panic; Release: UB** |

---

## 详细原理

### 1. 元组

```rust
// 创建
let point = (1.0_f64, 2.0_f64);           // (f64, f64)
let person = ("Alice", 30, true);          // (&str, i32, bool)

// 访问：按位置
println!("{} 今年 {} 岁", person.0, person.1);

// 解构
let (name, age, active) = person;
println!("{name}: {age}, active={active}");

// 忽略某些字段
let (x, _, z) = (1, 2, 3);  // _ 忽略第二个

// 元组作为函数返回值（多返回值）
fn min_max(nums: &[i32]) -> (i32, i32) {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    (min, max)
}

let nums = [3, 1, 4, 1, 5, 9, 2, 6];
let (min, max) = min_max(&nums);
println!("最小: {min}, 最大: {max}");

// Unit 类型（空元组）
let nothing: () = ();  // "没有有意义信息的值"
```

### 2. 数组

```rust
// 创建
let scores = [85, 92, 78, 95];        // [i32; 4]
let zeros = [0; 10];                   // 10 个 0，等同 [0, 0, ..., 0]

// 类型中的 N 是常量：
let a: [i32; 4] = [1, 2, 3, 4];
let b: [i32; 5] = [1, 2, 3, 4, 5];
// a 和 b 是不同类型！[i32; 4] ≠ [i32; 5]

// 访问
println!("第一个: {}", scores[0]);
println!("最后一个: {}", scores[scores.len() - 1]);
println!("长度: {}", scores.len());

// 越界：debug 模式 panic
// scores[100]; // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100'
// 安全访问：
let safe = scores.get(100);  // Option<&i32>，不会 panic
println!("安全访问 [100]: {safe:?}");  // None
```

### 3. 切片（slice）

```rust
let arr = [1, 2, 3, 4, 5];

// 切片是对数组（或 Vec）的视图
let all: &[i32] = &arr;          // 全部
let first_three: &[i32] = &arr[..3];  // [1, 2, 3]
let last_two: &[i32] = &arr[3..];     // [4, 5]
let middle: &[i32] = &arr[1..4];      // [2, 3, 4]

// 切片不拥有数据（借用），可以有多个
println!("{all:?}");
println!("{first_three:?}");

// 函数接受切片（比接受 &Vec<T> 更通用）
fn sum(s: &[i32]) -> i32 { s.iter().sum() }

let v = vec![1, 2, 3];
println!("sum of array: {}", sum(&arr));   // 数组自动转切片
println!("sum of vec:   {}", sum(&v));     // Vec 也可以
```

---

## 完整运行示例

```rust
fn main() {
    println!("=== 元组 ===");
    let color = (255_u8, 128_u8, 0_u8);  // RGB
    let (r, g, b) = color;
    println!("RGB({r}, {g}, {b}) → #{:02X}{:02X}{:02X}", r, g, b);

    // 嵌套元组
    let point3d = ((1.0_f64, 2.0), 3.0);
    println!("3D 点: x={}, y={}, z={}", point3d.0.0, point3d.0.1, point3d.1);
    println!();

    println!("=== 数组 ===");
    let temperatures: [f64; 7] = [22.5, 23.0, 19.8, 24.1, 21.5, 20.0, 22.0];
    println!("一周温度: {temperatures:?}");
    println!("平均气温: {:.1}°C", temperatures.iter().sum::<f64>() / temperatures.len() as f64);
    println!("最高温: {:.1}°C", temperatures.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    println!();

    println!("=== 切片与通用函数 ===");
    fn statistics(data: &[f64]) -> (f64, f64, f64) {
        let sum: f64 = data.iter().sum();
        let mean = sum / data.len() as f64;
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (mean, min, max)
    }

    let (mean, min, max) = statistics(&temperatures);
    println!("均值={mean:.1}, 最小={min:.1}, 最大={max:.1}");

    // 同一函数处理 Vec
    let more = vec![25.0, 26.5, 23.0];
    let (m2, n2, x2) = statistics(&more);
    println!("vec 统计: 均值={m2:.1}, 最小={n2:.1}, 最大={x2:.1}");
    println!();

    println!("=== 数组安全访问 ===");
    let data = [10, 20, 30, 40, 50];

    // 越界 → None（不 panic）
    match data.get(2) {
        Some(v) => println!("  data[2] = {v}"),
        None => println!("  索引越界"),
    }

    match data.get(99) {
        Some(v) => println!("  data[99] = {v}"),
        None => println!("  data[99] 越界 → None"),
    }
    println!();

    println!("=== 解构模式 ===");
    let [first, second, rest @ ..] = [1, 2, 3, 4, 5];
    println!("first={first}, second={second}, rest={rest:?}");

    // 元组解构
    let pairs = [(1, "one"), (2, "two"), (3, "three")];
    for (num, name) in &pairs {
        println!("  {num} = {name}");
    }
}
```

---

## 注意点与陷阱

### 陷阱 1：数组长度是类型的一部分

```rust
fn sum_three(a: [i32; 3]) -> i32 { a.iter().sum() }

let arr3 = [1, 2, 3];
let arr4 = [1, 2, 3, 4];

sum_three(arr3); // ✅
// sum_three(arr4); // ❌ [i32; 4] 不是 [i32; 3]！

// 解决：使用切片
fn sum(a: &[i32]) -> i32 { a.iter().sum() }
sum(&arr3); // ✅
sum(&arr4); // ✅
```

### 陷阱 2：release 模式整数溢出回绕

```rust
// debug 模式：溢出 panic
// release 模式：溢出回绕（256 → 0）
let x: u8 = 255;
// x += 1; // debug: panic; release: 0
```

---

## 下一步

- 继续阅读：[6. 函数与代码块](./6-函数与代码块.md)
- 回到目录：[第 1 章：Quick Startup](./README.md)
