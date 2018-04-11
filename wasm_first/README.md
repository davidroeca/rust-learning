### Rust Web assembly

Going through some rust web assembly tutorials

### Required tools

```bash
# Update rustup
rustup update

# You must add the `wasm32-unknown-unknown` target to rustup:
rustup target add wasm32-unknown-unknown

# for compiling a smaller wasm module
cargo install wasm-gc

# for serving http
cargo install https
```

### Build and run

```bash
# Compile and shrink web assembly
make

# Serve http, typically on 8000 (check)
make serve

# now load http://localhost:{PORT} (typically 8000) in your browser
```
