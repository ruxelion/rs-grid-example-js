//! basic-js — rs-grid wasm-bindgen example (vanilla JS / library mode).
//!
//! Exposes a `JsGrid` class to JavaScript. The host page owns the
//! surrounding HTML chrome — this crate only mounts the grid on a
//! `<canvas>` element supplied by the caller.

use std::rc::Rc;

use example_common::{
    build_model, class_map::resolve_classes, layout::LayoutSnapshot,
};
use rs_grid_core::state::GridState;
use rs_grid_web::{storage, theme_from_css_vars, GridCanvas, Locale};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// localStorage key for the persisted column layout. Shared with the other
/// framework demos so a layout survives switching demos in the same browser.
const LS_KEY: &str = "rs-grid-basic-layout";

/// Handle to a mounted rs-grid instance, usable from JS.
#[wasm_bindgen]
pub struct JsGrid {
    inner: GridCanvas,
}

#[wasm_bindgen]
impl JsGrid {
    /// Mount a new grid on `canvas` with `row_count` rows
    /// and `col_count` columns of fake data.
    ///
    /// Uses `f64` instead of `u64` to avoid JS `BigInt`.
    ///
    /// A persisted column layout (widths / order / pinned count) is restored
    /// from `localStorage` before mount, and re-saved whenever the user
    /// resizes, reorders, or pins/unpins a column.
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        row_count: f64,
        col_count: f64,
    ) -> JsGrid {
        console_error_panic_hook::set_once();

        let mut model = build_model(row_count as u64, col_count as usize);

        // Restore a previously persisted layout (if any) before mount.
        if let Some(snapshot) =
            storage::get_item(LS_KEY).and_then(|raw| LayoutSnapshot::from_json(&raw))
        {
            snapshot.apply(&mut model);
        }

        let theme = theme_from_css_vars();

        let css_w = canvas.client_width() as f64;
        let css_h = canvas.client_height() as f64;
        let state = GridState::new(model, css_w, css_h);

        let gc = GridCanvas::mount(canvas, state, theme, Locale::from_browser());
        gc.set_class_resolver(Rc::new(resolve_classes));

        // Persist the column layout whenever it changes so user-resized /
        // reordered / pinned columns survive a page reload.
        let gc_save = gc.clone();
        gc.set_on_columns_changed(move || {
            let snapshot = LayoutSnapshot::new(
                gc_save.column_widths(),
                gc_save.column_order(),
                gc_save.pinned_count(),
            );
            if let Some(json) = snapshot.to_json() {
                storage::set_item(LS_KEY, &json);
            }
        });

        gc.render();

        JsGrid { inner: gc }
    }

    /// Re-read the CSS theme variables and apply them.
    pub fn set_theme_from_css(&self) {
        self.inner.set_theme(theme_from_css_vars());
    }

    /// Switch the UI locale by IETF language tag (e.g. `"fr"`, `"ja"`).
    pub fn set_locale(&self, tag: &str) {
        self.inner.set_locale(Locale::from_language_tag(tag));
    }

    /// Enable or disable in-place cell editing.
    pub fn set_editable(&self, on: bool) {
        self.inner.set_editable(on);
    }

    /// Enable or disable cell / row / column selection.
    pub fn set_selectable(&self, on: bool) {
        self.inner.set_selectable(on);
    }

    /// Enable or disable drag-to-reorder of columns.
    pub fn set_column_reorderable(&self, on: bool) {
        self.inner.set_column_reorderable(on);
    }

    /// Clear the persisted column layout. The host page should reload the
    /// page afterwards to rebuild the grid with its default layout.
    pub fn reset_layout(&self) {
        storage::remove_item(LS_KEY);
    }

    /// Bridge column-validation errors to a JS callback.
    ///
    /// The callback is invoked as `cb(row, col_key, error_message)`.
    pub fn set_on_validation_error(&self, cb: js_sys::Function) {
        self.inner
            .set_on_validation_error(move |row: u64, col: &str, msg: &str| {
                let _ = cb.call3(
                    &JsValue::NULL,
                    &JsValue::from_f64(row as f64),
                    &JsValue::from_str(col),
                    &JsValue::from_str(msg),
                );
            });
    }

    /// Bridge cell-button clicks to a JS callback.
    ///
    /// The callback is invoked as `cb(row, col_key, button_id)`.
    pub fn set_on_cell_button_click(&self, cb: js_sys::Function) {
        self.inner
            .set_on_cell_button_click(move |row: u64, col: &str, btn: &str| {
                let _ = cb.call3(
                    &JsValue::NULL,
                    &JsValue::from_f64(row as f64),
                    &JsValue::from_str(col),
                    &JsValue::from_str(btn),
                );
            });
    }

    /// Set the number of pinned (frozen) columns.
    pub fn set_pinned_count(&self, count: usize) {
        self.inner.set_pinned_count(count);
    }

    /// Filter rows by text on a given column.
    pub fn set_filter(&self, col_key: &str, text: &str) {
        self.inner.set_filter(col_key, text);
    }

    /// Remove all active filters.
    pub fn clear_filters(&self) {
        self.inner.clear_filters();
    }

    /// Export edited cell patches as a TSV string.
    pub fn export_patches(&self) -> String {
        self.inner.export_patches()
    }

    /// Import cell patches from a TSV string.
    pub fn import_patches(&self, data: &str) {
        self.inner.import_patches(data);
    }

    /// Detach event listeners and clean up.
    pub fn detach(&self) {
        self.inner.detach();
    }
}
