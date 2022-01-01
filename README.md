# lenna-core
[![build](https://github.com/lenna-project/lenna-core/actions/workflows/ci.yml/badge.svg)](https://github.com/lenna-project/lenna-core/actions)
[![Crates.io](https://img.shields.io/crates/v/lenna_core)](https://crates.io/crates/lenna_core)
[![dependency status](https://deps.rs/repo/github/lenna-project/lenna-core/status.svg)](https://deps.rs/repo/github/lenna-project/lenna-core)

Lenna is a library for image processing algorithms and apps.

This is the core library for lenna.

![Lenna CLI](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/lenna-project/lenna-core/main/docs/uml/cli.puml)

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

![Build](https://raw.githubusercontent.com/lenna-project/lenna-core/main/docs/images/build.gif)

## create plugins

```rust
use image::DynamicImage;
use lenna_core::core::processor::Processor;
use lenna_core::plugins::PluginRegistrar;
use lenna_core::ImageProcessor;
use lenna_core::ExifProcessor;
use lenna_core::ProcessorConfig;

lenna_core::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new(Plugin{ config: Config::default() }));
}
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
struct Config {
}
#[derive(Clone)]
pub struct Plugin {
    config: Config
};
impl Processor for Plugin {
    fn name(&self) -> String {
        "plugin".into()
    }

    fn title(&self) -> String {
        "Plugin".into()
    }

    fn author(&self) -> String {
        "chriamue".into()
    }

    fn description(&self) -> String {
        "Plugin description.".into()
    }

    fn process(
        &mut self,
        config: ProcessorConfig,
        image: &mut Box<lenna_core::LennaImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config = serde_json::from_value(config.config).unwrap();
        self.process_exif(&mut image.exif).unwrap();
        self.process_image(&mut image.image).unwrap();
        Ok(())
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::to_value(Config::default()).unwrap()
    }
}
impl ImageProcessor for Plugin {
    fn process_image(
        &self,
        image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = DynamicImage::ImageRgba8(image.to_rgba8());
        *image = Box::new(img);
        Ok(())
    }
}
impl ExifProcessor for Plugin {}
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
