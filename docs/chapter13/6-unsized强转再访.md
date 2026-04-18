# 6. Unsized 强转（再访）

> 关键词：&[T;N] → &[T]、Box<[T;N]> → Box<[T]>

## 一分钟结论

Unsized coercion 是 coercion 家族的一员。它让：

- `&[T; N]` → `&[T]`
- `Box<[T; N]>` → `Box<[T]>`
- `&String` → `&str`

隐式发生，让固定大小的容器能被当作切片使用。

## 对应代码

- [topic_06_unsized_coercion.rs](../../chapters/chapter13/src/topic_06_unsized_coercion.rs)

## 与 chapter12 的对照

Chapter 12 主要讲"什么是 unsized"；这一章把它**纳入 coercion 的统一框架**——两种视角看同一件事。
