// 这个文件把“方法链”和“错误传播”接到一起看。
// 运行时要观察：只要每一步都返回兼容的 `Result<&mut Self, E>`，就能继续链式写下去。
// 一旦某一步失败，`?` 会立刻让整个流程提前返回。
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
