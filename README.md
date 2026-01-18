# Simple 3D Renderer

A simple 3D renderer built in Rust using the `iced` GUI library.
It displays various 3D shapes and allows for interactive rotation and viewing.

Check the project out [here](https://tiredsnorlax.github.io/wasm-3d-renderer/)

## Running the Project

### Native

To run the project natively, use the following command:

```bash
cargo run
```

### WebAssembly (Wasm)

To run the project in a web browser, you will need to have `trunk` installed.
You can install it with the following command:

```bash
cargo install trunk
```

Once `trunk` is installed, you can run the project with:

```bash
trunk serve --open
```

This will build the project and serve it on a local web server, and `--open` will automatically open it in your web browser.
