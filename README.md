# Paint by Numbers!

As the name suggests, this is a Paint by Numbers generator and player. Inspired by my [QWERHacks 2025 project](https://devpost.com/software/paint-by-campus), I wanted to remake the paint by numbers app to be faster and fully client-side. To do this, I re-wrote the backend part in Rust with the intention of compiling it to WebAssembly.

## Rust compilation

To compile the Rust app, we will use `wasm-pack` (`cargo install wasm-pack`) to create a production wasm binary, along with helper js and ts files. Here is the command, which you should run in the `pbn` directory:

```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
```

### Testing Rust functions

To run tests, simply replace the `<TEST_NAME>` with the name of the test in the following function:

```bash
cargo test -- --nocapture <TEST_NAME>
```

## Running the frontend app

To run the frontend, make sure the Rust package is compiled. Then, install the packages using `npm install`. Finally, run the app using the following:

```bash
npm run dev
```
