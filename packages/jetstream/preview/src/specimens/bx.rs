//! Box specimen — neutral container: padding, fixed dimensions, overflow.
//!
//! All groups are spec-driven: dimensions and overflow resolve through the
//! `BoxSpec` / `js_box` path, not hand-coded wrappers.

use crate::compat::js_box;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{BoxSpec, ControlSize, Overflow, PaddingScale};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");
    let border = resolve_color(theme, "color.border.subtle");

    // Outer chrome makes the box bounds visible without styling the Box itself
    // (Box is a neutral container — background/border are out of contract scope).
    let frame = |inner: El| {
        div()
            .border(1.0)
            .border_color(border)
            .rounded(rem_to_px(0.25))
            .child(inner)
    };

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Default (no padding)",
            secondary,
            frame(js_box(
                &BoxSpec::new(),
                theme,
                vec![label("Flush content")
                    .text_color(text_primary)
                    .text_size(body_font)],
            )),
        ))
        .child(group(
            "With padding (md)",
            secondary,
            frame(js_box(
                &BoxSpec::new().with_padding(PaddingScale::Md),
                theme,
                vec![label("Content inside a padded box")
                    .text_color(text_primary)
                    .text_size(body_font)],
            )),
        ))
        .child(group(
            "Large padding (lg)",
            secondary,
            frame(js_box(
                &BoxSpec::new().with_padding(PaddingScale::Lg),
                theme,
                vec![label("Large padded box")
                    .text_color(text_primary)
                    .text_size(body_font)],
            )),
        ))
        .child(group(
            "Fixed 12x6rem (md padding)",
            secondary,
            frame(js_box(
                &BoxSpec::new()
                    .with_padding(PaddingScale::Md)
                    .with_width("12rem")
                    .with_height("6rem"),
                theme,
                vec![label("Fixed 12x6rem")
                    .text_color(text_primary)
                    .text_size(body_font)],
            )),
        ))
        .child(group(
            "Overflow hidden (10x3rem)",
            secondary,
            frame(js_box(
                &BoxSpec::new()
                    .with_padding(PaddingScale::Sm)
                    .with_width("10rem")
                    .with_height("3rem")
                    .with_overflow(Overflow::Hidden),
                theme,
                vec![label("This long line is clipped by the box boundary")
                    .text_color(text_primary)
                    .text_size(body_font)],
            )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
