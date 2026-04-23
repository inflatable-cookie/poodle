//! MetaBar specimen — wrapping inline metadata row.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::meta_bar::js_meta_bar;
use poodle_jetstream_components::meta_item::js_meta_item;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, MetaBarSpec, MetaItemSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    let item = |lbl: &str, val: &str| -> JsEl {
        js_meta_item(
            &MetaItemSpec::new().with_label(lbl),
            theme,
            Some(label(val).text_color(primary).text_size(body_font)),
        )
    };

    div().flex_col().gap(24.0)
        .child(group("With separators (default)", secondary,
            js_meta_bar(
                &MetaBarSpec::new(),
                theme,
                vec![
                    item("Status", "Published"),
                    item("Duration", "3:42"),
                    item("Format", "WAV"),
                    item("BPM", "128"),
                ],
            )
        ))
        .child(group("Without separators", secondary,
            js_meta_bar(
                &MetaBarSpec::new().with_show_separators(false),
                theme,
                vec![
                    item("Owner", "Tom"),
                    item("Modified", "2 hours ago"),
                    item("Size", "48 MB"),
                ],
            )
        ))
        .child(group("Single item", secondary,
            js_meta_bar(
                &MetaBarSpec::new(),
                theme,
                vec![item("Version", "1.4.2")],
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
