//! 小项目：用线程并行"抓取"多个 URL（**模拟**，不发真实网络请求）。
//!
//! 关键点：
//! - 用 `thread::spawn` + `mpsc::channel` 收结果
//! - 每个任务返回 `Result<Vec<u8>, String>`，避免一个失败拖垮全部
//! - **真实场景**下会用 `reqwest` 发 HTTP 请求，再用 tokio 或 rayon 并发
//!
//! 为了让本章 `cargo run` 一键通过，不发真实网络，这里用 `sleep + 伪造数据`模拟耗时下载。

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn fake_fetch(url: &str) -> Result<String, String> {
    // 模拟"每个 URL 下载耗时 20ms"，真实代码替换为 reqwest::blocking::get
    thread::sleep(Duration::from_millis(20));
    if url.starts_with("http") {
        Ok(format!("<html>contents of {url}</html>"))
    } else {
        Err(format!("invalid url: {url}"))
    }
}

pub fn run() {
    println!("== Mini Project: parallel fetch (simulated) ==");

    let urls = vec![
        "http://a.example".to_string(),
        "http://b.example".to_string(),
        "http://c.example".to_string(),
        "http://d.example".to_string(),
        "bad_url".to_string(),
    ];

    let start = Instant::now();
    let (tx, rx) = mpsc::channel::<(String, Result<String, String>)>();

    for url in urls.clone() {
        let tx_cloned = tx.clone();
        thread::spawn(move || {
            let result = fake_fetch(&url);
            tx_cloned.send((url, result)).ok();
        });
    }
    drop(tx); // 关掉原 tx，让 for rx 能在所有子线程结束后退出

    let mut succeeded = 0;
    let mut failed = 0;
    for (url, res) in rx {
        match res {
            Ok(body) => {
                println!("  ✓ {url}: {} bytes", body.len());
                succeeded += 1;
            }
            Err(e) => {
                println!("  ✗ {url}: {e}");
                failed += 1;
            }
        }
    }

    println!(
        "  {succeeded}/{} succeeded, {failed} failed, elapsed = {:?}",
        urls.len(),
        start.elapsed()
    );
    println!();
}
