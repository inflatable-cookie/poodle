//! Grid specimen — flex-wrap grid layouts.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::grid::js_grid;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, GridSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let caption_font = rem_to_px(size_font_rem(ControlSize::Sm));
    let text_primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    let cell = |text: &str| label(text).text_color(text_primary).text_size(caption_font)
        .px(rem_to_px(0.75)).py(rem_to_px(0.5)).bg(tint(accent, 0.10)).rounded(rem_to_px(0.25));

    div().flex_col().gap(24.0)
        .child(group("Default grid", secondary,
            js_grid(&GridSpec::new(), theme, vec![
                cell("Cell 1"), cell("Cell 2"), cell("Cell 3"),
                cell("Cell 4"), cell("Cell 5"), cell("Cell 6"),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
