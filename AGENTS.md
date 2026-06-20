# rs-grid-example-js — Claude guide

Full-featured demo of **rs-grid** in **vanilla JS / library mode**. The crate compiles to
a WASM module exposing an extended `JsGrid` class; a host page ([`index.html`](index.html)
+ [`main.js`](main.js)) drives it with the same control panel as the framework demos
(dataset / column / theme / language selectors, editable / selectable / column-reorder
toggles, column-layout persistence, reset). Wrapper logic: [`src/lib.rs`](src/lib.rs).
Full notes: [`README.md`](README.md).

## Quick reference

```sh
wasm-pack build --target web --out-dir pkg   # → pkg/basic_js.{js,wasm}
npx serve . -p 4183                           # then open http://localhost:4183
```

## JsGrid API (public contract — keep in sync with `main.js` + README)

```
new(canvas, rows, cols)        set_theme_from_css()        set_locale(tag)
set_editable(on)               set_selectable(on)          set_column_reorderable(on)
set_pinned_count(n)            set_filter(key, text)       clear_filters()
reset_layout()                 set_on_validation_error(cb) set_on_cell_button_click(cb)
export_patches()               import_patches(tsv)         detach()
```

Layout (widths / order / pinned count) persists to `localStorage` under
`rs-grid-basic-layout`, shared with the framework demos.

## End-to-end tests (Playwright)

A functional + visual-regression suite ([`e2e/`](e2e/)) drives the host page through the
`JsGrid` API.

```sh
cd e2e && npm install              # first time
npx playwright install chromium    # first time
cd .. && wasm-pack build --target web --out-dir pkg   # build pkg/ first
cd e2e && npm test                 # run
cd e2e && npm run update-snapshots # regenerate visual baselines
```

## Critical: this repo does NOT contain the library

`rs-grid-*` and `example-common` are **git dependencies pinned to a tag** (currently
`rs-grid-core-v0.1.3`, see [`Cargo.toml`](Cargo.toml)):

- The library source is in the separate `rs-grid` repo. Editing files here changes only
  the `JsGrid` wrapper / host page — never grid behaviour.
- **All deps must share the exact same tag.** Mixing per-crate tags breaks the build
  (`example-common` must match the library it was built against).
- To adopt a new library version: bump the tag on all deps together, then `cargo update`.

> **Temporary (pre-release dev):** `Cargo.toml` carries a `[patch."…/rs-grid"]` block
> pointing the `rs-grid-*` deps at a local working tree so the demo can build against
> unreleased API (`example_common::layout`, `rs_grid_web::storage`). Remove it and bump
> the `tag` once the new rs-grid version ships. `js-sys` was added as a dependency for the
> JS callback bridges (`set_on_*`).

## Conventions

- `pkg/` is wasm-pack output — never hand-edit.
- `themes/` is **vendored** from the rs-grid reference theme — re-vendor rather than hand-edit.
- Rust files are auto-formatted on save (PostToolUse `rustfmt` hook). No clippy hook: this
  is a `cdylib` + `wasm-bindgen` crate, so host-target clippy does not apply.
- In `e2e/`, `node_modules/` and `test-results/` are gitignored; `tests/snapshots/` (visual
  baselines) are committed.
