# lenna-core

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