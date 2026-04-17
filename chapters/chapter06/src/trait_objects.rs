// 这个文件演示 trait object：当一组值的具体类型不同，但行为接口相同时，可以放进同一个集合。
// 运行时要观察：`Vec<Box<dyn Draw>>` 里既能放按钮，也能放标签。
// 代价是调用会走动态分发，而不是编译期静态分发。
trait Draw {
    fn draw(&self) -> String;
}

struct Button {
    label: &'static str,
}

struct Label {
    text: &'static str,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("button: {}", self.label)
    }
}

impl Draw for Label {
    fn draw(&self) -> String {
        format!("label: {}", self.text)
    }
}

pub fn run() {
    println!("== Trait Objects ==");

    let widgets: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "Submit" }),
        Box::new(Label { text: "Ready" }),
    ];

    for widget in widgets {
        println!("{}", widget.draw());
    }
    println!();
}
