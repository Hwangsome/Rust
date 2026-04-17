// 这一章的练习重点是：把“能调用”和“能遍历”都看成 trait 驱动的能力，而不是单纯语法。
// 如果练习里卡住，多回看每个函数签名到底拿的是所有权、共享引用还是可变引用。
pub fn run() {
    println!("== Lab ==");
    println!("1. 写一个闭包，捕获外部阈值并判断字符串长度");
    println!("2. 写一个接收 fn 指针的函数，并传入普通函数测试");
    println!("3. 为一个自定义 struct 实现 Iterator 或 IntoIterator");
    println!("4. 对 Vec 同时试验 iter、iter_mut、into_iter");
    println!("5. 用 filter + map + collect 组合出一个新集合");
    println!("6. 用 Option::into_iter 或 flatten 处理可选值");
    println!();
}
