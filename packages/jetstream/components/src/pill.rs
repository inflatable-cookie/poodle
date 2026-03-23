//! Pill — Jetstream pill/chip component backed by PillSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::PillSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_pill(spec: &PillSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, spec.text_color_token());

    let mut el = ui_element::label(&spec.label)
        .bg(fill)
        .text_color(text_color)
        .text_size(12.0)
        .rounded(999.0) // pill shape
        .pl(8.0).pr(8.0)
        .pt(2.0).pb(2.0);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
