# 第 5 章：Testing

这一章开始回答“写完代码之后，怎么确认它真的按预期工作”。重点不是背命令，而是分清测试的层次：

- 单元测试在测什么
- 集成测试在测什么
- `cargo test` 的控制参数在解决什么问题
- benchmark 为什么不等于普通测试

## 本章目标

- 建立单元测试、集成测试、benchmark 的边界
- 学会读懂 `#[test]`、`#[cfg(test)]`、`#[ignore]`、`#[should_panic]`
- 知道如何过滤测试、显示输出、单独运行 ignored tests

## 推荐阅读顺序

1. [单元测试](./1-单元测试.md)
2. [控制测试如何运行](./2-控制测试如何运行.md)
3. [集成测试](./3-集成测试.md)
4. [基准测试](./4-基准测试.md)

## 对应代码与运行方式

- Cargo package：`chapter05`
- 运行方式：`cargo run -p chapter05`
- 测试方式：`cargo test -p chapter05`
- 章节入口：[chapters/chapter05/src/main.rs](../../chapters/chapter05/src/main.rs)
- 集成测试示例：[tests/smoke.rs](../../chapters/chapter05/tests/smoke.rs)

## 本章关键输出

`cargo test -p chapter05 -- --nocapture`：

```text
test result: ok. 3 passed; 0 failed; 1 ignored
test smoke_test ... ok
```

运行章节：

```text
add(2, 3) = 5
cargo test -p chapter05 -- --ignored
sum = 50005000
粗略计时结果 = 154.042µs
```

## 本章常见误区

- 把测试只理解成“写几个 assert”
- 把 benchmark 当成“跑得快的测试”
- 只会 `cargo test`，不会筛选、忽略和观察输出
