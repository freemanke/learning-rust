# Learning Rust

Rust 是一种预编译静态类型语言，生成的可执行文件不需要额外的运行时环境

## 参考文档
- Rust 语言官方文档 [https://www.rust-lang.org/](https://www.rust-lang.org/)
- Rust 程序设计语言中文版 [https://kaisery.github.io/trpl-zh-cn/title-page.html](https://kaisery.github.io/trpl-zh-cn/title-page.html)

## 术语表

- rustup 版本管理工具，用于安装和管理 Rust 的工具链（包括编译器 rustc、包管理器 cargo 等）
- cargo 包管理器和构建工具，主要用于管理项目依赖和执行编译、测试等任务
- rustc 编译器
- crate 编译器能够编译的最小代码单元，crate 是代码组织的最高级别单元，一个 crate 内部可以进一步分成多个模块（module）
- crates.io 开源包管理平台
- rustacean 是 Rust 社区中对 Rust 开发者的昵称
- dependencies 代码所需要的库
- package 包，是 Cargo 的概念，它用于构建、测试、共享 crate。一个 Package 包含一个 Cargo.toml 文件，该文件描述了如何构建其中包含的 Crates。一个 Package 可以包含多个 Crate，其中至少一个是 library crate。
- crate 单元包，是一个模块树，它可以产生一个 binary 或 library。每个 Crate 有一个 Crate root（binary: src/main.rs，library: src/lib.rs），它是源代码文件，Rust 编译器从这里开始构建 Crate 的模块树。
- module 模块，是用于组织和管理代码的基本单元。它可以包含其他模块、结构体、函数等。模块通过 `use` 语句可以在其他模块中引用，这有助于控制代码的组织、作用域和私有路径。
- trait 特征，类型可以实现的抽象接口

## 开发环境准备

- 安装 rustup 工具 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 更新 rust 工具链和 rustup 自身 `rustup update`
- 安装格式化工具 `rustup component add rustfmt`
- 安装 lint 工具 `rustup component add clippy`
- 安装集成开发环境 Jetbrains RustRover

## 构建运行发布

```shell
cargo new learning-rust
cd learning-rust
cargo check
cargo build
cargo build --release
cargo fmt # 代码格式化
cargo clippy # 捕捉常见错误和改进 Rust 代码
cargo run
cargo publish
```

## 语言特性

- 变量默认是不可变 (immutable) `let x = 100`，如果需要可变变量需要显示声明 `let mut x = 100`
- 常量永远不可变，不可以对常量使用 `mut` `const TIMOUT_IN_SECONDS=60`
- 同一个作用域内，变量可以多次申明，第二次声明的同名变量隐藏(Shadowing)了第一次声明的同名变量 `let x=5; let x=x+1;`
- 字符串插值 `let greeting = format!("Hello, my name is {} and I am {} years old.", name, age);`
- 字符串插值 `println!("Pi to 2 decimal places: {:.2}", value);`

## 单元测试



## 错误处理


