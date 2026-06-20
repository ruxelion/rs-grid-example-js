// Host controller for the rs-grid JS (library-mode) demo.
//
// The Rust crate exposes a `JsGrid` class via wasm-bindgen; this script owns
// the surrounding HTML chrome and drives the grid through that public API —
// the same feature set as the Leptos / Dioxus / Yew demos.

import init, { JsGrid } from "./pkg/basic_js.js";

const SUPPORTED_LANGS = [
  "en", "fr", "de", "es", "it", "pt", "nl", "pl",
  "tr", "ru", "uk", "ar", "ja", "zh", "ko",
];

/** Detect the browser language, restricted to the languages we offer. */
function initialLang() {
  const tag = (navigator.language || "en").split("-")[0];
  return SUPPORTED_LANGS.includes(tag) ? tag : "en";
}

async function main() {
  await init();

  const canvas = document.getElementById("rs-grid-canvas");
  const datasetSel = document.getElementById("dataset-size");
  const columnSel = document.getElementById("column-count");
  const themeSel = document.getElementById("theme");
  const langSel = document.getElementById("language");
  const editableCb = document.getElementById("editable");
  const selectableCb = document.getElementById("selectable");
  const reorderCb = document.getElementById("column-reorder");
  const resetBtn = document.getElementById("reset-layout");
  const rowsLabel = document.getElementById("rows-label");
  const colsLabel = document.getElementById("cols-label");
  const validationBox = document.getElementById("validation-error");
  const buttonBox = document.getElementById("button-action");

  // Reflect the detected language in the selector.
  langSel.value = initialLang();

  let grid = null;

  function updateLabels() {
    rowsLabel.textContent = datasetSel.selectedOptions[0].textContent;
    colsLabel.textContent = columnSel.selectedOptions[0].textContent;
  }

  // (Re)build the grid for the current dataset size / column count. A fresh
  // model is needed when those change, so the old grid is detached first.
  function buildGrid() {
    if (grid) grid.detach();

    const rows = Number(datasetSel.value);
    const cols = Number(columnSel.value);
    grid = new JsGrid(canvas, rows, cols);

    // Apply current control state to the fresh grid.
    grid.set_locale(langSel.value);
    grid.set_editable(editableCb.checked);
    grid.set_selectable(selectableCb.checked);
    grid.set_column_reorderable(reorderCb.checked);

    grid.set_on_validation_error((row, col, msg) => {
      validationBox.textContent = `[${col}] ${msg}`;
      validationBox.hidden = false;
    });
    grid.set_on_cell_button_click((row, col, btn) => {
      buttonBox.textContent = `Button clicked: [${btn}] row=${row} col=${col}`;
      buttonBox.hidden = false;
    });

    updateLabels();
  }

  // Dataset / column changes rebuild the grid (new model).
  datasetSel.addEventListener("change", buildGrid);
  columnSel.addEventListener("change", buildGrid);

  // Theme: set the document root class, then re-read the CSS theme vars.
  themeSel.addEventListener("change", () => {
    document.documentElement.className = themeSel.value;
    if (grid) grid.set_theme_from_css();
  });

  // Language: switch the UI locale in place.
  langSel.addEventListener("change", () => {
    if (grid) grid.set_locale(langSel.value);
  });

  // Toggles.
  editableCb.addEventListener("change", () => {
    if (grid) grid.set_editable(editableCb.checked);
  });
  selectableCb.addEventListener("change", () => {
    if (grid) grid.set_selectable(selectableCb.checked);
  });
  reorderCb.addEventListener("change", () => {
    if (grid) grid.set_column_reorderable(reorderCb.checked);
  });

  // Reset the persisted layout, then reload to rebuild with defaults.
  resetBtn.addEventListener("click", () => {
    if (grid) grid.reset_layout();
    window.location.reload();
  });

  buildGrid();
}

main();
