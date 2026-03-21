//! Box specimen — container with padding and overflow.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::bx::js_box;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{BoxSpec, PaddingScale};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let border = resolve_color(theme, "semantic.color.border.subtle");

    div().flex_col().gap(24.0)
        .child(group("With padding", secondary,
            div().border(1.0).border_color(border).rounded(4.0)
                .child(js_box(&BoxSpec::new().with_padding(PaddingScale::Md), theme, vec![
                    label("Content inside a padded box").text_color(text_primary).text_size(13.0),
                ]))
        ))
        .child(group("Large padding", secondary,
            div().border(1.0).border_color(border).rounded(4.0)
                .child(js_box(&BoxSpec::new().with_padding(PaddingScale::Lg), theme, vec![
                    label("Large padded box").text_color(text_primary).text_size(13.0),
                ]))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
