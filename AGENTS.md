# rs-grid-example-js — Claude guide

Standalone demo of **rs-grid** in **vanilla JS / library mode**. The crate compiles to a
WASM module exposing a `JsGrid` class; the host page owns the surrounding HTML and supplies
a `<canvas>`. App logic: [`src/lib.rs`](src/lib.rs). Full notes: [`README.md`](README.md).

## Quick reference

```sh
# Build the WASM package (no dev server — this is library mode)
wasm-pack build --target web --release --out-dir pkg
# → pkg/basic_js.js + pkg/basic_js_bg.wasm
```

JS usage:

```js
import init, { JsGrid } from "./pkg/basic_js.js";
await init();
const grid = new JsGrid(document.querySelector("canvas"), 1_000, 20); // rows, cols
grid.set_theme_from_css(); // re-read --rs-grid-* CSS vars after a theme change
```

## Critical: this repo does NOT contain the library

`rs-grid-*` and `example-common` are **git dependencies pinned to a tag** (currently
`rs-grid-core-v0.1.3`, see [`Cargo.toml`](Cargo.toml)):

- The library source is in the separate `rs-grid` repo. Editing files here changes only
  the `JsGrid` wrapper in `src/lib.rs` — never grid behaviour.
- **All deps must share the exact same tag.** Mixing per-crate tags breaks the build
  (`example-common` must match the library it was built against).
- To adopt a new library version: bump the tag on all deps together, then `cargo update`.

## Conventions

- `pkg/` is wasm-pack output — never hand-edit.
- The `JsGrid` API surface (method names) is the public contract demoed here; keep it in
  sync with the README usage block when you change it.
- Rust files are auto-formatted on save (PostToolUse `rustfmt` hook). No clippy hook: this
  is a `cdylib` + `wasm-bindgen` crate, so host-target clippy does not apply.
