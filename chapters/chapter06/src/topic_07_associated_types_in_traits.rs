//! 关联类型（associated types）：让 trait 声明"这个方法会产出哪类数据"。
//!
//! 与泛型参数相比：
//!
//! - `trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }` —— 关联类型
//! - `trait Iterator<T> { fn next(&mut self) -> Option<T>; }` —— 泛型参数
//!
//! 两者核心差异：
//! - **关联类型**：每个实现者**只能选一次** `Item = ?`；调用端看起来就像一个属性
//! - **泛型参数**：每个实现者**可以多次**选不同 `T`，调用端要写 `Iterator::<T>::next`
//!
//! 标准库的 `Iterator`、`Deref`、`Add` 全都用了关联类型——这一节做最小示例，
//! 下一节 `topic_08` 会把两种写法对比着讲"什么时候选哪个"。

#[derive(Debug)]
struct Kmh(u32);

#[derive(Debug)]
struct Mph(u32);

trait DistanceThreeHours {
    /// 关联类型：每个实现者自己决定"距离用什么类型表示"。
    type Distance;

    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    // Kmh 的距离用 u32 表示（公里）
    type Distance = u32;
    fn distance_in_three_hours(&self) -> Self::Distance {
        self.0 * 3
    }
}

impl DistanceThreeHours for Mph {
    // Mph 的距离用 f32 表示（英里）
    type Distance = f32;
    fn distance_in_three_hours(&self) -> Self::Distance {
        (self.0 * 3) as f32
    }
}

/// 一个接收"实现了 DistanceThreeHours 且距离可打印"的泛型函数。
///
/// 用 `T::Distance: std::fmt::Display` 把关联类型继续约束——
/// 这就是关联类型的**第二个价值**：它让 trait 的使用者也能对"输出类型"做 bound。
fn report<T>(label: &str, source: &T)
where
    T: DistanceThreeHours,
    T::Distance: std::fmt::Display,
{
    println!("{label}: 3h 后距离 = {}", source.distance_in_three_hours());
}

pub fn run() {
    println!("== Associated Types in Traits ==");

    println!("-- (1) 同一个 trait，每个实现者选一种距离类型 --");
    let city = Kmh(80);
    let hwy = Mph(65);
    println!("80 km/h for 3h => {} km",   city.distance_in_three_hours());
    println!("65 mph  for 3h => {} miles", hwy.distance_in_three_hours());
    println!();

    println!("-- (2) 通过 T::Distance 继续 bound --");
    report("city", &city);
    report("hwy ", &hwy);
    println!();

    println!("-- (3) 为什么不用 Trait<Distance> 泛型参数 --");
    println!("  对 Kmh 来说，距离只可能是 km 这一种——让调用端再写 <u32> 是噪声");
    println!("  所以 Iterator::Item、Deref::Target 都选了关联类型而不是泛型参数");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// ----------------------------------------------
// 		Associated Types in Traits
// ----------------------------------------------

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

trait DistanceThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}
fn main() {
    let speed_Kmh = Kmh { value: 90 };
    let distance_Km = speed_Kmh.distance_in_three_hours();

    println!(
        "At {:?}, you will travel {:?} in 3 hours",
        speed_Kmh, distance_Km
    );

    let speed_Mph = Mph { value: 90 };
    let distance_Miles = speed_Mph.distance_in_three_hours();
    println!(
        "At {:?}, you will travel {:?}, in 3 hours",
        speed_Mph, distance_Miles
    );
}

/* 
--------------------------------------------------------------------------------------------------
Concept/Topic                | Explanation
-----------------------------|--------------------------------------------------------------------
Associated Types             | Associated types define placeholder types within a trait.
                             | The concrete type is specified by each implementing type.
                             | This allows flexible return types tied to specific impelmentation.

Associated Types vs Generics | Associated types and generics serve different roles.
                             | Generics abstract over types at definition time.
                             | Associated types defer type specification to trait implementations.

Note on Type Keyword         | The `type` keyword inside traits declares associated types.
                             | This differs from type aliases used elsewhere in Rust.
-------------------------------------------------------------------------------------------------
 */
"###;
