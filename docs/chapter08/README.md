# 第 8 章：Project Structure

> **Cargo package**：`chapter08`  
> **运行方式**：`cargo run -p chapter08`  
> **源码**：`chapters/chapter08/src/`

## 本章目标

将 chapter07 中定义在单一文件内的 accommodation 业务，用**文件式模块**（file-based module pattern）重新组织，演示 Rust 项目如何按职责拆分文件并通过 `mod.rs` + `pub use` 对外暴露统一接口。

## 目录结构

```
chapters/chapter08/src/
├── main.rs
├── lab.rs
└── accommodation/
    ├── mod.rs      ← 声明子模块、定义 trait、pub use re-export
    ├── hotel.rs    ← Hotel 结构体及其 impl
    ├── airbnb.rs   ← Airbnb 结构体及其 impl
    └── hostel.rs   ← Hostel<T> 结构体及其 impl
```

## 主题模块

| # | 文件 | 主题 |
|---|------|------|
| 1 | [1-文件式模块.md](./1-文件式模块.md) | 文件式模块（mod.rs 模式） |
| 2 | [2-Trait关联常量.md](./2-Trait关联常量.md) | Trait 关联常量（Associated Constants） |

## 前后章节

- **上一章**：[第 7 章 · Traits](../chapter07/README.md)
- **下一章**：[第 9 章 · Functional Programming Aspects](../chapter09/README.md)
