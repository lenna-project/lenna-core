# lenna-core ![build](https://github.com/lenna-project/lenna-core/actions/workflows/ci.yml/badge.svg)

Lenna is a library for image processing algorithms and apps.

This is the core library for lenna.

## quickstart

```sh
cargo build
cargo test
```

## create plugins

```rust
use lenna_core::core::processor::Processor;
use lenna_core::plugins::PluginRegistrar;

lenna_core::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new(Plugin));
}

pub struct Plugin;
```

## 🌏 Language Support

| <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust" width="16px" height="16px" /> Rust | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/JavaScript-logo.png/240px-JavaScript-logo.png" alt="JavaScript" width="16px" height="16px" /> JavaScript | <img src="https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg" alt="Python" width="16px" height="16px" /> Python | <img src="https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg" alt="C++" width="16px" height="16px" /> C++ | <img src="https://upload.wikimedia.org/wikipedia/commons/1/1f/WebAssembly_Logo.svg" alt="WASM" width="16px" height="16px" /> WASM |
| :---------: | :---------: | :---------: | :---------: | :---------: |
| Yes | Yes | No | No | Yes |

## 📜 License

This software is licensed under the [MIT](https://github.com/lenna-project/lenna-core/blob/main/LICENSE) © [lenna-project](https://github.com/lenna-project).
