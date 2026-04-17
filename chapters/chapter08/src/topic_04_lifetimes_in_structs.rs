// 这个文件演示“带引用字段的 struct”为什么必须写生命周期。
// 运行时要观察：`ArrayProcessor` 并不拥有数据，它只是临时借来一段切片做处理。
// 只要底层数组还活着，这个 struct 就能安全工作。
struct ArrayProcessor<'a> {
    data: &'a [i32],
}

impl<'a> ArrayProcessor<'a> {
    fn new(data: &'a [i32]) -> Self {
        Self { data }
    }

    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }

    fn update_data(&mut self, data: &'a [i32]) {
        self.data = data;
    }
}

pub fn run() {
    println!("== Lifetimes in Structs ==");

    let first = [1, 2, 3];
    let second = [4, 5, 6];
    let mut processor = ArrayProcessor::new(&first);

    println!("first slice sum => {}", processor.sum());
    processor.update_data(&second);
    println!("updated slice sum => {}", processor.sum());
    println!();
}
