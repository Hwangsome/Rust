// 这个文件演示 IntoIterator：它决定“一个自定义类型怎样被展开成迭代器”。
// 运行时要观察：`for song in playlist` 并不是语言魔法，而是调用了 `into_iter()`。
// 当类型想支持 `for` 循环时，通常要实现的就是这个 trait。
struct Playlist {
    songs: Vec<String>,
}

impl Playlist {
    fn new(songs: Vec<String>) -> Self {
        Self { songs }
    }
}

impl IntoIterator for Playlist {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.songs.into_iter()
    }
}

pub fn run() {
    println!("== IntoIterator ==");

    let playlist = Playlist::new(vec![
        "Borrow Checker Blues".to_string(),
        "Trait Bound Jam".to_string(),
    ]);

    for song in playlist {
        println!("song => {}", song);
    }
    println!();
}
