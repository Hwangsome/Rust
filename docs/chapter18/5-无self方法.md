# 5. 无 `self` 接收者的方法限制

> - **所属章节**：第 18 章 · Trait Object Limitations
> - **代码位置**：`chapters/chapter18/src/topic_05_function_with_no_self_parameter.rs`
> - **上一篇**：[4. 方法泛型参数限制](./4-方法泛型限制.md)
> - **下一篇**：[6. 大小与 Trait Object](./6-大小与TraitObject.md)

---

## 没有 self 的方法无法通过 dyn 调用

```rust
trait Animal {
    fn speak(&self) -> &str;          // ✅ 对象安全（有 &self）
    fn create() -> Self where Self: Sized; // where Self: Sized 排除出 dyn
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> &str { "Woof" }
    fn create() -> Self { Dog }
}

let a: &dyn Animal = &Dog;
println!("{}", a.speak()); // ✅
// Dog::create() 可以用，但不能通过 dyn 调用
```

---

## 为什么 `-> Self` 的方法不对象安全

通过 `dyn Animal` 调用 `create()` 时，不知道 Self 是什么类型，也不知道要分配多大的空间来存放返回值。

---

## 下一步

- 继续阅读：[6. 大小与 Trait Object](./6-大小与TraitObject.md)
