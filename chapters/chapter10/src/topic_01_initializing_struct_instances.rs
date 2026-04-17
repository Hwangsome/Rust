// 这个文件演示三种常见初始化方式：自定义 new、Default、以及结构更新语法。
// 运行时要观察：Rust 没有语言级构造函数，但约定俗成会用关联函数 `new()` 表达创建逻辑。
// 当字段很多、部分值有默认配置时，Default 会明显减轻样板代码。
#[derive(Debug, Default)]
struct Student {
    id: u8,
    age: u8,
    name: String,
}

impl Student {
    fn new(name: String) -> Result<Self, String> {
        if name.chars().all(|ch| ch.is_ascii_alphabetic()) {
            Ok(Self {
                id: 0,
                age: 20,
                name,
            })
        } else {
            Err("name should contain only ascii letters".to_string())
        }
    }
}

pub fn run() {
    println!("== Initializing Struct Instances ==");

    let validated = Student::new("Alice".to_string()).unwrap_or_default();
    let partially_customized = Student {
        age: 18,
        ..Student::default()
    };

    println!("validated student => {:?}", validated);
    println!("default + update syntax => {:?}", partially_customized);
    println!(
        "observe private fields inside module => id = {}, age = {}, name = {}",
        validated.id, validated.age, validated.name
    );
    println!();
}
