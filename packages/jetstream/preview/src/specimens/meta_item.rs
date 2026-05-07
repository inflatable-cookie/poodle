//! MetaItem specimen — label/value pair for metadata rows.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::meta_item::js_meta_item;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, InlineTypographyMode, MetaItemSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With label + text value", secondary,
            js_meta_item(
                &MetaItemSpec::new().with_label("Status"),
                theme,
                Some(label("Published").text_color(primary).text_size(body_font)),
            )
        ))
        .child(group("Value only (no label)", secondary,
            js_meta_item(
                &MetaItemSpec::new(),
                theme,
                Some(label("Stem waveform").text_color(primary).text_size(body_font)),
            )
        ))
        .child(group("Label only (no value slot)", secondary,
            js_meta_item(
                &MetaItemSpec::new().with_label("Type"),
                theme,
                None,
            )
        ))
        .child(group("Multiple items side by side", secondary,
            div().flex_row().gap(16.0).flex_wrap().items_center()
                .child(js_meta_item(
                    &MetaItemSpec::new().with_label("Duration"),
                    theme,
                    Some(label("3:42").text_color(primary).text_size(body_font)),
                ))
                .child(js_meta_item(
                    &MetaItemSpec::new().with_label("Format"),
                    theme,
                    Some(label("WAV 24-bit").text_color(primary).text_size(body_font)),
                ))
                .child(js_meta_item(
                    &MetaItemSpec::new().with_label("BPM"),
                    theme,
                    Some(label("128").text_color(primary).text_size(body_font)),
                ))
        ))
        .child(group("Inherit typography", secondary,
            div().flex_row().gap(8.0).items_center().text_size(20.0)
                .child(label("Owner"))
                .child(js_meta_item(
                    &MetaItemSpec::new()
                        .with_label("Team")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                    Some(label("Platform").text_color(primary).text_size(20.0)),
                ))
                .child(label("today"))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
