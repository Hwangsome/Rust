//! `async` / `await` 基础：Rust 的异步模型。
//!
//! 核心抽象：
//!
//! - `async fn` / `async { ... }` 返回一个 `Future`——**代表"未来某时能完成的值"**
//! - `Future` 本身是**惰性**的，不跑——需要一个 **executor / runtime** 来驱动它
//! - `.await` 让当前 async 函数**暂停**直到该 Future 完成
//!
//! 标准库只提供类型 / trait，**没有自带 runtime**——真正执行 async 代码需要 tokio / async-std / smol 等。
//!
//! 本章不引入 tokio 依赖，所以这里用一个最小手写的 block_on 来驱动 Future。

#![allow(unsafe_code)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// 手写一个极简 block_on：持续 poll 直到 Future 返回 Ready。
///
/// 生产里不要这么写——真正的 executor 还需要处理 wake 通知、调度等。
fn block_on<F: Future>(mut future: F) -> F::Output {
    // SAFETY: future 在 block_on 期间不移动；这里 pin 在栈上。
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => {} // 教学用：不休眠，直接 busy-poll
        }
    }
}

fn noop_waker() -> Waker {
    use std::task::{RawWaker, RawWakerVTable};
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| RawWaker::new(std::ptr::null(), &VTABLE),
        |_| {},
        |_| {},
        |_| {},
    );
    // SAFETY: VTABLE 里全是 no-op，不会访问 data pointer。
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

async fn double(x: i32) -> i32 {
    x * 2
}

async fn pipeline(x: i32) -> i32 {
    let a = double(x).await;
    let b = double(a).await;
    b + 1
}

pub fn run() {
    println!("== Async/Await Basics ==");

    let result = block_on(pipeline(5));
    println!("  pipeline(5) = {result} (= 5*2*2 + 1)");

    println!();
    println!("-- 记忆 --");
    println!("  async fn 返回 Future；Future 需要 executor 驱动");
    println!("  真实项目用 tokio: `#[tokio::main] async fn main() {{ ... }}`");
    println!();
}
