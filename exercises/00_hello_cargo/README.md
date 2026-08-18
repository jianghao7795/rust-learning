# 阶段 0 项目：Cargo 问候程序

这是 `docs/00-environment.md` 的配套项目，用于认识 package、library crate、binary crate、测试和构建产物。

```bash
cargo run -p stage00-hello-cargo -- Rust
cargo test -p stage00-hello-cargo
cargo build -p stage00-hello-cargo --release
```

程序接收一个可选名称并输出问候语。可以尝试修改包名、版本号和问候格式，然后观察 `cargo check`、`cargo test` 与 `cargo build` 的区别。
