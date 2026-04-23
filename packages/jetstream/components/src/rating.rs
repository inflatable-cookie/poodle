//! Rating — Jetstream star rating backed by RatingSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::RatingSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_rating(spec: &RatingSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let active = resolve_color(theme, spec.active_color_token());
    let inactive = resolve_color(theme, spec.inactive_color_token());

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let icon_size = rem_to_px(size_font_rem(effective_size));
    // Gap between stars: 0.125rem (2px equivalent) — constant rem value
    let gap = rem_to_px(0.125);

    let mut el = ui_element::div().flex_row().gap(gap);

    for i in 0..spec.max {
        let is_filled = (i as f64) < spec.value;
        let color = if is_filled { active } else { inactive };
        el = el.child(ui_element::icon("star").w(icon_size).h(icon_size).text_color(color));
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
