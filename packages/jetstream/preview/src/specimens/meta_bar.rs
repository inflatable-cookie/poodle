//! MetaBar specimen — wrapping inline metadata row.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_code;
use crate::compat::{js_meta_bar, js_meta_bar_sep};
use crate::compat::js_meta_item;
use crate::compat::js_pill;
use crate::compat::{rem_to_px, size_font_rem};

use poodle_specs::{
    CodeSpec, ControlSize, InlineTypographyMode, MetaBarSpec, MetaItemSpec, PillSpec, PillTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    let item = |lbl: &str, val: &str| -> El {
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
        .child(group("Rich children (Code + Pill suppression)", secondary,
            // Code value + a Pill that opts out of its leading dot
            // (Svelte `:has(.poodle-pill)` suppression) via js_meta_bar_sep.
            js_meta_bar_sep(
                &MetaBarSpec::new(),
                theme,
                vec![
                    (
                        js_meta_item(
                            &MetaItemSpec::new().with_label("ID"),
                            theme,
                            Some(js_code(
                                &CodeSpec::new()
                                    .with_content("proj_01JX9G9NVV")
                                    .with_inline(true),
                                theme,
                            )),
                        ),
                        true,
                    ),
                    (
                        js_pill(
                            &PillSpec::new().with_label("Active").with_tone(PillTone::Success),
                            theme,
                        ),
                        false, // pill suppresses its leading dot
                    ),
                    (item("Owner", "Tom"), true),
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
        .child(group("Inherited typography (inline in copy)", secondary,
            div().flex_row().gap(8.0).items_center().text_size(20.0)
                .child(label("Owned by").text_color(secondary))
                .child(js_meta_item(
                    &MetaItemSpec::new()
                        .with_label("Team")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                    Some(label("Platform").text_color(primary).text_size(20.0)),
                ))
                .child(label("since 2024").text_color(secondary))
        ))
        .child(group("Single item", secondary,
            js_meta_bar(
                &MetaBarSpec::new(),
                theme,
                vec![item("Version", "1.4.2")],
            )
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
