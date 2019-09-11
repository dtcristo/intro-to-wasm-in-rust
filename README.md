# Introduction to WebAssembly in Rust

Examples shown in my talk at Rust Melbourne meetup 14/08/2019. The recording is available on [YouTube](https://www.youtube.com/watch?v=WONGc1zUxtc).

## Installation

Install the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt).

```sh
brew install wabt
```

Install Rust with [rustup](https://rustup.rs/).

Add `wasm32-unknown-unknown` target.

```sh
rustup target add wasm32-unknown-unknown
```

Build all examples.

```sh
./build.sh
```

Start a webserver like [es-dev-server](https://github.com/open-wc/open-wc/tree/master/packages/es-dev-server)
that supports `application/wasm` MIME type.

```sh
npm install -g es-dev-server
es-dev-server
```

View examples in browser at [http://localhost:8000/](http://localhost:8000/).

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
