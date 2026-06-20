# rs-grid-example-js

Full-featured example of [rs-grid](https://github.com/ruxelion/rs-grid) in
**vanilla JS / library mode**. The crate compiles to a WASM module exposing an
extended `JsGrid` class; a host page ([`index.html`](index.html) +
[`main.js`](main.js)) drives it with the same control panel as the framework
demos — dataset-size / column-count / theme / language selectors, editable /
selectable / column-reorder toggles, column-layout persistence, and reset.

This example pins the library at a released tag (`rs-grid-core-v0.1.3`) via a git
dependency — see [`Cargo.toml`](Cargo.toml). (A temporary `[patch]` block builds
against a local `rs-grid` working tree during pre-release development; it is
removed once the new tag ships.)

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
```

## Build & run the host demo

```sh
wasm-pack build --target web --out-dir pkg
# → pkg/basic_js.js  +  pkg/basic_js_bg.wasm
npx serve . -p 4183     # then open http://localhost:4183
```

## Use from JavaScript

```js
import init, { JsGrid } from "./pkg/basic_js.js";

await init();
const canvas = document.querySelector("canvas");
const grid = new JsGrid(canvas, 1_000, 20); // rows, columns

grid.set_theme_from_css();        // re-read --rs-grid-* CSS vars after a change
grid.set_locale("fr");            // switch UI language by IETF tag
grid.set_editable(false);         // toggle in-place editing
grid.set_selectable(true);        // toggle cell/row/column selection
grid.set_column_reorderable(true);// toggle drag-to-reorder columns
grid.set_pinned_count(2);         // freeze the leading columns
grid.set_filter("name", "alice");
grid.clear_filters();
grid.set_on_validation_error((row, col, msg) => console.warn(col, msg));
grid.set_on_cell_button_click((row, col, btn) => console.log(btn, row, col));
grid.reset_layout();              // clear the persisted column layout
// grid.detach();                 // when removing the grid
```

Column layout (widths / order / pinned count) is persisted to `localStorage`
under `rs-grid-basic-layout` and restored on the next construction — shared with
the framework demos.

## End-to-end tests

A Playwright suite (functional + visual regression) in [`e2e/`](e2e/) drives the
host page through the `JsGrid` API.

```sh
cd e2e && npm install              # first time
npx playwright install chromium    # first time
cd .. && wasm-pack build --target web --out-dir pkg   # build pkg/ first
cd e2e && npm test                 # run
cd e2e && npm run update-snapshots # regenerate visual baselines
```
