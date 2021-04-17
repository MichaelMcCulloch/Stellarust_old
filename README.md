# Stellarust

the backend can be build with `cargo build` and started with `cargo run`

the front end can be built with `wasm-pack build --target web --out-name wasm --out-dir static` and started with `miniserve -v ./static --index index.html -p <port>`