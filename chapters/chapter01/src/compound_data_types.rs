// 这一节说明“单个值不够时，怎样把多个值组合在一起”。
// 这里先只讲 tuple 和 array，因为它们最能体现“固定结构”的特点。
pub fn run() {
    println!("== Compound Data Types ==");

    // tuple 可以把不同类型的值放在一起，适合演示“按位置访问”的数据组合。
    let user_profile: (&str, i32, char) = ("alice", 20, 'A');
    println!(
        "tuple => name: {}, age: {}, grade: {}",
        user_profile.0, user_profile.1, user_profile.2
    );

    // array 要求元素类型一致，而且长度固定。
    let scores: [i32; 5] = [90, 85, 88, 92, 95];
    println!("array 第一个元素 = {}", scores[0]);
    println!("array 长度 = {}", scores.len());
    println!();
}
