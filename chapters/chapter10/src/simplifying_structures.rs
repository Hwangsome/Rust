// 这个文件演示“把一个很大的 struct 拆成几个更小的部分”为什么能缓解借用问题。
// 运行时要观察：拆分之后，不同子结构可以分别借用，编译器更容易判断它们互不冲突。
// 这既是借用层面的收益，也是建模层面的收益。
#[derive(Debug)]
struct DocumentState {
    meta: Meta,
    stats: Stats,
}

#[derive(Debug)]
struct Meta {
    title: String,
}

#[derive(Debug)]
struct Stats {
    reads: u32,
    likes: u32,
}

fn read_title(meta: &Meta) -> &str {
    &meta.title
}

fn bump_likes(stats: &mut Stats) {
    stats.likes += 1;
}

fn total_engagement(stats: &Stats) -> u32 {
    stats.reads + stats.likes
}

pub fn run() {
    println!("== Simplifying Structures ==");

    let mut doc = DocumentState {
        meta: Meta {
            title: "Rust Notes".to_string(),
        },
        stats: Stats {
            reads: 10,
            likes: 2,
        },
    };

    let title = read_title(&doc.meta);
    bump_likes(&mut doc.stats);
    let engagement = total_engagement(&doc.stats);

    println!("title = {title}, engagement = {engagement}");
    println!();
}
