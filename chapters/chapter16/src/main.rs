// 第 16 章对应课程 Part 3 的章节：Variance and Its Types。
// 这批内容比前面章节更偏高级主题，所以当前仓库先把原始代码完整纳入，
// 再通过统一的 `main.rs` 入口把每个 topic 串起来。
// 如果你第一次跑这一章，建议先只看三件事：
// 1. `main.rs` 决定了这一章的演示顺序。
// 2. 每个 `topic_XX_*.rs` 都保留了原始课程代码全文。
// 3. `lab.rs` 放在最后，是为了把“收录原始代码”继续推进成“真正可运行的教学代码”。

mod lab;
mod topic_01_background_for_variance_references_refresher_part_1;
mod topic_02_background_for_variance_references_refresher_part_2;
mod topic_03_background_for_variance_lifetime_bounds;
mod topic_04_background_for_variance_implied_bounds;
mod topic_05_subtyping_and_variance_covariance;
mod topic_06_contravariance;
mod topic_07_invariance_part_1;
mod topic_08_invariance_part_2;

fn main() {
    println!("Chapter 16: Variance and Its Types");
    println!();

    topic_01_background_for_variance_references_refresher_part_1::run();
    topic_02_background_for_variance_references_refresher_part_2::run();
    topic_03_background_for_variance_lifetime_bounds::run();
    topic_04_background_for_variance_implied_bounds::run();
    topic_05_subtyping_and_variance_covariance::run();
    topic_06_contravariance::run();
    topic_07_invariance_part_1::run();
    topic_08_invariance_part_2::run();
    lab::run();
}
