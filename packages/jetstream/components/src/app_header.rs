//! AppHeader — Jetstream app header backed by AppHeaderSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{AppHeaderSpec, ControlDensity, ControlSize};

use crate::presentation::{panel_space_x_rem, rem_to_px, size_font_rem};
use crate::theme_ext::resolve_color;

pub fn js_app_header(spec: &AppHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.background_token());
    let border = resolve_color(theme, "color.border.subtle");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    // Height: 3rem is the app-header canonical height; expressed via helper, not literal px.
    let height = rem_to_px(3.0);
    // Horizontal padding from density helper — Default density maps to 1rem panel space.
    let pad_x = rem_to_px(panel_space_x_rem(ControlDensity::Default));
    // Title uses Md control size as the app header baseline.
    let title_font = rem_to_px(size_font_rem(ControlSize::Md));
    // Subtitle is rendered slightly smaller than the title.
    let subtitle_font = rem_to_px(size_font_rem(ControlSize::Md) * 0.875);
    // Inline gap between title and subtitle.
    let gap = rem_to_px(0.5);

    let mut title_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap);

    if let Some(ref title) = spec.title {
        title_row = title_row.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(title_font)
                .text_weight(600),
        );
    }

    if let Some(ref subtitle) = spec.subtitle {
        title_row = title_row.child(
            ui_element::label(subtitle)
                .text_color(text_secondary)
                .text_size(subtitle_font),
        );
    }

    ui_element::div()
        .bg(fill)
        .border_b_1()
        .border_color(border)
        .h(height)
        .grow()
        .flex_row()
        .items_center()
        .pl(pad_x)
        .pr(pad_x)
        .child(title_row)
}
