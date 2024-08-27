## main函数
- main函数是每个Rust可执行程序最先运行的代码
## 运行项目
### cargo run
- 在`rust_001_hello_world`目录下，执行`cargo run`(首先对项目进行编译，然后再运行)
   - 等同于运行如下两个命令
      1. `cargo build`
      2. `./target/debug/rust_001_hello_world`
         - 在debug模式下，代码的编译速度会非常快，但是运行速度就慢了。因为在debug模式下，Rust编译器不会做任何的优化，只为了尽快的编译完成，让开发更加顺畅
### 提高程序性能
- 运行如下命令
  1. `cargo run --release`
  2. `cargo build --release`
  3. `./target/release/rust_001_hello_world`
### cargo check
- 快速检查代码能否编译通过
### 总结
- 常用`cargo check`检查代码
- 打测试包`cargo build`
- 打生产包`cargo build --release`
