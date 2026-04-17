// 这个文件回到“方法链的约束”本身，只是这次把重点放到错误处理前置条件上。
// 运行时要观察：返回 `&mut Self` 的方法可以继续往下链，但一旦返回 `Result<&mut Self, E>`，
// 链的主体就变成了 Result，后续需要 match、`?` 或适配器继续处理。
#[derive(Debug)]
struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    fn new(owner: String, initial_balance: i32) -> Self {
        Self {
            balance: initial_balance,
            owner,
        }
    }

    fn change_owner(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
        }
        self
    }

    fn view_owner(&self) -> &Self {
        println!("owner => {}", self.owner);
        self
    }

    fn try_min_balance(&mut self, required: i32) -> Result<&mut Self, &'static str> {
        if self.balance >= required {
            Ok(self)
        } else {
            Err("balance is below the required threshold")
        }
    }
}

pub fn run() {
    println!("== Method Chaining Constraints ==");

    let mut account =
        BankAccount::new("Micheal".to_string(), 4_000).change_owner("New Owner".to_string());

    account.deposit(100).withdraw(50).view_owner();
    println!("balance after plain chain => {}", account.balance);

    match account.try_min_balance(10_000) {
        Ok(account) => println!("result chain can continue => {}", account.balance),
        Err(error) => println!("result changes chain shape => {}", error),
    }

    println!();
}
