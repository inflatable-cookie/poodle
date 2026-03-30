//! Pill — Jetstream pill/chip component backed by PillSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{PillSize, PillSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Map PillSize to font size in rem (parallels the ControlSize scale).
fn pill_font_rem(size: PillSize) -> f32 {
    match size {
        PillSize::Xs => 0.625,
        PillSize::Sm => 0.6875,
        PillSize::Md => 0.75,
        PillSize::Lg => 0.8125,
        PillSize::Xl => 0.875,
    }
}

/// Map PillSize to horizontal padding in rem.
fn pill_px_rem(size: PillSize) -> f32 {
    match size {
        PillSize::Xs => 0.375,
        PillSize::Sm => 0.4375,
        PillSize::Md => 0.5,
        PillSize::Lg => 0.5625,
        PillSize::Xl => 0.625,
    }
}

/// Map PillSize to vertical padding in rem.
fn pill_py_rem(size: PillSize) -> f32 {
    match size {
        PillSize::Xs => 0.0625,
        PillSize::Sm => 0.0625,
        PillSize::Md => 0.125,
        PillSize::Lg => 0.1875,
        PillSize::Xl => 0.25,
    }
}

pub fn js_pill(spec: &PillSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let font_size = rem_to_px(pill_font_rem(spec.size));
    let pad_x = rem_to_px(pill_px_rem(spec.size));
    let pad_y = rem_to_px(pill_py_rem(spec.size));

    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, spec.text_color_token());

    let mut el = ui_element::label(&spec.label)
        .bg(fill)
        .text_color(text_color)
        .text_size(font_size)
        .rounded(999.0) // pill shape
        .pl(pad_x).pr(pad_x)
        .pt(pad_y).pb(pad_y);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
