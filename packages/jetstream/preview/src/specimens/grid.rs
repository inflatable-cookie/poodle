//! Grid specimen — flex-wrap grid layouts.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::grid::js_grid;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::GridSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    let cell = |text: &str| label(text).text_color(text_primary).text_size(12.0)
        .px(12.0).py(8.0).bg(tint(accent, 0.10)).rounded(4.0);

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
