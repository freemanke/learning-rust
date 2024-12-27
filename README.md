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
- module 
- panic 

## 开发环境准备

- 安装 rustup 工具 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 更新 rust 工具链和 rustup 自身 `rustup update`
- 安装集成开发环境 Jetbrains RustRover

## 构建运行发布

```shell
cargo new learning-rust
cd learning-rust
cargo check
cargo build
cargo build --release
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


