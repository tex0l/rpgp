//! Empty crate root — the actual tests live under `tests/`. This crate
//! exists purely so `wasm-pack test --node tests/wasm` has a self-
//! contained dep graph (no native-only dev-deps from the main `pgp`
//! crate get pulled into the wasm32 build).
