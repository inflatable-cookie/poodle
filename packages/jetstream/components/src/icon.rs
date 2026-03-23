//! Icon — Jetstream icon component backed by IconSpec.
//!
//! Contract: `docs/contracts/foundation/icon.md`
//! Reference: `packages/svelte/primitives/src/Icon.svelte`
//!
//! Renders an SVG icon via the engine's `icon()` constructor. The icon is
//! rasterized on demand by the engine's IconCache and tinted by `text_color`.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::IconSpec;

use crate::theme_ext::resolve_px;

pub fn js_icon(spec: &IconSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let size = resolve_px(theme, spec.size_token());

    // Use the engine's SVG icon constructor. The icon name maps to
    // `{icon_base_path}/{name}.svg` and is rasterized at the requested size.
    // `text_color` from the parent is inherited for tinting.
    ui_element::icon(&spec.name)
        .w(size)
        .h(size)
        .flex_row()
        .items_center()
        .justify_center()
}
