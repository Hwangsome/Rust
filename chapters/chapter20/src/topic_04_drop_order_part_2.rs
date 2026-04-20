//! Drop 顺序 part 2：struct / tuple 字段**按声明顺序**依次 drop。

struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  drop field: {}", self.0);
    }
}

struct Holder {
    first: Announce,
    second: Announce,
    third: Announce,
}

pub fn run() {
    println!("== Drop Order (part 2): struct fields ==");
    let _h = Holder {
        first:  Announce("first"),
        second: Announce("second"),
        third:  Announce("third"),
    };
    println!("  leaving scope; expect first → second → third (声明顺序)");
    println!();
}
