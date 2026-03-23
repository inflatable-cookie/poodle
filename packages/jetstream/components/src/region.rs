//! Region — Jetstream semantic region backed by RegionSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::RegionSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_region(spec: &RegionSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let border = resolve_color(theme, spec.border_color_token());
    let label_color = resolve_color(theme, spec.label_color_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let padding = resolve_px(theme, spec.padding_token());

    let mut el = ui_element::div()
        .min_h(spec.min_height_px)
        .rounded(radius)
        .border(1.0)
        .border_color(border)
        .pl(padding).pr(padding)
        .pt(padding).pb(padding);

    if !spec.label.is_empty() {
        el = el.child(
            ui_element::label(&spec.label)
                .text_color(label_color)
                .text_size(12.0)
        );
    }

    for child in children {
        el = el.child(child);
    }

    el
}
