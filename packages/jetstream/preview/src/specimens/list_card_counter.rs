//! ListCardCounter specimen — icon + count states.

use crate::compat::js_list_card_counter;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{InlineTypographyMode, ListCardCounterSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Static",
            secondary,
            div()
                .flex_row()
                .gap(16.0)
                .items_center()
                .child(js_list_card_counter(
                    &ListCardCounterSpec::new("file-text", 24),
                    theme,
                ))
                .child(js_list_card_counter(
                    &ListCardCounterSpec::new("image", 8),
                    theme,
                )),
        ))
        .child(group(
            "Linked (hover for primary text)",
            secondary,
            js_list_card_counter(
                &ListCardCounterSpec::new("file-text", 12).with_href("#docs"),
                theme,
            ),
        ))
        .child(group(
            "Tooltip prop (row only until trigger overlay is composed)",
            secondary,
            js_list_card_counter(
                &ListCardCounterSpec::new("paperclip", 3).with_tooltip("Attachments"),
                theme,
            ),
        ))
        .child(group(
            "Inherit typography",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .text_size(20.0)
                .child(label("Views"))
                .child(js_list_card_counter(
                    &ListCardCounterSpec::new("eye", 128)
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                ))
                .child(label("today")),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
