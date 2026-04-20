// 第 14 章对应课程 Part 3 的章节：Concurrency。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_thread_basics;
mod topic_02_ownership_in_threads;
mod topic_03_message_passing_through_channels_part_1;
mod topic_04_message_passing_through_channels_part_2;
mod topic_05_sharing_states_part_1;
mod topic_06_sharing_states_part_2;
mod topic_07_synchronization_through_barriers;
mod topic_08_scoped_threads;
mod topic_09_thread_parking;
mod topic_10_async_await_basics;
mod topic_11_tokio_tasks;
mod topic_12_computationally_expensive_functions;
mod topic_13_project_webscrapping_using_threads;

fn main() {
    println!("Chapter 15: Concurrency");
    println!();

    topic_01_thread_basics::run();
    topic_02_ownership_in_threads::run();
    topic_03_message_passing_through_channels_part_1::run();
    topic_04_message_passing_through_channels_part_2::run();
    topic_05_sharing_states_part_1::run();
    topic_06_sharing_states_part_2::run();
    topic_07_synchronization_through_barriers::run();
    topic_08_scoped_threads::run();
    topic_09_thread_parking::run();
    topic_10_async_await_basics::run();
    topic_11_tokio_tasks::run();
    topic_12_computationally_expensive_functions::run();
    topic_13_project_webscrapping_using_threads::run();
    lab::run();
}
