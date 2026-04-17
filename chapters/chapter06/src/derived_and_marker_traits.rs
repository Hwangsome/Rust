// 这个文件演示两件事：`derive` 能自动补常见 trait，marker trait 用来表达“满足某组性质”。
// 运行后要观察：`Config` 不用手写 Debug / Clone / Default / PartialEq，也能直接使用。
// marker trait 没有方法，但它仍然能作为约束把多个 trait 绑定在一起。
#[derive(Debug, Clone, Default, PartialEq)]
struct Config {
    retries: u8,
    verbose: bool,
}

trait Resettable: Clone + Default + PartialEq {}

impl<T> Resettable for T where T: Clone + Default + PartialEq {}

fn reset_if_needed<T>(value: &mut T)
where
    T: Resettable + std::fmt::Debug,
{
    if *value != T::default() {
        *value = T::default();
    }
    println!("after reset => {:?}", value);
}

pub fn run() {
    println!("== Derived and Marker Traits ==");

    let mut config = Config {
        retries: 3,
        verbose: true,
    };
    println!("before reset => {:?}", config);
    reset_if_needed(&mut config);
    println!();
}
