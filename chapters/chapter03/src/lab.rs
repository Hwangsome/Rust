//! 第 3 章练习说明。
//!
//! 这一章把"自定义类型"和"标准库提供的类型"放到一起学。
//! 练习的核心是：**让类型表达业务含义**，而不只是"能用就行"。


// 练习草稿：`enum` 的元组变体里只能写**字段类型**，不能写 `"南美"` 这种字面量。
// 正确：携带字符串用 `String`（或 `&'static str`）；构造时再写 `Shape::North("南美".into())`。

#[derive(Debug, Clone)]
struct Student {
    id: i32,
    name: String,
}

impl Student {
    fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// 在**学生列表**里按 `id` 查找。
    ///
    /// 为什么不能写成 `fn get_by_id(id: i32) -> Option<Student>` 且**不**传列表？
    /// 因为编译器不知道要去哪一份数据里查——`id` 只是数字，必须给出 `&[Student]`（或 `Vec`、数据库句柄等）作为数据源。
    ///
    /// - `Option<&Student>`：零拷贝，只返回指向表里那条记录的借用。
    /// - 若要 `Option<Student>`：对已找到的引用 `.cloned()`（因此上面 `derive(Clone)`）。
    fn get_by_id(students: &[Student], id: i32) -> Option<&Student> {
        students.iter().find(|s| s.id == id)
    }

    fn get_by_id_owned(students: &[Student], id: i32) -> Option<Student> {
        Self::get_by_id(students, id).cloned()
    }

    /// 与 `new` 类似：也是**关联函数**，用 **`Student::make_student(...)`** 调用（前面是类型名，不是变量）。
    ///
    /// 使用示例：
    /// - `let s = Student::make_student(3, "Carol".to_string());`
    /// - `let s = Student::make_student(3, String::from("Carol"));`
    ///
    /// `name` 参数类型是 `String`，不能直接把 `&str` 传进去，需要 `.to_string()` / `.into()` / `String::from`。
    /// （想少写字可统一用上面的 `Student::new(id, "Carol")`，参数是 `impl Into<String>`。）
    fn make_student(id: i32, name: String) -> Student {
        Student { id, name }
    }
}

// -------------------------------------------------------------------------
// 普通函数（不在 `impl` 里）里传 `struct`：按值 vs 引用 vs 可变引用
// -------------------------------------------------------------------------
//
// 和方法里的 `&self` 是同一套所有权规则，只是**显式把 `Student` 写在参数类型上**。

/// 只读：借 `Student`，调用方**仍拥有**原来的 `Student`。
fn print_student_card(s: &Student) {
    println!("    [只读] id={}, name={}", s.id(), s.name());
}

/// 修改：可变借用，函数里改字段，调用方**仍拥有**同一个 `Student`（不能同时有别处只读借它）。
fn promote_student_name(s: &mut Student, new_name: &str) {
    s.name = new_name.to_string();
}

/// 按值：拿走所有权，函数结束 `s` 被 drop；调用方**之后不能再**用传进去的那个变量。
fn student_into_audit_line(s: Student) -> String {
    format!("审计: id={} name={}（本条记录消费后不再可用）", s.id(), s.name())
}

fn student_lookup_lab() {
    let roster = vec![
        Student::new(1, "Alice"),
        Student::new(2, "Bob"),
    ];
    println!("  [Student 按 id 查找 demo]");
    match Student::get_by_id(&roster, 2) {
        Some(s) => println!("    id=2 -> {} ({})", s.name(), s.id()),
        None => println!("    id=2 -> 无此人"),
    }
    match Student::get_by_id_owned(&roster, 99) {
        Some(s) => println!("    id=99 -> {s:?}"),
        None => println!("    id=99 -> 无此人（预期）"),
    }

    let _carol = Student::make_student(3, "Carol".to_string());
    println!("    make_student 示例: {:?}", _carol);

    println!("  [函数参数里传 struct：& / &mut / 按值]");
    let mut alice = Student::new(10, "Alice");
    print_student_card(&alice);
    promote_student_name(&mut alice, "Alice Wang");
    print_student_card(&alice);
    let line = student_into_audit_line(alice);
    println!("    {line}");
    // `alice` 已被 move 进 `student_into_audit_line`；取消下一行会 E0382：
    // print_student_card(&alice);


}

