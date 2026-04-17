// 这个文件演示 Iterator trait 的核心：只要实现 `next()`，就能被 `for` 循环消费。
// 运行时要观察：`for record in records` 看起来简单，但背后其实反复调用了 `next()`。
// 这也是很多标准库适配器能够工作的基础。
#[derive(Debug)]
struct EmployeeRecords {
    records: Vec<String>,
    index: usize,
}

impl EmployeeRecords {
    fn new(records: Vec<String>) -> Self {
        Self { records, index: 0 }
    }
}

impl Iterator for EmployeeRecords {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.records.len() {
            None
        } else {
            let current = self.records[self.index].clone();
            self.index += 1;
            Some(current)
        }
    }
}

pub fn run() {
    println!("== Iterators ==");

    let records = EmployeeRecords::new(vec![
        "Ada".to_string(),
        "Grace".to_string(),
        "Linus".to_string(),
    ]);

    for record in records {
        println!("record => {}", record);
    }
    println!();
}
