//! `self` 与 `Self`：与 `docs/chapter03/11-self与Self.md` 对齐的可运行示例。
//!
//! ## 和 `topic_02` 的分工
//!
//! - `topic_02`：用 `Car` 把「关联函数 / `&self` / `&mut self` / 消费 `self`」串成一条业务线。
//! - 本文件：**最小对照表**，一眼看清 `Self`、`&self`、`&mut self`、按值 `self`、`mut self` 的差别。
//!
//! 运行：`cargo run -p chapter03`（本模块在 `lab` 之前执行）。

/// 小计数器：演示 `Self` 别名、`&self`、`&mut self`、按值 `self` 消费。
#[derive(Debug)]
struct Counter(i32);

impl Counter {
    /// 关联函数：无 `self`。`Self` 在此等价于 `Counter`。
    fn new(v: i32) -> Self {
        Self(v)
    }

    /// `&self`：只读；调用方仍拥有 `Counter`。
    fn peek(&self) -> i32 {
        self.0
    }

    /// `&mut self`：独占改字段。
    fn bump(&mut self) {
        self.0 += 1;
    }

    /// 按值 `self`：消费实例，把内部值交出去（之后不能再 `peek`）。
    fn into_inner(self) -> i32 {
        self.0
    }
}

/// 演示 `mut self`（按值 + 参数绑定可变）：不是 `&mut self`，调用仍**消费** `Acc`。
#[derive(Debug)]
struct Acc {
    n: i32,
}

impl Acc {
    fn new(n: i32) -> Self {
        Self { n }
    }

    /// `mut self`：拿到所有权，但在方法体里可以改 `self.n` 再返回 `Self`。
    fn reorder(mut self, delta: i32) -> Self {
        self.n += delta;
        self
    }
}

pub fn run() {
    println!("== self and Self (doc 11) ==");

    // -------------------------------------------------------------------------
    // `Self`、点号调用与“脱糖”直觉（不必手写脱糖，理解即可）
    // -------------------------------------------------------------------------
    println!("-- (1) `Self` 与 `&self` / `&mut self` / 按值 `self` --");
    let mut c = Counter::new(0);
    println!("peek (等价于 Counter::peek(&c)): {}", c.peek());
    c.bump();
    println!("after bump: {}", c.peek());
    let inner = c.into_inner();
    println!("into_inner -> {inner}");
    // `c` 已被消费；取消下一行注释会得到 E0382：
    // println!("{c:?}");

    println!();

    // -------------------------------------------------------------------------
    // `mut self` vs `&mut self`
    // -------------------------------------------------------------------------
    println!("-- (2) `mut self`（按值且绑定可变）≠ `&mut self`（可变借用）--");
    let a = Acc::new(10);
    let a2 = a.reorder(5);
    println!("reorder 消费原 Acc，返回新 Acc: {a2:?}");
    // `a` 已 move 进 `reorder`；不能再使用 `a`。

    println!();
    println!("提示：方法链里 `-> Self` / `-> &mut Self` 与 `self` 形态强相关，见 topic_10。");
}