// =============================================================================
// 方法接收者（receiver）四种写法 —— 结论速查（可对照下方 `TaylorSwiftSong` 各方法）
// =============================================================================
//
// | 写法（故事演示顺序） | 所有权是否 move 给方法 | 方法体内能否改字段（常见） | 外层调用典型写法        |
// |----------------------|------------------------|----------------------------|-------------------------|
// | ① `self`             | 是（按值）             | 改字段常用 `mut self`      | `x = x.sign()` 接回新 `Self`；无返回则 `x` 不可用 |
// | ② `mut self`         | 是（按值）             | 可以（`self` 绑定是 mut）  | `x = x.packaging()` 重新接回所有权 |
// | ③ `&self`            | 否（不可变借用）       | 否                         | `x.quote()`，`x` 仍可用   |
// | ④ `&mut self`        | 否（可变借用）         | 可以                       | `let mut x; x.trim()`    |
//
// 与英文笔记同义（修正拼写）：
// - `self` — immutable by-value receiver: **takes ownership**; `self` binding not `mut` → 不能给 `self` 整体重赋值，字段修改也受限制（教学上常记成「按值、默认不让你乱改」；要改字段一般用 `mut self`）。
// - `mut self` — mutable by-value receiver: **takes ownership**, and the **`self` parameter binding is `mut`**, so you can mutate fields and/or `return self`.
// - `&self` — immutable reference: **no ownership moved**; shared read.
// - `&mut self` — mutable reference: **no ownership moved**; exclusive write (外层需 `let mut`)。
//
// 选型口诀（入门先这么记，再对照表格与下方故事纠偏）：
// - 调用结束后**还想接着用「外面手上这一份」**（多次只读、或原地改字段）：优先 **`&self` / `&mut self`** —— **所有权不交给方法**，只是**借**出去。
// - **愿意把这一份按值交给方法**（消费进流水线、或变换后由返回值接棒）：用 **`self` / `mut self`** —— 调用时**右侧那份值会 move**；若**没有** `x = x.foo()` 这类接回，外面就**不能**再用原来的绑定里的旧值。
// - 易混点：**`self` 不等于「外面永远不用这个对象」**。常见是 **`self` + 返回 `Self`**，外面写 **`x = x.transform()`**：语义仍是按值进方法，但用**返回值**把所有权接回，故事线继续。
// - **`&mut self` 单独记一档**：外面**仍拥有**，但要**独占可变借用**；外层通常是 **`let mut x`**。
//
// 详细 runnable：**`run()`** 里「单曲发行流水线」故事，演示顺序固定为
// **`self` → `mut self` → `&self` → `&mut self`**（与表格自上而下一致）。
// =============================================================================

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_seconds: u32,
}

impl TaylorSwiftSong {


    fn make_taylor_swift_song(title: String, release_year: u32, duration_seconds: u32) -> TaylorSwiftSong {
        TaylorSwiftSong {
            title,
            release_year,
            duration_seconds,
        }
    }

    // --- 故事线：一张单曲从「手边 Demo」到「可发片」的四幕（接收者顺序：self → mut self → &self → &mut self）---

    /// **① `self`（按值）——第一幕：把唯一一份 Demo 母带签给厂牌**
    ///
    /// 乐理/流程隐喻：歌手手里只有**一份**可签的载体；签字交货时，**旧那份在语义上被收走**（`move`）。
    /// 厂牌回你一张**可下厂的正式母带**（这里用返回 `Self` 接住故事，title 打上 `[Master]` 标记）。
    ///
    /// 调用：`song = song.hand_over_demo_for_pressing();` —— 右侧的 `song` **move** 进本方法。
    fn hand_over_demo_for_pressing(self) -> Self {
        println!(
            "      …厂牌收走签字件：「{}」备案完毕，换发压片用母带。",
            self.title
        );
        Self {
            title: format!("{} [Master]", self.title),
            release_year: self.release_year,
            duration_seconds: self.duration_seconds,
        }
    }

    /// **② `mut self`（按值 + 形参可变）——第二幕：发行科在自家流水线里拆封加料**
    ///
    /// 仍是 **move**：旧 `song` 进流水线；但 **`self` 绑定是 `mut`**，可以在封装前把 hidden track 写进总时长，
    /// 再把**同一条所有权**交回给外层的 `song = …`。
    ///
    /// ### 常见疑问：参数写了 `mut self`，为什么外层还可以是 `let song2`（不要 `mut`）？
    ///
    /// - 这里的 **`mut` 只作用在「方法形参 `self`」这个绑定上**：表示**函数体里**可以把 `self` 当可变变量来改字段。
    /// - 调用方做的是 **按值 `move`**：只要外层**拥有**这份 `TaylorSwiftSong`，`let` 或 **`let mut` 都能 move 进去**——
    ///   **不要求**外层一定是 `let mut`，因为并不是从外层「反复借 `&mut`」。
    /// - **外层必须 `let mut` 的典型情况**：要多次调用 **`&mut self`** 方法（例如连续 `song.engineer_…()`），
    ///   编译器需要从 `song` **借出可变引用**，这时绑定必须是 `mut`。
    /// - **另一个坑**：若写 `song2.add_bonus_hidden_track(2);` 却**不接住返回值**，改好的那份 `Self` 会随函数返回**立刻被 drop**，
    ///   等于白加；应写 `let song2 = song2.add_bonus_hidden_track(2);` 或 `song2 = song2.add_bonus_hidden_track(2)`（且按需 `mut`）。
    fn add_bonus_hidden_track(mut self, extra_seconds: u32) -> Self {
        println!(
            "      …发行科写入 hidden track，总时长 +{} s（`mut self` 里改字段再返回）。",
            extra_seconds
        );
        self.duration_seconds = self.duration_seconds.saturating_add(extra_seconds);
        self
    }

