//! 线程与所有权：`move` 关键字 + `Send` / `Sync` trait。
//!
//! 传给 `thread::spawn(F)` 的 F 必须满足 `F: Send + 'static`：
//! - `'static`：闭包不能借用**比 static 还短寿的**外部引用
//! - `Send`：闭包（包括它捕获的所有东西）可以跨线程"寄过去"
//!
//! 解决方法：用 `move` 关键字把捕获变量**按值**搬进闭包。

use std::thread;

pub fn run() {
    println!("== Ownership in Threads ==");

    println!("-- (1) 没有 move：闭包借用外部 → 编译错误 --");
    // 下面这段如果取消 move 会编译失败（E0373）：
    // let v = vec![1, 2, 3];
    // thread::spawn(|| println!("{v:?}")).join().ok();

    println!("-- (2) 用 move 把所有权搬进闭包 --");
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // v 的所有权已经 move 进这个闭包
        println!("  child sees v = {v:?}");
    });
    handle.join().ok();
    // println!("{v:?}"); // ← 在 main 线程这里 v 已经失效，编译失败
    println!();

    println!("-- (3) Send + Sync 的基本规则 --");
    println!("  Send：所有权可以跨线程转移（大多数类型都是）");
    println!("  Sync：&T 可以跨线程共享");
    println!("  Rc<T> 不是 Send，Arc<T> 是 → 跨线程要用 Arc");
    println!("  RefCell<T> 不是 Sync，Mutex<T> / RwLock<T> 是");
    println!();
}
