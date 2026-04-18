//! 泛型：把"结构相同、类型不同"的代码合并成一套模板。
//!
//! 泛型的关键直觉：**类型参数**就像函数的**值参数**，只不过它在编译期被"填入"具体类型。
//! 编译器会对每个实际使用的类型组合生成一份**特化的代码副本**——这叫 **monomorphization（单态化）**。
//!
//! 好处：
//! - 运行时**零成本**（调用泛型和调用普通函数一样快）
//! - 类型安全（不像模板字符串替换那样容易出错）
//!
//! 代价：
//! - 二进制体积可能增加（因为"每种用法一份代码"）
//!
//! 本节覆盖 5 个子场景：
//! 1. 泛型 struct（一个 + 两个类型参数）
//! 2. 泛型 impl 块
//! 3. 部分特化 impl：为某个具体类型额外加方法
//! 4. 泛型函数（free function）
//! 5. 泛型方法与 mixup：类型参数可以只出现在方法签名上

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// 泛型 impl：impl<T, U> 里声明的参数必须和 `Point<T, U>` 对应。
impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    /// 方法里**再引入**新的类型参数 V/W：mixup 的泛型独立于 struct 本身。
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

// 部分特化：只有当 T = i32 且 U = i32 时才拥有 `manhattan` 方法。
impl Point<i32, i32> {
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// 泛型自由函数 + trait bound（下一节详细讲，这里先感性看）。
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T {
    let mut best = slice[0];
    for &item in &slice[1..] {
        if item > best {
            best = item;
        }
    }
    best
}

pub fn run() {
    println!("== Generics ==");

    println!("-- (1) 同一个 Point 模板承载不同类型 --");
    let int_point: Point<i32, i32> = Point::new(3, 5);
    let str_float_point: Point<&str, f64> = Point::new("left", 4.5);
    println!("int_point        = {int_point:?}");
    println!("str_float_point  = {str_float_point:?}");
    println!();

    println!("-- (2) 方法里再开泛型参数：mixup --");
    // 这里 int_point 是 `Point<i32, i32>`，Point::new("left", 4.5) 是 `Point<&str, f64>`，
    // mixup 返回 `Point<i32, f64>`——左边取 self.x 的类型，右边取 other.y 的类型。
    let mixed = Point::new(3, 5).mixup(Point::new("left", 4.5));
    println!("mixed = {mixed:?}");
    println!();

    println!("-- (3) 部分特化：只有 Point<i32, i32> 才有 manhattan --");
    let origin_ish = Point::new(-3, 4);
    println!("manhattan({origin_ish:?}) = {}", origin_ish.manhattan());
    // let float_point = Point::new(1.0, 2.0);
    // float_point.manhattan();  // ← 会 E0599：Point<f64, f64> 上没有 manhattan
    println!();

    println!("-- (4) 泛型自由函数 largest --");
    println!("largest(&[1,5,2,9,3])        = {}", largest(&[1, 5, 2, 9, 3]));
    println!("largest(&[\"banana\",\"apple\",\"cherry\"]) = {:?}", largest(&["banana", "apple", "cherry"]));
    println!();

    println!("-- (5) 单态化：编译器会为每个使用到的 T 生成一份代码 --");
    println!("  largest::<i32>   和 largest::<&str>  是两份独立代码");
    println!("  零运行期开销，但可能增加二进制体积");
    println!();
}
#[allow(dead_code)]
const ORIGINAL_COURSE_SOURCE: &str = r###"
// -------------------------------------------
// 			Generics
// -------------------------------------------

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }

    fn new_1(x: i32, y: i32) -> Point<i32, i32> {
        Point { x, y }
    }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }
}

fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn add_points_i32(p1: &Point<i32, i32>, p2: &Point<i32, i32>) -> Point<i32, i32> {
    unimplemented!();
}

fn add_points_f64(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> Point<f64, f64> {
    unimplemented!();
}

fn main() {
    let origin = Point::new(0, 0);
    let p1 = Point::new(1.0, 4.0);

    let p2 = Point::new(5, 5.0);

    origin.printing();
    // p1.printing();

    add_points(&origin, &origin); // add_points_i32(&origin, &origin);
    add_points(&p1, &p1); // add_points_f64(&p1, &p1);
}
/* 
-----------------------------------------------------------------------------------------------------------------
Concept/Topic           | Explanation
------------------------|----------------------------------------------------------------------------------------
Generics Fundamentals   | Generics allow defining types and functions with placeholders for concrete types.
                        | They enable reusable and flexible abstractions.
                        | Generic parameters are declared inside angle brackets after the type or function name.

Generic Structs         | Struct fields can use generic parameters instead of concrete types.
                        | A single generic for all fields enforces the same type across fields.
                        | Difference generics for each field allow fields to have different types.

Generics in impl Blocks | Implementation blocks must explicitly declare generic parameters.
                        | For instance, impl<T, U> struct<T, U>.

Concrete impl           | impl blocks with concrete realizations for generics allow behavior for specific types.
                        | Duplicate method names within overlapping impls are not allowed.

Generic Free Functions  | Functions can also declare generic parameters independently of structs.
                        | Rust generates concrete versions of generic code at compile time (monomorphization).
                        | Monomorphization incurs no runtime cost but may increase binary size.
-----------------------------------------------------------------------------------------------------------------
*/
"###;
