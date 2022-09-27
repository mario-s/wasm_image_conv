# Webassembly to convert an image into grayscale

Sample about how to use a Webassemly in a Browser for image manipulation.

## Requirement

`cargo install wasm-pack`

## Building

`wasm-pack build --target web`

## Verify result in browser

Run a webserver, e.g. `python3 -m http.server`


## References

- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- https://bojanstipic.rs/blog/01-introduction-to-webassembly-and-rust/
- https://rustwasm.github.io/docs/wasm-bindgen/introduction.html
- https://developpaper.com/implementation-of-a-simple-image-processing-application-based-on-webassembly/
