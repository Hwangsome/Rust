# 5. Trait Object 强转

> - **所属章节**：第 13 章 · Coercion in Rust
> - **代码位置**：`chapters/chapter13/src/topic_05_trait_objects_coercion.rs`
> - **上一篇**：[4. 函数项强转](./4-函数项强转.md)
> - **下一篇**：[6. Unsized 强转（再访）](./6-unsized强转再访.md)

---

## `&T` / `Box<T>` → `&dyn Trait` / `Box<dyn Trait>`

当 T 实现了 Trait，并且 Trait 是 object-safe 时：

```rust
trait Draw { fn draw(&self) -> String; }

struct Button { label: String }
struct Circle { radius: f64 }

impl Draw for Button { fn draw(&self) -> String { format!("button: {}", self.label) } }
impl Draw for Circle { fn draw(&self) -> String { format!("circle r={}", self.radius) } }

fn render(d: &dyn Draw) { println!("{}", d.draw()); }

let b = Button { label: "OK".into() };
let c = Circle { radius: 5.0 };

render(&b); // &Button → &dyn Draw（自动）
render(&c); // &Circle → &dyn Draw（自动）

// Box 同理
let widgets: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: "Submit".into() }), // Box<Button> → Box<dyn Draw>
    Box::new(Circle { radius: 3.0 }),
];
for w in &widgets { println!("{}", w.draw()); }
```

---

## 为什么会发生

`&T` coerce 到 `&dyn Trait` 的原理：
1. 从细指针（8字节地址）变成胖指针（16字节：地址 + vtable）
2. vtable 中存放 T 对 Trait 各方法的函数指针
3. 这是 Unsized Coercion 的一种

---

## 下一步

- 继续阅读：[6. Unsized 强转（再访）](./6-unsized强转再访.md)
