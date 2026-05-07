//! MetaItem — label + value pair for use inside MetaBar.
//!
//! Contract: `docs/contracts/components/meta-item.md`
//! Reference: `packages/gpui/components/src/primitives/meta_item.rs`
//!
//! Renders a small uppercase label followed by a value slot.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MetaItemSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a MetaItem element from a MetaItemSpec and an optional value element.
///
/// Contract anatomy:
/// ```text
/// [Root]  flex-row, wrap, gap 0.375rem
///   ├── [Label]  uppercase, text.secondary, 0.6875rem, semibold
///   └── [Value]  caller-supplied content or placeholder, text.primary, 0.875rem
/// ```
pub fn js_meta_item(spec: &MetaItemSpec, theme: &JetstreamThemeProvider, value: Option<JsEl>) -> JsEl {
    let label_color = resolve_color(theme, "color.text.secondary");
    let value_color = resolve_color(theme, "color.text.primary");
    let label_size = rem_to_px(spec.label_font_size_rem());
    let value_size = rem_to_px(spec.value_font_size_rem());
    let gap = rem_to_px(spec.gap_rem());

    // Jetstream's current text API does not expose font-family, letter-spacing,
    // or line-height controls, so those typography details remain documented
    // runtime deltas for now.
    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(gap)
        .min_w(0.0);

    if let Some(ref text) = spec.label {
        row = row.child(
            ui_element::label(&text.to_uppercase())
                .text_color(label_color)
                .text_size(label_size)
                .text_weight(600)
        );
    }

    let value_el = value.unwrap_or_else(|| {
        ui_element::div()
            .text_size(value_size)
            .text_color(value_color)
    });

    row = row.child(
        ui_element::div()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(gap)
            .min_w(0.0)
            .text_size(value_size)
            .text_color(value_color)
            .child(value_el)
    );

    row
}
