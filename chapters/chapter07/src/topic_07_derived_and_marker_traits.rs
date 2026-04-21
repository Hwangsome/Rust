//! `derive` 与 marker trait：用最少的代码得到最多的能力。
//!
//! ## `#[derive(...)]`
//!
//! 标准库里一部分 trait 可以让编译器**自动生成实现**，这叫 derive（派生）。
//! 常见的可 derive trait：
//!
//! | trait          | 能力                                |
//! |----------------|----------------------------------|
//! | `Debug`        | `{:?}` 打印                           |
//! | `Clone`        | `.clone()` 深拷贝                     |
//! | `Copy`         | 按位复制（只允许"自己和所有字段都是 Copy"的类型）|
//! | `PartialEq` / `Eq` | 相等性比较                         |
//! | `PartialOrd` / `Ord` | 顺序比较                           |
//! | `Hash`         | 哈希（供 `HashMap` / `HashSet` 使用）      |
//! | `Default`      | `T::default()` 创建默认值              |
//!
//! ## Marker trait
//!
//! 没有方法、只"标记"某种性质的 trait：
//!
//! - `Copy`、`Send`、`Sync`、`Sized` 都是标准库里的 marker trait
//! - 你也能自己定义一个，把一组 bound 打包成一个名字
//!
//! 本节演示：
//! 1. derive 一大堆 trait
//! 2. `T::default()` / `a == b` / `Debug` 打印
//! 3. 自定义 marker trait `Resettable`
//! 4. blanket impl（为所有满足条件的类型一次性实现）

use std::fmt::Debug;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Config {
    retries: u8,
    verbose: bool,
}

/// 自定义 marker trait：用来把"能复位并比较的东西"抽象成一个名字。
/// 没有任何方法体。
trait Resettable: Clone + Default + PartialEq {}

/// **Blanket impl**：为所有"恰好满足 Clone + Default + PartialEq"的类型，一次性实现 Resettable。
/// 这个写法让 `impl Resettable for Config {}` 都省了——只要 Config 派生了那三个 trait，
/// 它就自动是 Resettable。
impl<T> Resettable for T where T: Clone + Default + PartialEq {}

/// 使用 marker trait 作为 bound：签名更简洁，不用写一堆 `T: Clone + Default + PartialEq + Debug`。
fn reset_if_needed<T>(value: &mut T)
where
    T: Resettable + Debug,
{
    if *value != T::default() {
        *value = T::default();
    }
    println!("after reset => {value:?}");
}

pub fn run() {
    println!("== Derived and Marker Traits ==");

    println!("-- (1) derive 自动获得一组能力 --");
    let mut config = Config { retries: 3, verbose: true };
    let cloned = config.clone();        // 来自 Clone
    let default_val: Config = Config::default(); // 来自 Default
    println!("Debug:     {config:?}");
    println!("cloned:    {cloned:?}");
    println!("default:   {default_val:?}");
    println!("PartialEq: config == cloned ? {}", config == cloned);
    println!("PartialEq: config == default? {}", config == default_val);
    println!();

    println!("-- (2) marker trait + blanket impl --");
    println!("Config 自动满足 Resettable（因为 derive 了 Clone + Default + PartialEq）");
    reset_if_needed(&mut config);
    println!();

    println!("-- (3) 何时手写 vs derive --");
    println!("  默认都用 derive；");
    println!("  只有在‘相等定义 / 默认值 / 调试输出’需要定制时，才手写 impl。");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Derived Traits
// 			Marker Traits
// -------------------------------------------

trait Properties: PartialEq + Default + Clone {}
#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}
impl Properties for Student {}
fn main() {
    let s_1 = Student {
        name: String::from("ABC"),
        age: 35,
        sex: 'M',
    };

    let s_2 = Student {
        name: String::from("XYZ"),
        age: 40,
        sex: 'M',
    };

    println!("Student: {:?}", s_1);
    println!("s_1 and s_2 are equal: {}", s_1 == s_2);
}

/*
------------------------------------------------------------------------------------------
Concept/Topic        | Explanation
---------------------|--------------------------------------------------------------------
Derived Traits       | The #[derive(..)] attribute automatically implements common traits.
                     | Examples include Debug, PartialEq, Clone, and Default.
                     | This avoids writing manual boilerplate implementations.

Debug and Formatting | The Debug trait enables printing using the {:?} formatter.
                     | The PartialEq enables comparison using ==.

Marker Traits        | Marker traits have no methods and serve as metadata indicators.
                     | They express constraints or properties about a type.
                     | They often include super traits to enforce required capabilities.
------------------------------------------------------------------------------------------
*/
"###;