    /// **③ `&self`（不可变借用）——第三幕：媒体通气会，只读母带信息写通稿**
    ///
    /// 记者**不能**把母带揣走，只能借读 `title / year / duration` 写引用文案；`song` 一直留在棚里。
    fn press_kit_quote(&self) -> String {
        format!(
            "《{}》（{}）— 官方口径：总长 {} 秒（母带未离开仓库）。",
            self.title, self.release_year, self.duration_seconds
        )
    }

    /// **④ `&mut self`（可变借用）——第四幕：混音师戴耳机剪掉尾部静音**
    ///
    /// 独占写入口：把尾部空白秒数裁掉；**所有权从未离开**外层 `song`，只是波表被改了。
    fn engineer_snip_tail_silence(&mut self, trim_seconds: u32) {
        println!(
            "      …混音师剪掉尾部静音 {} s（`&mut self` 独占改）。",
            trim_seconds
        );
        self.duration_seconds = self.duration_seconds.saturating_sub(trim_seconds);
    }


    fn is_longger_than(&self, other:Self) -> bool {
        self.duration_seconds > other.duration_seconds
    }

}


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn enum_lab() {

    let directions = Direction::East;
    let match_result = match  directions{
        Direction::North => "向北",
        Direction::South => "向南",
        Direction::East  => "向东",
        Direction::West  => "向西",
    };

    println!("match result: {}", match_result);

}




