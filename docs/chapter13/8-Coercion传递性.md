# 8. Coercion 传递性

> 关键词：chain coercion、multi-step

## 一分钟结论

只要每一步都是合法 coercion，它们能自动**链式**发生。例如：

```text
Box<String>  →  String        (deref)
&String      →  &str          (deref)
合起来: &Box<String> → &str
```

## 对应代码

- [topic_08_transitivity_in_coercion.rs](../../chapters/chapter13/src/topic_08_transitivity_in_coercion.rs)

## 设计哲学

Rust 的 deref chain 允许你写 API 时**不用关心调用方传进来的具体包装层数**——
只要最终能 deref 到目标类型就行。`&String` / `&Box<String>` / `&Rc<String>` 给 `fn f(&str)` 都一样能工作。
