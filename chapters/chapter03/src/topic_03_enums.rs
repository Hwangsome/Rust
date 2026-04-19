//! Enum：表达"这个值同时只能处于若干状态之一"。
//!
//! Rust 的 enum 比 C/Java 的 enum 强大得多——它的每个分支可以**携带自己的数据**，
//! 这让它能当"代数数据类型"（algebraic data type）用。`Option<T>`、`Result<T, E>`
//! 其实就是标准库里的 enum。
//!
//! 本节演示：
//! 1. 没有字段的简单 enum（最像其他语言的 enum）
//! 2. 每个分支携带不同数据的 enum（真正的强项）
//! 3. 在 `impl` 里给 enum 加方法
//! 4. `match` 对 enum 做**穷尽性检查**（exhaustiveness check）——漏写分支时编译失败

// 1) 简单 enum：只列分支，没有数据。
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 2) 带数据的 enum：每个分支的"形状"可以不同。
///
/// - `Quit`：没有数据
/// - `Move { x, y }`：带命名字段（就像内嵌的具名 struct）
/// - `Write(String)`：带单个 String（像元组结构体）
/// - `ChangeColor(i32, i32, i32)`：带多个字段
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 3) 给 enum 加方法：和 struct 一样写 impl。
impl TrafficLight {
    /// 把每种信号灯映射成"该做什么"的文字。
    fn describe(&self) -> &'static str {
        match self {
            TrafficLight::Red => "停止",
            TrafficLight::Yellow => "准备",
            TrafficLight::Green => "通行",
        }
    }

    /// 返回下一个信号灯——这是"状态机"风格建模的典型场景。
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

impl Message {
    /// 用一个方法统一处理"任意一种 Message"——enum 的典型价值点。
    fn handle(&self) {
        match self {
            Message::Quit => println!("[Quit] 退出"),
            Message::Move { x, y } => println!("[Move] 移动到 ({x}, {y})"),
            Message::Write(text) => println!("[Write] 写入文本: {text}"),
            Message::ChangeColor(r, g, b) => println!("[ChangeColor] RGB({r}, {g}, {b})"),
        }
    }
}

#[derive(Debug)]
struct Credential {
    user_name: String,
    pass_word: String,
}

/// 演示 Enum 可以有值
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(Credential),
    DebitCard(Credential),
    PayPal(Credential)
}

fn print_payment_enum(payment_method_type: PaymentMethodType) {
    // 若只 `println!("{:?}", …)`：`derive(Debug)` 的读字段**不算**进 `dead_code` 分析，
    // 元组变体里的负载仍可能被报未使用。这里用 `match` **真正用到**负载字段。
    //
    // `match &payment_method_type` 里的 **`&`**：对枚举做**不可变借用**，匹配的是 **`&PaymentMethodType`**。
    // - 这样**不会**在分支里把变体里的 `Credential` **move** 出来；各分支里的 `s` 类型是 **`&Credential`**（透过引用解构）。
    // - 若写成 `match payment_method_type` 且分支为 `CreditCard(s)` 且按值拿走 `s`，容易**部分 move**，
    //   后面这行 `println!("{payment_method_type:?}")` 就可能无法再合法打印整颗枚举。
    // - 小结：**`match &x` = 只读拆解；`match x` = 常配合按值拿走负载**（看你是否还要用 `x`）。
    let detail = match &payment_method_type {
        PaymentMethodType::CreditCard(s) => format!(
            "信用卡尾号/标识: {} | 密码字段长度: {}",
            s.user_name,
            s.pass_word.len()
        ),
        PaymentMethodType::DebitCard(s) => format!(
            "借记卡: {} | 密码字段长度: {}",
            s.user_name,
            s.pass_word.len()
        ),
        PaymentMethodType::PayPal(s) => format!(
            "PayPal 账户: {} | 密码字段长度: {}",
            s.user_name,
            s.pass_word.len()
        ),
    };
    println!("PaymentMethod: {payment_method_type:?} → {detail}");
}

