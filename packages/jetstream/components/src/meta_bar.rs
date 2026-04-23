//! MetaBar — wrapping inline metadata row backed by MetaBarSpec.
//!
//! Contract: `docs/contracts/components/meta-bar.md`
//! Reference: `packages/gpui/components/src/primitives/meta_bar.rs`
//!
//! Inserts dot separators between children when show_separators is true.
//! Uses color.text.secondary (at 72% opacity) for dot color.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MetaBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

/// Build a MetaBar element from a MetaBarSpec.
///
/// Contract anatomy:
/// ```text
/// [Root]   flex-row, wrap, gap space.inline.sm
///   ├── [Child]  each caller-provided item
///   └── [Dot]   3px dot separator (when show_separators and idx > 0)
/// ```
pub fn js_meta_bar(spec: &MetaBarSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let gap = resolve_px(theme, "space.inline.sm");
    let separator_color: Color = resolve_color(theme, "color.text.secondary").into();
    // Dots at 72% opacity — matches GPUI reference
    let dot_color = Color::new(separator_color.r, separator_color.g, separator_color.b, separator_color.a * 0.72);
    let dot_size = rem_to_px(0.25); // 4px dot

    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(gap)
        .min_w(0.0);

    for (idx, child) in children.into_iter().enumerate() {
        if idx > 0 && spec.show_separators {
            row = row.child(
                ui_element::div()
                    .w(dot_size)
                    .h(dot_size)
                    .rounded(999.0)
                    .bg(dot_color)
            );
        }
        row = row.child(
            ui_element::div()
                .min_w(0.0)
                .child(child)
        );
    }

    row
}
