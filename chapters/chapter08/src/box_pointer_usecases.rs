// 这个文件补充 Box 的两个常见场景：放大对象到堆上，以及存放 trait object。
// 运行时要观察：Box 不只是“堆分配”，它还是一种明确的单一所有权表达。
// 当值很大、或需要把不同具体类型塞进同一个集合时，Box 都很常见。
trait Storage {
    fn description(&self) -> String;
}

struct DiskStorage;
struct CloudStorage;

impl Storage for DiskStorage {
    fn description(&self) -> String {
        "disk".to_string()
    }
}

impl Storage for CloudStorage {
    fn description(&self) -> String {
        "cloud".to_string()
    }
}

pub fn run() {
    println!("== Box Pointer Use Cases ==");

    let boxed_numbers = Box::new([0_u8; 32]);
    println!("boxed array length => {}", boxed_numbers.len());

    let stores: Vec<Box<dyn Storage>> = vec![Box::new(DiskStorage), Box::new(CloudStorage)];
    for store in stores {
        println!("storage => {}", store.description());
    }
    println!();
}
