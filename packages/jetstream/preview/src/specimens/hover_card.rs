//! HoverCard specimen — hover cards with content.

use crate::compat::js_hover_card;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, HoverCardSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let caption_font = rem_to_px(size_font_rem(ControlSize::Sm));
    let text_primary = resolve_color(theme, "color.text.primary");

    div()
        .flex_col()
        .gap(24.0)
        // With user profile content
        .child(group(
            "With content",
            secondary,
            js_hover_card(
                &HoverCardSpec::new().with_open(true),
                theme,
                Some(
                    div()
                        .flex_col()
                        .gap(8.0)
                        .p(rem_to_px(0.75))
                        .child(
                            label("Jane Doe")
                                .text_color(text_primary)
                                .text_size(body_font),
                        )
                        .child(
                            label("Software Engineer")
                                .text_color(secondary)
                                .text_size(body_font),
                        )
                        .child(
                            label("Joined March 2024")
                                .text_color(secondary)
                                .text_size(caption_font),
                        ),
                ),
            ),
        ))
        // With minimal content
        .child(group(
            "Minimal content",
            secondary,
            js_hover_card(
                &HoverCardSpec::new(),
                theme,
                Some(
                    div().p(rem_to_px(0.75)).child(
                        label("Quick preview")
                            .text_color(text_primary)
                            .text_size(body_font),
                    ),
                ),
            ),
        ))
        // Empty
        .child(group(
            "Empty (no content)",
            secondary,
            js_hover_card(&HoverCardSpec::new(), theme, None),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
