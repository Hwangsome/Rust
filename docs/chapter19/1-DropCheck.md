# 1. Drop Check

编译器验证：一个值被 drop 时，它所借用的其他值必须仍然活着。违反时报 E0505 或类似错误。

## 对应代码

- [topic_01_drop_check.rs](../../chapters/chapter19/src/topic_01_drop_check.rs)
