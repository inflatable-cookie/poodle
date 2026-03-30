//! NavCard — Jetstream navigation card backed by NavCardSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::NavCardSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_nav_card(spec: &NavCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.description_color_token());

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(rem_to_px(1.0)).pr(rem_to_px(1.0)).pt(rem_to_px(0.75)).pb(rem_to_px(0.75))
        .flex_col().gap(rem_to_px(0.25));

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(rem_to_px(0.875)).text_weight(600));

    if let Some(ref desc) = spec.description {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(rem_to_px(0.8125)));
    }

    el
}
