//! 初始化 struct 实例：`new`、`Default`、结构更新语法、带校验构造。
//!
//! Rust 没有"语言级构造函数"——`new` 只是一个**约定俗成**的关联函数名。这一设计的好处是：
//!
//! - 一个类型可以有任意多个"构造函数"（`new`、`from_str`、`with_capacity`、`empty`...）
//! - 它们都是普通方法，支持返回 `Result` 做输入校验
//!
//! 本节覆盖 4 种典型写法：
//!
//! 1. `#[derive(Default)]` + `T::default()` —— 所有字段都用类型的默认值
//! 2. 自定义 `fn new(...) -> Result<Self, E>` —— 带校验的构造（失败返回 Err）
//! 3. **结构更新语法** `Thing { field: v, ..other }` —— 只覆盖部分字段，其余复用
//! 4. `unwrap_or_default()` —— Result 失败时回退到默认值

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

// `run()` 是当前主题统一的演示入口。
// `main.rs` 会按章节顺序调用它，所以这里的输出就是读者最先看到的现象。
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
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 		Initializing Struct Instance
// -------------------------------------------

use rust_course::Student;
fn main() {
    let std_1 = Student::new("joseph".to_string()).unwrap_or_default();
    println!("{:?}", std_1);

    let std_2 = Student::default();
    println!("{:?}", std_2);

    let std_3 = Student {
        age: 12,
        ..Default::default()
    };
}


// code in lib

#[derive(Debug, Default)]
pub struct Student {
    id: u8, // add later on
    pub age: u8,
    pub name: String,
}

impl Student {
    // pub fn new(std_name: String) -> Self {
    //     Self {
    //         id: 0,
    //         age: 20,
    //         name: std_name,
    //     }
    // }

    pub fn new(std_name: String) -> Result<Self, String> {
        let x = std_name.chars();
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}


// impl Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             name: "unknown".to_string(),
//             age: 20,
//         }
//     }
// }




/* 
-----------------------------------------------------------------------------------------------------
Concept / Topic      | Explanation
---------------------|-------------------------------------------------------------------------------
Constructor Function | Rust does not provide built-in constructors like many other languages.
                     | A common Rust convention is to define an associated function called new().
                     | It can access private fields as it belongs to the same impl block. 
                     | We can therefore keep fields private. 

Validating Inputs    | The constructor can validate inputs before creating a struct instance.
in Constructors      | If validation succeeds, the function returns the instance inside Ok.
                     | Otherwise, it returns an error indicating invalid input.

Default Trait        | The Default trait provides a way to create instances with predefined values.
                     | It enables the compiler to generates default values for fields.
                     | We can set some fields manually while initializing others from default values.
-------------------------------------------------------------------------------------------------------
*/
"###;
