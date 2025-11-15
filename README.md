# 🦀 Rust Learning Journey

> 我的 Rust 语言学习之路：从语法入门到实战项目，记录每一步成长与思考

<p align ="center">
    <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"> 
    <img src="https://img.shields.io/badge/CLion-black?style=for-the-badge&logo=clion&logoColor=white">
    <img src="https://img.shields.io/badge/vercel-%23000000.svg?style=for-the-badge&logo=vercel&logoColor=white">
    <img src="https://img.shields.io/badge/ea-%23000000.svg?style=for-the-badge&logo=ea&logoColor=white">
    <img src="https://img.shields.io/badge/steam-%23000000.svg?style=for-the-badge&logo=steam&logoColor=white">
    <img src="https://img.shields.io/badge/unity-%23000000.svg?style=for-the-badge&logo=unity&logoColor=white">
    <img src="https://img.shields.io/badge/ApplePay-000000.svg?style=for-the-badge&logo=Apple-Pay&logoColor=white">
    <img src="https://img.shields.io/badge/Codepen-000000?style=for-the-badge&logo=codepen&logoColor=white">
</p>

## 仓库介绍

这是一个用于记录我学习 Rust 编程语言的**全过程仓库**，包含：



* 基础语法练习代码（变量、所有权、生命周期、模式匹配等核心概念）

* 经典算法与数据结构的 Rust 实现

* 小型实战项目（命令行工具、Web 服务、系统编程等）

* 学习笔记与踩坑总结

* 优质学习资源整理（文档、教程、社区链接）

适合 Rust 初学者参考，也作为我个人技术沉淀的载体。代码会持续更新，从基础到进阶逐步深入，力求每一份代码都附带清晰注释和思路说明。

## 学习路径

### 阶段 1：基础入门

环境搭建与 Hello World

变量、数据类型与运算符

控制流（if-else、循环、匹配）

函数与模块

所有权、借用与生命周期

结构体与枚举

泛型与特性（Trait）

### 阶段 2：进阶特性

错误处理（Result、Option、panic!）

并发编程（线程、通道、Mutex）

异步编程（async/await）

宏编程

unsafe Rust

### 阶段 3：实战项目

命令行工具（文件处理、数据统计）

Web 服务（使用 Axum/Tide 框架）

系统编程（文件 IO、进程管理）

嵌入式开发入门（可选）

## 目录结构



```
rust-learning-journey/

├── 01\_basics/          # 基础语法练习

│   ├── variables/      # 变量与数据类型

│   ├── control\_flow/   # 控制流

│   ├── functions/      # 函数

│   └── ...

├── 02\_advanced/        # 进阶特性

│   ├── ownership/      # 所有权

│   ├── concurrency/    # 并发

│   ├── async/          # 异步

│   └── ...

├── 03\_projects/        # 实战项目

│   ├── cli-tools/      # 命令行工具

│   ├── web-service/    # Web服务

│   └── ...

├── notes/              # 学习笔记（Markdown格式）

│   ├── rust\_syntax.md  # 语法笔记

│   ├── pitfalls.md     # 踩坑总结

│   └── ...

└── resources.md        # 优质学习资源汇总
```

## 环境配置



* Rust 版本：>= 1.70.0（推荐使用`rustup`管理版本）

* 编辑器：VS Code + Rust Analyzer 插件

* 构建工具：Cargo（Rust 官方构建工具）

快速开始：



```
\# 克隆仓库

git clone https://github.com/你的用户名/rust-learning-journey.git

cd rust-learning-journey

\# 运行基础练习（以变量为例）

cd 01\_basics/variables

cargo run

\# 运行实战项目

cd 03\_projects/cli-tools/xxx-project

cargo run
```

## 学习资源

详细资源清单见 [res](resources.md)[ource](resources.md)[s.md](resources.md)，核心推荐：



1. 官方文档：[The R](https://doc.rust-lang.org/book/)[ust P](https://doc.rust-lang.org/book/)[rogra](https://doc.rust-lang.org/book/)[mming](https://doc.rust-lang.org/book/)[ Lang](https://doc.rust-lang.org/book/)[uage](https://doc.rust-lang.org/book/)（权威入门）

2. 练习平台：[E](https://exercism.org/tracks/rust)[xerci](https://exercism.org/tracks/rust)[sm Ru](https://exercism.org/tracks/rust)[st Tr](https://exercism.org/tracks/rust)[ack](https://exercism.org/tracks/rust)（针对性练习）

3. 实战教程：[Rust](https://doc.rust-lang.org/rust-by-example/)[ by Ex](https://doc.rust-lang.org/rust-by-example/)[ample](https://doc.rust-lang.org/rust-by-example/)（代码驱动学习）

4. 社区：[Rust](https://rustcc.cn/)[ 中文社区](https://rustcc.cn/)、Stack Overflow Rust 标签

## 贡献与交流



* 如果你发现代码错误或有优化建议，欢迎提交 Issue 或 PR

* 如果你也是 Rust 学习者，欢迎一起交流学习经验，共同进步！

## 许可证

[MIT License](LICENSE) - 允许自由使用、修改和分发本仓库代码（注明出处即可）

> （注：文档部分内容可能由 AI 生成）