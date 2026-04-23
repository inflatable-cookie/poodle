//! Popover specimen — popover panels with different content.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::popover::js_popover;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, PopoverSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // With text content
        .child(group("With text content", secondary,
            js_popover(&PopoverSpec::new(), theme, Some(
                div().flex_col().gap(8.0).p(12.0)
                    .child(label("Popover title").text_color(text_primary).text_size(body_font))
                    .child(label("This is some popover content.").text_color(secondary).text_size(body_font))
            ))
        ))
        // With different content
        .child(group("With rich content", secondary,
            js_popover(&PopoverSpec::new(), theme, Some(
                div().flex_col().gap(8.0).p(12.0)
                    .child(label("Settings").text_color(text_primary).text_size(body_font))
                    .child(label("Adjust your preferences below.").text_color(secondary).text_size(body_font))
                    .child(div().h(1.0).bg(resolve_color(theme, "color.border.subtle")))
                    .child(label("Option A").text_color(text_primary).text_size(body_font))
                    .child(label("Option B").text_color(text_primary).text_size(body_font))
            ))
        ))
        // Empty
        .child(group("Empty (no content)", secondary,
            js_popover(&PopoverSpec::new(), theme, None)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
