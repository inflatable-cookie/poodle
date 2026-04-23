//! Box specimen — container with padding and overflow.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::bx::js_box;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{BoxSpec, ControlSize, PaddingScale};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");
    let border = resolve_color(theme, "color.border.subtle");

    div().flex_col().gap(24.0)
        .child(group("With padding", secondary,
            div().border(1.0).border_color(border).rounded(rem_to_px(0.25))
                .child(js_box(&BoxSpec::new().with_padding(PaddingScale::Md), theme, vec![
                    label("Content inside a padded box").text_color(text_primary).text_size(body_font),
                ]))
        ))
        .child(group("Large padding", secondary,
            div().border(1.0).border_color(border).rounded(rem_to_px(0.25))
                .child(js_box(&BoxSpec::new().with_padding(PaddingScale::Lg), theme, vec![
                    label("Large padded box").text_color(text_primary).text_size(body_font),
                ]))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
