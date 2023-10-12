# bevy-visualizer

as always...

```
docker run --rm -it -v $(pwd):/app rust:1.72 /bin/bash
```

https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md



## webasm build

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/bevy_visualizer.wasm --out-dir ./docs/ --target web
```
