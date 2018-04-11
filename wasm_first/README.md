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