/// 通过 **`&mut PaymentMethodType`** 原地改枚举：所有权仍在外层，只把**可变借用**交进来。
///
/// 写法要点：`match payment_method_type`（类型已是 `&mut …`）时，元组变体里的 `s` 会被推断为 **`&mut Credential`**，
/// 可对 `user_name`、`pass_word` 等字段做 `push_str` 等，**不必**先 `move` 出整颗枚举。
/// 若要把整颗枚举换成**另一种变体**（例如从卡切到 PayPal），可写：
/// `*payment_method_type = PaymentMethodType::PayPal(Credential { user_name: "x@mail.com".into(), pass_word: "".into() });`
fn update_payment_enum(payment_method_type: &mut PaymentMethodType) {
    match payment_method_type {
        PaymentMethodType::CreditCard(s) => {
            s.user_name.push_str(" [卡面已更新]");
            s.pass_word.push_str(" [密钥已轮换]");
        }
        PaymentMethodType::DebitCard(s) => {
            s.user_name.push_str(" [卡面已更新]");
            s.pass_word.push_str(" [密钥已轮换]");
        }
        PaymentMethodType::PayPal(s) => {
            s.user_name.push_str(" [邮箱已验证]");
            s.pass_word.push_str(" [令牌已刷新]");
        }
    }
}

// 肉
#[derive(Debug)]
enum Meat {
    Beef,
    Pork,
    Chicken,
}
// 蔬菜
#[derive(Debug)]
enum Vegetable {
    Tomato,
    Lettuce,
    Onion,
}
// 甜点
#[derive(Debug)]
enum Dessert {
    IceCream,
    Cake,
}
// 餐厅物品
enum RestarantItem {
    Burriot(Meat),
    Salad(Vegetable),
    Dessert(Dessert),   
}

impl RestarantItem {
    fn describe(&self) -> String {
        match self {
            RestarantItem::Burriot(meat) => format!("肉: {meat:?}"),
            RestarantItem::Salad(vegetable) => format!("蔬菜: {vegetable:?}"),
            RestarantItem::Dessert(dessert) => format!("甜点: {dessert:?}"),
        }
    }
}


pub fn run() {
    println!("== Enums ==");

    println!("-- (1) 简单 enum + match --");
    let light = TrafficLight::Green;
    println!("当前是 {light:?}: {}", light.describe());
    let next = light.next();
    println!("下一个状态: {next:?}: {}", next.describe());
    println!();

    println!("-- (2) match 的穷尽性检查 --");
    // 如果在下面的 match 里漏掉一个分支，编译器会报：
    // error[E0004]: non-exhaustive patterns: `TrafficLight::Yellow` not covered
    let example = TrafficLight::Red;
    match example {
        TrafficLight::Red => println!("红灯"),
        TrafficLight::Yellow => println!("黄灯"),
        TrafficLight::Green => println!("绿灯"),
    }
    println!();

    println!("-- (3) 每个分支携带不同数据 --");
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages.iter() {
        msg.handle();
    }
    println!();

    // 练习：按值打印（`print_payment_enum` 拿走所有权）
    print_payment_enum(PaymentMethodType::CreditCard(Credential {
        user_name: String::from("Yellow"),
        pass_word: String::from("***"),
    }));
    print_payment_enum(PaymentMethodType::DebitCard(Credential {
        user_name: String::from("Yellow"),
        pass_word: String::from("***"),
    }));
    print_payment_enum(PaymentMethodType::PayPal(Credential {
        user_name: String::from("Yellow"),
        pass_word: String::from("***"),
    }));

    println!("-- (4) `&mut Enum`：原地改负载（`update_payment_enum`）--");
    let mut wallet = PaymentMethodType::CreditCard(Credential {
        user_name: String::from("4111****"),
        pass_word: String::from("***"),
    });
    println!("  更新前: {wallet:?}");
    update_payment_enum(&mut wallet);
    println!("  更新后: {wallet:?}");

    println!("-- (5) 餐厅物品（遍历子枚举全部变体）--");
    for meat in [Meat::Beef, Meat::Pork, Meat::Chicken] {
        println!("  {}", RestarantItem::Burriot(meat).describe());
    }
    for veg in [Vegetable::Tomato, Vegetable::Lettuce, Vegetable::Onion] {
        println!("  {}", RestarantItem::Salad(veg).describe());
    }
    for d in [Dessert::IceCream, Dessert::Cake] {
        println!("  {}", RestarantItem::Dessert(d).describe());
    }

    let lunch = RestarantItem::Burriot(Meat::Beef).describe();
    println!("午餐: {lunch}");
    let dinner = RestarantItem::Salad(Vegetable::Tomato).describe();
    println!("晚餐: {dinner}");
    let dessert = RestarantItem::Dessert(Dessert::IceCream).describe();
    println!("甜点: {dessert}");
}
