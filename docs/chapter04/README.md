# 第 4 章：Organizing your Code

这一章开始从“语法和类型”切到“代码怎么组织”。真正写 Rust 项目时，你很快会发现难点不只是：

- 代码能不能编过

而是：

- 这个东西放哪个 crate
- 为什么这里要拆 module
- 谁应该 `pub`
- API 路径该不该重导出
- 发布出去后别人会怎么用

## 本章目标

- 分清 package、crate、module 三层概念
- 理解模块不仅是“分文件”，更是命名空间和边界管理
- 建立对可见性、隐私和重导出的工程直觉
- 知道依赖管理和 crate 发布的最小流程

## 推荐阅读顺序

1. [代码组织](./1-代码组织.md)
2. [模块基础](./2-模块基础.md)
3. [可见性说明符](./3-可见性说明符.md)
4. [模块中的隐私与重导出](./4-模块中的隐私与重导出.md)
5. [使用外部依赖](./5-使用外部依赖.md)
6. [发布你的 Crate](./6-发布你的Crate.md)

## 对应代码与运行方式

- Cargo package：`chapter04`
- 运行方式：`cargo run -p chapter04`
- 章节入口：[chapters/chapter04/src/main.rs](../../chapters/chapter04/src/main.rs)
- 练习模块：[lab.rs](../../chapters/chapter04/src/lab.rs)

## 本章关键输出

```text
package 是最高层组织单位，Cargo.toml 负责描述它。
crate 是编译单元，binary crate 产出可执行文件，library crate 产出可复用库。
module 用来在 crate 内继续组织命名空间和可见性。
```

```text
私有函数只能在当前模块内部使用。
公开函数可以被模块外调用。
```

```text
pub struct 不等于字段也自动公开，通常要通过构造函数或方法暴露访问。
```

## 本章常见误区

- 把 module 只理解成“分文件”
- 把 `pub` 当成“写了就省事”
- 以为 `pub struct` 会自动公开全部字段
- 把“能发布”误解成“准备好了发布”
