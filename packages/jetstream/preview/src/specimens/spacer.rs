//! Spacer specimen — flexible space filler in flex contexts.
//!
//! Mirrors the GPUI specimen (push-apart + between-three-items) and keeps the
//! min-size / no-grow coverage the Jetstream target already had. Every spacer
//! is a real `js_spacer`; the surrounding boxes resolve tokens.

use crate::compat::js_spacer;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::SpacerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.default");

    let boxed = |text: &str| -> El {
        div()
            .px(12.0)
            .py(6.0)
            .border_1()
            .border_color(border)
            .rounded(4.0)
            .child(label(text).text_color(secondary).text_size(11.0))
    };

    div()
        .flex_col()
        .gap(24.0)
        // --- Push items apart (logo / sign-in) ---
        .child(group(
            "Push items apart",
            secondary,
            div()
                .flex_row()
                .items_center()
                .gap(8.0)
                .w(400.0)
                .border_1()
                .border_color(border)
                .rounded(4.0)
                .p(8.0)
                .child(boxed("Logo"))
                .child(js_spacer(&SpacerSpec::new().with_grow(1.0)))
                .child(boxed("Sign in")),
        ))
        // --- Between three items (left / center / right) ---
        .child(group(
            "Between three items",
            secondary,
            div()
                .flex_row()
                .items_center()
                .gap(8.0)
                .w(400.0)
                .border_1()
                .border_color(border)
                .rounded(4.0)
                .p(8.0)
                .child(boxed("Left"))
                .child(js_spacer(&SpacerSpec::new().with_grow(1.0)))
                .child(boxed("Center"))
                .child(js_spacer(&SpacerSpec::new().with_grow(1.0)))
                .child(boxed("Right")),
        ))
        // --- Spacer with min-size (grow 0, reserved width) ---
        .child(group(
            "Spacer with min-size",
            secondary,
            div()
                .flex_row()
                .items_center()
                .border_1()
                .border_color(border)
                .rounded(4.0)
                .p(8.0)
                .child(boxed("A"))
                .child(
                    js_spacer(&SpacerSpec::new().with_grow(0.0).with_min_size(64.0))
                        .bg(tint(accent, 0.12))
                        .rounded(2.0),
                )
                .child(boxed("B")),
        ))
        // --- No-grow fixed spacer (min-size only) ---
        .child(group(
            "No-grow fixed spacer",
            secondary,
            div()
                .flex_row()
                .items_center()
                .border_1()
                .border_color(border)
                .rounded(4.0)
                .p(8.0)
                .child(boxed("X"))
                .child(
                    js_spacer(&SpacerSpec::new().with_grow(0.0).with_min_size(32.0))
                        .bg(tint(accent, 0.08))
                        .rounded(2.0),
                )
                .child(boxed("Y")),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
