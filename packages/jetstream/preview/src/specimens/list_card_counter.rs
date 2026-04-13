//! ListCardCounter specimen — icon + count states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::list_card_counter::js_list_card_counter;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::ListCardCounterSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group(
            "Static",
            secondary,
            div().flex_row().gap(16.0).items_center()
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
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
