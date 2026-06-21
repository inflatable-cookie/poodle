//! Separator specimen — orientation, tone, and decorative/semantic modes.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::separator::js_separator;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, RuleTone, SeparatorOrientation, SeparatorSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    let body = move |s: &str| label(s).text_color(primary).text_size(body_font);

    div()
        .flex_col()
        .gap(24.0)
        // --- Horizontal (default) ---
        .child(group(
            "Horizontal (default)",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .self_stretch()
                .child(body("Content above"))
                .child(js_separator(&SeparatorSpec::new(), theme))
                .child(body("Content below")),
        ))
        // --- Tone emphasis: subtle vs default ---
        .child(group(
            "Tone emphasis",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .self_stretch()
                .child(body("Subtle (default)"))
                .child(js_separator(
                    &SeparatorSpec::new().with_tone(RuleTone::Subtle),
                    theme,
                ))
                .child(body("Default"))
                .child(js_separator(
                    &SeparatorSpec::new().with_tone(RuleTone::Default),
                    theme,
                ))
                .child(body("Below")),
        ))
        // --- Vertical ---
        .child(group(
            "Vertical",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .h(60.0)
                .items_center()
                .child(body("Left"))
                .child(js_separator(
                    &SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                    theme,
                ))
                .child(body("Center"))
                .child(js_separator(
                    &SeparatorSpec::new()
                        .with_orientation(SeparatorOrientation::Vertical)
                        .with_tone(RuleTone::Default),
                    theme,
                ))
                .child(body("Right")),
        ))
        // --- Decorative (aria-hidden) ---
        .child(group(
            "Decorative",
            secondary,
            div()
                .flex_col()
                .self_stretch()
                .child(js_separator(
                    &SeparatorSpec::new().with_decorative(true),
                    theme,
                )),
        ))
        // --- Semantic (decorative=false → role=separator) ---
        .child(group(
            "Semantic (decorative=false)",
            secondary,
            div()
                .flex_col()
                .self_stretch()
                .child(js_separator(
                    &SeparatorSpec::new().with_decorative(false),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
