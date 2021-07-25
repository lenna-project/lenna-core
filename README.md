# lenna-core
[![build](https://github.com/lenna-project/lenna-core/actions/workflows/ci.yml/badge.svg)](https://github.com/lenna-project/lenna-core/actions)
[![Crates.io](https://img.shields.io/crates/v/lenna_core)](https://crates.io/crates/lenna_core)
[![dependency status](https://deps.rs/repo/github/lenna-project/lenna-core/status.svg)](https://deps.rs/repo/github/lenna-project/lenna-core)

Lenna is a library for image processing algorithms and apps.

This is the core library for lenna.

## quickstart

```sh
cargo build
```

### run tests

```sh
cargo test
cargo test --features=python
wasm-pack test --node
```

![Build](docs/images/build.gif)

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

## 🐍 build python bindings

Create a virtual environment for python and install lenna-core using pip.

```bash
virtualenv -p python3 .venv
source .venv/bin/activate
pip install .
python src/plugins/python/test.py
```

```python
import lenna_core_py
print(lenna_core_py.Resize.description())
```


## 🌏 Language Support

| <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust" width="16px" height="16px" /> Rust | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/JavaScript-logo.png/240px-JavaScript-logo.png" alt="JavaScript" width="16px" height="16px" /> JavaScript | <img src="https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg" alt="Python" width="16px" height="16px" /> Python | <img src="https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg" alt="C++" width="16px" height="16px" /> C++ | <img src="https://upload.wikimedia.org/wikipedia/commons/1/1f/WebAssembly_Logo.svg" alt="WASM" width="16px" height="16px" /> WASM |
| :---------: | :---------: | :---------: | :---------: | :---------: |
| Yes | Yes | Yes | No | Yes |

## 📜 License

This software is licensed under the [MIT](https://github.com/lenna-project/lenna-core/blob/main/LICENSE) © [lenna-project](https://github.com/lenna-project).
