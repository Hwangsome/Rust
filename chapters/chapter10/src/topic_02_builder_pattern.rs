//! Builder 模式：当字段多、可选项多、还可能有分支构造逻辑时的推荐写法。
//!
//! 痛点对比：
//!
//! ```text
//! // 不用 builder
//! fn new(name: String) -> Customer
//! fn new_with_membership(name, membership) -> Customer
//! fn new_with_all(name, username, membership, country, age) -> Customer  // 参数位置记不住
//! ```
//!
//! Builder 把"必填字段"作为起点、"可选字段"作为链式方法、"最终转换"用 `build()`：
//!
//! ```text
//! Customer::builder("Alice")
//!     .username("ally")       // 可选
//!     .country("US")           // 可选
//!     .age(30)                 // 可选
//!     .build();                // 终结
//! ```
//!
//! 这里的每个链式方法都返回 `Self`（按值 take-and-give），保持链不断。
//! 如果需要在某些步骤做校验，可以把方法签名改成返回 `Result<Self, E>`——整个链就用 `?` 串。
//!
//! 本 crate 的标准库外没有引入 `derive_builder` 之类的过程宏；这里手写的版本
//! 最能体现 builder 在所有权层面的工作方式。

#[derive(Debug, Clone, Default)]
enum MembershipType {
    #[default]
    New,
    Casual,
    Loyal,
}

#[derive(Debug, Default)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    country: String,
    age: u8,
}

#[derive(Debug, Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    country: Option<String>,
    age: Option<u8>,
}

impl Customer {
    fn builder(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            ..CustomerBuilder::default()
        }
    }
}

impl CustomerBuilder {
    fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    fn membership(mut self, membership: MembershipType) -> Self {
        self.membership = Some(membership);
        self
    }

    fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    fn build(self) -> Customer {
        Customer {
            name: self.name,
            username: self.username.unwrap_or_default(),
            membership: self.membership.unwrap_or_default(),
            country: self.country.unwrap_or_else(|| "unknown".to_string()),
            age: self.age.unwrap_or_default(),
        }
    }
}

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
pub fn run() {
    println!("== Builder Pattern ==");

    let casual_user = Customer::builder("Joseph".to_string())
        .username("joe123")
        .membership(MembershipType::Casual)
        .country("CN")
        .age(28)
        .build();

    let loyal_user = Customer::builder("Micheal".to_string())
        .username("micheal2000")
        .membership(MembershipType::Loyal)
        .build();

    println!("customer built step by step => {:?}", casual_user);
    println!(
        "loyal user summary => name = {}, username = {}, membership = {:?}, country = {}, age = {}",
        loyal_user.name,
        loyal_user.username,
        loyal_user.membership,
        loyal_user.country,
        loyal_user.age
    );
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Builder Pattern
// -------------------------------------------

#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
    new,
    causual,
    loyal,
}

impl Default for Membershiptype {
    fn default() -> Self {
        Membershiptype::new
    }
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            ..Default::default() /* username: None,
                                  * membership: None,
                                  * gender: None,
                                  * country: None,
                                  * age: None, */
        }
    }
    // fn new(name: String) -> Self {
    //     Customer {
    //         name: name,
    //         ..Default::default()
    //     }
    // }

    // fn new_2(name: String, username: String) -> Self {
    //     Customer {
    //         name: name,
    //         username: username,
    //         ..Default::default()
    //     }
    // }

    // fn new_3(name: String, username: String, membership: Membershiptype) -> Self
    // {     Customer {
    //         name: name,
    //         username: username,
    //         membership: membership,
    //         ..Default::default()
    //     }
    // }
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }
    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }
    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }
    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}
fn main() {
    // let new_user = Customer::new("Nouman".to_string());
    // let user_with_login = Customer::new_2("Joseph".to_string(),
    // "joe123".to_string()); let user_with_membership = Customer::new_3(
    //     "Micheal".to_string(),
    //     "micheal2000".to_string(),
    //     Membershiptype::loyal,
    // );

    let new_user = Customer::new("Nouman".to_string()).build();
    let user_with_login = Customer::new("Joseph".to_string())
        .username("joe123".to_string())
        .build();

    let user_with_membership = Customer::new("Micheal".to_string())
        .username("micheal2000".to_string())
        .membership(Membershiptype::loyal)
        .build();
}

/*
--------------------------------------------------------------------------------------------------
Concept / Topic           | Explanation
--------------------------|-----------------------------------------------------------------------
Constructor Proliferation | Complex data structures often require many inputs and optional settings.
                          | This can lead to multiple constructor functions with long argument lists.
                          
Builder Pattern Idea      | The builder pattern simplifies construction of complex objects.
                          | It constructs an object step by step instead of passing many arguments.

Builder Structure         | A separate builder struct is introduced to assist in object construction.
                          | It contains the same fields as the original struct.
                          | Optional fields are stored as Option while mandatory fields remain required.

Builder Setter Methods    | Individual methods are defined to set values for optional fields.
                          | Each method takes a mutable reference to self and returns &mut Self.
                          | Returning the same instance allows methods to be chained together.

Build Method              | The build method finalizes construction and returns the completed struct.
                          | Values are copied from the builder into the final struct instance.                       
--------------------------------------------------------------------------------------------------
*/
"###;
