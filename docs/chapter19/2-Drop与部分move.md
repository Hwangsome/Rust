# 2. Drop 与部分 Move

- 对**没有** Drop 的 struct：partial move（拆出一个字段）合法，剩余字段仍 drop
- 对**有** Drop 的 struct：partial move 非法（编译失败）—— 否则 drop 时字段状态不一致

## 对应代码

- [topic_02_drop_check_and_partial_move.rs](../../chapters/chapter19/src/topic_02_drop_check_and_partial_move.rs)
