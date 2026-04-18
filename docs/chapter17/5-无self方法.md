# 5. 方法必须有 self 接收者

通过 `dyn Trait` 调用的方法**必须**有 `&self` / `&mut self` / `self: Box<Self>`。

`fn create() -> Self` 不能通过 dyn 调用（但加 `where Self: Sized` 后可以让整个 trait 仍 object-safe）。

## 对应代码

- [topic_05_function_with_no_self_parameter.rs](../../chapters/chapter17/src/topic_05_function_with_no_self_parameter.rs)
