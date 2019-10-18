# diffcount-yew
WASM Diff Cell Calculator.

This is a play project to hack on rust, and tinker with WASM using [Yew](https://github.com/yewstack/yew)

## Getting Started 


- Install WASM Rust Target  `rustup target add wasm32-unknown-unknown`
- Install Cargo Web `cargo install cargo-web`
- Build and Host the application directly to native wasm (no emscriptem) `cargo web start --target=wasm32-unknown-unknown --auto-reload` 

You should now have a web host, auto recompiling on source file change, exposing the app on http://localhost:8000 for your auto-reloading testing goodness :)
