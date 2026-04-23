//! DetailSection specimen — labeled content section within a detail view.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::detail_section::js_detail_section;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, DetailSectionSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    div().flex_col().gap(24.0)
        .child(group("With title, description and body", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Overview")
                    .with_description("General information about this item."),
                theme,
                vec![label("Section body content goes here.").text_color(text_primary)],
                None,
            )
        ))
        .child(group("With title and trailing actions", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Members"),
                theme,
                vec![label("Body content.").text_color(text_primary)],
                Some(
                    button("Edit")
                        .text_color(accent)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md)))
                        .focusable()
                        .cursor_pointer(),
                ),
            )
        ))
        .child(group("Empty body, no separator", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Empty Section")
                    .with_separated(false),
                theme,
                vec![],
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
