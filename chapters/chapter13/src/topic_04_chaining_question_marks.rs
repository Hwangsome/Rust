//! `?` 链式错误处理：让方法链和错误传播协同工作。
//!
//! 一个流水线式的 builder：
//!
//! ```ignore
//! Order::new()
//!     .set_item("laptop")?
//!     .set_quantity(2)?
//!     .set_total(1500.0)?
//!     .submit()
//! ```
//!
//! 工作条件：**每一步都要返回 `Result<Self, E>` 或 `Result<&mut Self, E>`**。
//! 其中任何一步失败，`?` 会立刻让整个链提前结束，把错误抛给调用者。
//!
//! 与上一节"返回 `Self`"的非错误版 builder 对比：把 `-> Self` 换成 `-> Result<Self, E>`
//! 就得到了带校验的版本。链的写法几乎不变，只是外部函数要返回 `Result` + 用 `?`。

#[derive(Debug)]
struct Order {
    item_count: u32,
    price_per_item: f64,
    balance: f64,
    completed: bool,
}

#[derive(Debug)]
enum OrderError {
    InvalidOrder,
    PaymentFailed,
    ShippingError,
}

impl Order {
    fn validate(&self) -> Result<&Self, OrderError> {
        if self.item_count == 0 {
            Err(OrderError::InvalidOrder)
        } else {
            Ok(self)
        }
    }

    fn process_payment(&mut self) -> Result<&mut Self, OrderError> {
        let total_price = self.item_count as f64 * self.price_per_item;
        if self.balance < total_price {
            Err(OrderError::PaymentFailed)
        } else {
            self.balance -= total_price;
            Ok(self)
        }
    }

    fn ship_order(&mut self) -> Result<&mut Self, OrderError> {
        if self.item_count > 10 {
            Err(OrderError::ShippingError)
        } else {
            self.completed = true;
            Ok(self)
        }
    }

    fn complete_order(&mut self) -> Result<(), OrderError> {
        self.validate()?;
        self.process_payment()?.ship_order()?;
        Ok(())
    }
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Chaining Question Marks ==");

    let mut success = Order {
        item_count: 5,
        price_per_item: 20.0,
        balance: 200.0,
        completed: false,
    };
    let mut failure = Order {
        item_count: 12,
        price_per_item: 20.0,
        balance: 300.0,
        completed: false,
    };

    println!("successful order => {:?}", success.complete_order());
    println!("order state after success => {:?}", success);
    println!("failing order => {:?}", failure.complete_order());
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// ---------------------------------------------------
// 	   Chaining with Question Mark
//          Prerequisite: Method Chaining Constraints
// ---------------------------------------------------
struct Order {
    item_count: i32,
    price_per_item: f64,
    balance: f64,
    completed: bool,
}

#[derive(Debug)]
enum OrderError {
    InvalidOrder,
    PayementFailed,
    ShippingError,
}

impl Order {
    fn validate(&self) -> Result<&Self, OrderError> {
        if self.item_count == 0 {
            Err(OrderError::InvalidOrder)
        } else {
            Ok(self)
        }
    }
    fn process_payement(&mut self) -> Result<&mut Self, OrderError> {
        let total_price = self.item_count as f64 * self.price_per_item;
        if self.balance < total_price {
            Err(OrderError::PayementFailed)
        } else {
            self.balance -= total_price;
            Ok(self)
        }
    }

    fn ship_order(&mut self) -> Result<&mut Self, OrderError> {
        if self.item_count > 10 {
            Err(OrderError::ShippingError)
        } else {
            self.completed = true;
            Ok(self)
        }
    }

    fn complete_order(&mut self) -> Result<(), OrderError> {
        self.validate()?;
        self.process_payement()?.ship_order()?;
        Ok(())
    }
}
fn main() {
    let mut order = Order {
        item_count: 5,
        price_per_item: 20.0,
        balance: 200.0,
        completed: false,
    };

    match order.complete_order() {
        Ok(_) => println!("Order completed successfully"),
        Err(e) => println!("Order processing failed: {:?}", e),
    }
}

/*
----------------------------------------------------------------------------------------------------
Concept/Topic               | Explanation
----------------------------|-----------------------------------------------------------------------
Method Chaining with ?      | Improve readability by chaining multiple fallable operations.

Chaining with ? constraints | Relies on Rust’s method chaining principles.
                            | Requires compatible return types between chained calls.
                            | Must respect borrowing and mutability constraints.
----------------------------------------------------------------------------------------------------
 */
"###;
