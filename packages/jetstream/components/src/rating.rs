//! Rating — Jetstream star rating backed by RatingSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::RatingSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_rating(spec: &RatingSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let active = resolve_color(theme, spec.active_color_token());
    let inactive = resolve_color(theme, spec.inactive_color_token());

    let mut el = ui_element::div().flex_row().gap(2.0);

    for i in 0..spec.max {
        let is_filled = (i as f64) < spec.value;
        let star = if is_filled { "★" } else { "☆" };
        let color = if is_filled { active } else { inactive };
        el = el.child(ui_element::label(star).text_color(color).text_size(16.0));
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
