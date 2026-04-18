# 2. Coercion 触发位置

> 关键词：coercion site

## 一分钟结论

**只在这些位置**编译器会主动尝试 coercion：

1. 函数调用的**实参**位置
2. `let x: Target = expr;` 带显式类型标注
3. `static` / `const` 初始化
4. `return` 表达式
5. 方法调用（deref 链式查找）

**不**触发的位置：
- 泛型参数的具体填充
- 没有类型标注的 `let` 绑定

## 对应代码

- [topic_02_coercion_sites.rs](../../chapters/chapter13/src/topic_02_coercion_sites.rs)
