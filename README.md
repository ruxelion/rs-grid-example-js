# rs-grid-example-js

Standalone example of [rs-grid](https://github.com/ruxelion/rs-grid) in
**vanilla JS / library mode**. The crate compiles to a WASM module that exposes
a `JsGrid` class; the host page owns the surrounding HTML and supplies a
`<canvas>` element.

This example pins the library at a released tag (`v0.1.0`) via a git
dependency — see [`Cargo.toml`](Cargo.toml).

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
```

## Build

```sh
wasm-pack build --target web --release --out-dir pkg
# → pkg/basic_js.js  +  pkg/basic_js_bg.wasm
```

## Use from JavaScript

```js
import init, { JsGrid } from "./pkg/basic_js.js";

await init();
const canvas = document.querySelector("canvas");
const grid = new JsGrid(canvas, 1_000, 20); // rows, columns

grid.set_pinned_count(2);
grid.set_filter("name", "alice");
grid.clear_filters();
// grid.detach();  // when removing the grid
```

The grid reads its theme from `--rs-grid-*` CSS variables on the page; call
`grid.set_theme_from_css()` after changing them.
