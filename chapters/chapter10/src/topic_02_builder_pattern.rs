// 这个文件演示 builder pattern 解决什么问题：当 struct 字段多、而且很多字段是可选项时，
// 与其堆很多 `new_xxx()` 重载，不如一步一步构造。
// 运行时要观察：builder 把“必填字段”和“可选字段”拆开了，调用端也更容易读。
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
