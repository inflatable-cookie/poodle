//! Combobox — Jetstream combobox backed by ComboboxSpec.
//!
//! Contract: `docs/contracts/foundation/combobox.md`
//! Uses SVG chevron icon and overlay() for dropdown.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::ComboboxSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_combobox(spec: &ComboboxSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border_color = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");

    // Hover border
    let border_c: Color = border_color.into();
    let text_primary: Color = text_color.into();
    let hover_border = border_c.mix(text_primary, 0.78);

    let display = spec.value.as_deref()
        .or(spec.placeholder.as_deref())
        .unwrap_or("Search...");

    ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .h(36.0).pl(12.0).pr(8.0)
        .flex_row().items_center().gap(4.0)
        .focusable()
        .cursor_pointer()
        .hover(|s| s.border_color(hover_border))
        .child(
            ui_element::label(display)
                .text_color(if spec.value.is_some() { text_color } else { muted })
                .text_size(13.0).grow()
        )
        .child(
            ui_element::icon("chevron-down")
                .w(16.0).h(16.0)
                .text_color(muted)
        )
}
