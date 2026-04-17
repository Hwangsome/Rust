// 这个文件对比“关联类型”和“泛型参数”的取舍。
// 运行后要观察：同一个左值类型 `Score` 可以对不同右值类型写多个泛型 trait 实现，
// 但 `Bag` 的“元素类型”只有一种，更适合写成关联类型。
trait PeekableCollection {
    type Item;

    fn first(&self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct Bag(Vec<&'static str>);

impl PeekableCollection for Bag {
    type Item = &'static str;

    fn first(&self) -> Option<Self::Item> {
        self.0.first().copied()
    }
}

#[derive(Debug, Clone, Copy)]
struct Score(u32);

trait Combine<Rhs> {
    type Output;

    fn combine(self, rhs: Rhs) -> Self::Output;
}

impl Combine<Score> for Score {
    type Output = u32;

    fn combine(self, rhs: Score) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Combine<u32> for Score {
    type Output = u32;

    fn combine(self, rhs: u32) -> Self::Output {
        self.0 + rhs
    }
}

pub fn run() {
    println!("== Choosing Associated vs Generic Type ==");

    let bag = Bag(vec!["rust", "trait", "iterator"]);
    println!("associated type picks one item kind => {:?}", bag.first());

    let score = Score(10);
    println!("generic rhs with Score => {}", score.combine(Score(5)));
    println!("generic rhs with u32 => {}", score.combine(2_u32));
    println!();
}
