//! Spacer specimen — flexible space filler.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::spacer::js_spacer;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::SpacerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.default");

    // A helper that renders a labelled box so the spacer's effect is visible
    let labeled_box = |text: &str| -> JsEl {
        div()
            .px(12.0).py(6.0)
            .border_1().border_color(border)
            .rounded(4.0)
            .child(label(text).text_color(secondary).text_size(11.0))
    };

    div().flex_col().gap(24.0)
        .child(group("Flex-row with growing spacer", secondary,
            div().flex_row().items_center().gap(8.0)
                .w(400.0)
                .border_1().border_color(border).rounded(4.0).p(8.0)
                .child(labeled_box("Left"))
                .child(js_spacer(&SpacerSpec::new().with_grow(1.0)))
                .child(labeled_box("Right"))
        ))
        .child(group("Spacer with min-size", secondary,
            div().flex_row().items_center()
                .border_1().border_color(border).rounded(4.0).p(8.0)
                .child(labeled_box("A"))
                .child(js_spacer(&SpacerSpec::new().with_grow(0.0).with_min_size(64.0))
                    .bg(tint(accent, 0.12)).rounded(2.0))
                .child(labeled_box("B"))
        ))
        .child(group("No-grow fixed spacer (min-size only)", secondary,
            div().flex_row().items_center()
                .border_1().border_color(border).rounded(4.0).p(8.0)
                .child(labeled_box("X"))
                .child(js_spacer(&SpacerSpec::new().with_grow(0.0).with_min_size(32.0))
                    .bg(tint(accent, 0.08)).rounded(2.0))
                .child(labeled_box("Y"))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
