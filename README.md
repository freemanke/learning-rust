# Learning Rust

Rust 是一种预编译静态类型语言，生成的可执行文件不需要额外的运行时环境

## 参考文档
- Rust 语言官方文档 [https://www.rust-lang.org/](https://www.rust-lang.org/)
- Rust 程序设计语言中文版 [https://kaisery.github.io/trpl-zh-cn/title-page.html](https://kaisery.github.io/trpl-zh-cn/title-page.html)

## 术语表

- rustup Rust 的版本管理工具，用于安装和管理 Rust 的工具链（包括编译器 rustc、包管理器 cargo 等）
- cargo Rust 的包管理器和构建工具，主要用于管理项目依赖和执行编译、测试等任务
- rustc Rust 语言编译器

## 开发环境准备

- 安装 rustup 工具 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 更新 rust 工具链和 rustup 自身 `rustup update`
- 安装集成开发环境 Jetbrains RustRover

## 构建运行和测试

```shell
cd hello_world
cargo build
cargo run
cargo publish
```



## 构建系统和包管理器
Cargo 是 Rust 的构建系统和包管理器
```
cargo build
cargo check 
cargo run
```

## 字符串插值
```
println!("This is my {}.", name);
```