pub fn run() {
    println!("== Lab ==");

    // 整条故事用同一绑定 `song` 讲完；顺序固定：**self → mut self → &self → &mut self**。
    println!("  [故事] 单曲发行流水线（接收者顺序：`self` → `mut self` → `&self` → `&mut self`）");
    println!("  设定：Taylor 把《Anti-Hero》Demo 交到厂牌 → 发行科加 hidden track → 媒体写通稿（只读）→ 混音师剪尾。");

    let mut song = TaylorSwiftSong::make_taylor_swift_song(String::from("Anti-Hero"), 2022, 200);
    println!("  起点（手边唯一 Demo）：{song:?}");

    println!("\n  [所有权旁证] 只调用、不把返回值接回有用绑定：`TaylorSwiftSong` 会 **move** 进方法，旧变量名不能再读。");
    {
        let draft = TaylorSwiftSong::make_taylor_swift_song(String::from("试压盘"), 2023, 1);
        // ▼ move：`draft` 整块按值交给方法的 `self`（`hand_over_demo_for_pressing(self)`）。
        let _master = draft.hand_over_demo_for_pressing();
        // 想亲眼看到 E0382：请**只取消下一行**的 `//`，再 `cargo build` —— `draft` 已在上一行被 move，不能再读。
        // println!("      还想看草稿？{draft:?}");
        println!("      试压盘：`draft` 已进厂，只剩 `_master`（此处用 `_` 立刻 drop 仅作旁证）。");
    }

    println!("\n  ① `self` —— `hand_over_demo_for_pressing`：签字交带，**旧值整块 move** 进方法，换回 `[Master]` 版");
    println!("      （下面拆成两行：先看 `song` → `incoming`，等价于 `song = song.hand_over_...()` 的「右侧那次 move」。）");
    println!("      若在 `incoming` 赋值后、`song = …` 前误用 `song`，同样会 E0382——此时 `song` 槽位里还没有新值。");
    let incoming = song; // ▼ move：`song` 里的值 move 到 `incoming`（`song` 暂时不可用，直到下面赋值写回）
    song = incoming.hand_over_demo_for_pressing(); // ▼ move：`incoming` → 方法的 `self`；返回的 `Self` 写回 `song`
    println!("      现在棚里这份：{song:?}");

    println!("\n  ② `mut self` —— `add_bonus_hidden_track`：仍是 **按值 move**（`incoming` → 方法的 `mut self`），厂里改时长再 **return self**");
    let incoming = song; // ▼ move：`song` → `incoming`
    song = incoming.add_bonus_hidden_track(37); // ▼ move：`incoming` → `mut self`；返回的 `Self` → `song`
    println!("      加录后：{song:?}");

    println!("\n  ③ `&self` —— `press_kit_quote`：媒体只**借读**写通稿，`song` 一直在");
    println!("      {}", song.press_kit_quote());
    println!("      再问一遍（仍合法）：{}", song.press_kit_quote());

    println!("\n  ④ `&mut self` —— `engineer_snip_tail_silence`：混音师 **独占可变借用**，剪掉尾部静音");
    println!("      剪前 duration_seconds = {}", song.duration_seconds);
    song.engineer_snip_tail_silence(8);
    println!("      剪后 duration_seconds = {}", song.duration_seconds);

    println!("\n  （故事收束：母带仍归 `song`；若要演示 **无返回的 `self` 消费**——例如母带碎纸机——可另写 `fn shred_master(self)` 且不再用 `song`。）");

    let song2 = TaylorSwiftSong::make_taylor_swift_song(
        String::from("Anti-Hero"),
        2015,
        1234
    );

    let longer = song2.is_longger_than(song);
    println!("longer than {}", longer);


    println!("▷ 练习 1：Struct 三种形式");
    println!("  - 写一个具名字段 struct `User`，一个元组结构体 `Meters(f64)`，一个单元结构体");
    println!("  - 给 `User` 派生 `Debug`，再用 `{{user:?}}` / `{{user:#?}}` 打印");

    println!();

    println!("▷ 练习 2：impl 里的四种函数");
    println!("  - 写一个关联函数 `new`");
    println!("  - 写一个 `&self` 方法做查询");
    println!("  - 写一个 `&mut self` 方法做修改");
    println!("  - 写一个 `self` 方法消费实例");
    println!("  - 分别调用，观察各自对实例所有权的影响");

    println!();

    println!("▷ 练习 3：Enum 带数据");
    println!("  - 写一个 `Shape` enum，包含 `Circle(f64)`、`Rect {{w,h}}`、`Point`");
    println!("  - 给它实现 `fn area(&self) -> f64`");
    println!("  - 用 match 穷尽处理；再刻意漏掉一个分支，观察 E0004");
    enum_lab();
    student_lookup_lab();

    println!();

    println!("▷ 练习 4：Option / Result 的组合子");
    println!("  - 写 `fn first_positive(v: &[i32]) -> Option<i32>`");
    println!("  - 写 `fn parse_pair(s: &str) -> Result<(i32, i32), String>`，用 `?` 传播错误");
    println!("  - 用 `unwrap_or_else` 给 Result 一个兜底");

    println!();

    println!("▷ 练习 5：HashMap 典型惯用法");
    println!("  - 给一句话做词频统计：`entry(key).or_insert(0) += 1`");
    println!("  - 用 `iter()` 找出出现次数最多的单词");

    println!();

    println!("▷ 练习 6：模式匹配多场景");
    println!("  - `match` + 范围 `1..=5`");
    println!("  - `match` + 或模式 `1 | 2 | 3`");
    println!("  - `match` + 守卫 `x if x > 0`");
    println!("  - `if let` + `else`");
    println!("  - `while let` 消费 `Vec`");

    println!();

    println!("▷ 练习 7：解构 struct");
    println!("  - 写一个函数签名 `fn describe(User {{ name, age, .. }}: &User)`");
    println!("  - 让 `match` 同时匹配字段具体值和绑定变量");

    println!();

    println!("▷ 练习 8：引用的赋值行为");
    println!("  - 写 `let r1 = &x; let r2 = r1;`，确认 r1/r2 都能用（& 是 Copy）");
    println!("  - 写 `let r1 = &mut x; let r2 = r1;`，确认 r1 被 move，不能再用");

    println!();

    println!("▷ 练习 9：方法链");
    println!("  - 给 `TextBuilder` 加一个 `trim()` 方法（返回 Self）继续链");
    println!("  - 给它加一个 `try_assert_no_spaces()`（返回 Result）");
    println!("  - 写一个 `build_validated() -> Result<TextBuilder, String>` 用 `?` 串联多步");

    println!();

    println!("完成标准：");
    println!("  - 能独立写一个带 `new` + 方法 + Debug 派生的 struct");
    println!("  - 能独立写一个含数据的 enum + 穷尽 match");
    println!("  - 能用 Option/Result 的组合子替代半数以上的 match");
    println!("  - 能指出方法链在什么时候会被 Result 打断，并用两种方式把它接回去");

    println!();
}
