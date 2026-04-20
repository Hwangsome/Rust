//! 方法链的错误处理约束：回到第 3 章那节"方法链的形状"，加入错误处理维度。
//!
//! 扩展 builder 的维度：
//!
//! ```text
//!  返回类型                       |  链能直接 .next() 吗？
//! ------------------------------|------------------------
//!  &mut Self                    |  ✅ 直接链
//!  Self                         |  ✅ 直接链（按值 take-and-give）
//!  Result<&mut Self, E>          |  ❌ 必须用 ? 或 .and_then / match
//!  Result<Self, E>               |  ❌ 同上
//!  Option<&mut Self>             |  ❌ 必须解开 Option
//! ```
//!
//! 本节用 `BankAccount::deposit / withdraw` 演示：
//! - `deposit` 返回 `&mut Self` 可直接链
//! - `withdraw` 返回 `Result<&mut Self, E>` 需要 `?` 才能继续链
//! - 调用链外层函数因此也要返回 `Result`，才能用 `?`

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Method Chaining Constraints
// -------------------------------------------
#[derive(Debug)]
struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    fn new(owner: String, initial_balance: i32) -> Self {
        println!("Account opened successfully");
        Self {
            balance: initial_balance,
            owner,
        }
    }

    fn change_owner(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }

    fn check_balance(&self) {
        println!("{}'s balance: ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        println!("Deposited ${} to {}'s account", amount, self.owner);
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
            println!("withdrew ${} from {}'s account", amount, self.owner);
        } else {
            println!(
                "Insufficient funds for withdrawl in {}'s account",
                self.owner
            );
        }
        self
    }

    fn view_owner(&self) -> &Self {
        println!("Account owner: {}", self.owner);
        self
    }
}

/*
---------------------------------------------------------------
Self type   | Methods & Notes
----------- | -------------------------------------------------
self        | change_owner(mut self, .. ) -> Self

&self       | check_balance(&self)
            | view_owner(&self) -> &Self

&mut self   | deposit(&mut self, .. ) -> &mut Self
            | withdraw(&mut self, ..) -> &mut Self

No self     |  new(String, i32) -> Self
---------------------------------------------------------------
*/

// Method Chaining: depends on how each method receives and return back self.
fn main() {
    let mut account = BankAccount::new(String::from("Micheal"), 4_000);
    // 1. Methods that does not return anything
    // Methods returning nothing cannot be chained further to grow the chain.
    account.check_balance();

    // 2. Methods that return a &mut Self
    // &mut Self -> chained with methods requiring &mut self or &self
    account.deposit(100).withdraw(50).view_owner();

    // 3. Methods that return &Self
    // &Self -> chained with methods requiring &self
    account.view_owner().check_balance();

    // 4. Method that retrun an owned form of Self
    // Self -> chained with methods accepting any of the three forms of self
    account
        .change_owner(String::from("new_owner"))
        .change_owner(String::from("another_owner"))
        .deposit(100)
        .view_owner();

    //println!("account: {:?}", account);
    // Method chaining works:
    // A() . B() (The output from method A() conforms to the input of B())
}

/*
--------------------------------------------------------------------------------------------
Concept / Topic            | Explanation / Details
---------------------------| ---------------------------------------------------------------
Method Chaining in Rust    | - Allows consecutive method calls on the same instance.
                           | - Validity of chain depends on how each method receives and returns  self.
                           | - Chaining works when output of a method matches the input requirement of the next.

Forms of  self  in Methods | - Methods can take self by ownership, by immutable reference, or by mutable reference.

Summary of Chaining Rules  | - Self  (owned) → can chain with all three (self ,  &self ,  &mut self).
                           | - &mut Self  → can chain with methods taking  &mut self  or  &self.
                           | - &Self  → can only chain with methods taking  &self.
                           | - Methods returning nothing, cannot be chained further.
 -------------------------------------------------------------------------------------------
*/

/*
----------------------------------------------------------------------------------------------
Form of Self         | Can Chain To methods   | Can Chain to  methods  | Can Chain To methods
                     | requiring self         | requiring &self        | requiring &mut self
-------------------- | ---------------------  | ---------------------- |----------------------
Self                 | ✅ Yes                 | ✅ Yes                | ✅ Yes 
&Self                | ❌ No                  | ✅ Yes                | ❌ No
&mut Self            | ❌ No                  | ✅ Yes                | ✅ Yes
----------------------------------------------------------------------------------------------
 */
"###;
