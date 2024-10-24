# rust-cli-template


## Run/bench wasm
1. Install wasmtime from: https://wasmtime.dev/
1. Set wasmrunner as: `export CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime --dir=. --`
1. Run `cargo run/bench --target wasm32-wasi`