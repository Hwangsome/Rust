# 3. 简化大型 Struct：拆分与组合

> - **所属章节**：第 11 章 · Structuring Projects
> - **Cargo package**：`chapter11`
> - **运行方式**：`cargo run -p chapter11`
> - **代码位置**：`chapters/chapter11/src/topic_03_simplifying_structures.rs`
> - **上一篇**：[2. Builder 模式](./2-Builder模式.md)
> - **下一篇**：本章最后一篇
> - **关键词**：struct 拆分、借用冲突、字段级借用、组合、职责分离

---

## 这一节解决什么问题

一个大 struct 里，你想同时借用两个字段——一个可变借用，一个不可变借用——但编译器不允许：

```rust
struct App {
    ui: Ui,
    data: Data,
    settings: Settings,
}

fn update(app: &mut App) {
    let ui_ref = &app.ui;       // 不可变借用
    app.data.update();          // 可变借用！
    println!("{}", ui_ref.status); // ❌ 同时持有不可变和可变
}
```

将大 struct 拆分成小 struct 后，可以分别独立借用每个子 struct，消除冲突。

---

## Rust 的字段级借用

```
Rust 的借用检查器是字段级的（不是整个 struct 级）：

let (a, b) = (&app.ui, &mut app.data);
↑ 可以！ui 和 data 是不同字段，不冲突

但如果你先借了整个 &app，就会锁住所有字段：
let r = &app;         // 借用了整个 app
app.data.update();    // ❌ 不能再借 app 的一部分
```

---

## 完整运行示例

```rust
#[derive(Debug, Default)]
struct EngineStats {
    rpm: u32,
    temperature: f64,
    oil_level: f64,
}

#[derive(Debug, Default)]
struct NavigationData {
    lat: f64,
    lon: f64,
    speed_kmh: f64,
}

#[derive(Debug, Default)]
struct DashboardState {
    warning_count: u32,
    last_error: Option<String>,
}

// 拆分后的大型系统
#[derive(Debug, Default)]
struct Car {
    engine: EngineStats,
    navigation: NavigationData,
    dashboard: DashboardState,
}

fn update_car(car: &mut Car) {
    // ✅ 可以同时借用不同字段
    let nav_ref = &car.navigation; // 不可变
    car.engine.rpm += 100;          // 可变（不同字段）
    car.dashboard.warning_count += 1;

    println!("当前位置: ({:.2}, {:.2})", nav_ref.lat, nav_ref.lon);
    println!("转速: {} RPM", car.engine.rpm);
}

pub fn run() {
    let mut car = Car::default();
    car.navigation.lat = 39.9;
    car.navigation.lon = 116.4;

    update_car(&mut car);
    println!("{:#?}", car);
}
```

---

## 拆分的好处总结

1. **消除借用冲突**：不同子 struct 可以独立借用
2. **职责清晰**：每个 struct 只管自己的数据
3. **可以单独传参**：`fn update_engine(engine: &mut EngineStats)` 比 `fn update_engine(car: &mut Car)` 更通用
4. **复用**：同一个 `NavigationData` 可以用在 Car、Drone 等多种类型里

---

## 下一步

第 11 章完成！

- 回到目录：[第 11 章：结构化项目](./README.md)
- 下一章：[第 11 章：错误处理](../chapter11/README.md)
