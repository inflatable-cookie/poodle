//! Button specimen — mirrors Svelte ButtonSpecimen.svelte sections.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{ButtonSpec, ButtonTone, ButtonVariant, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Variants
        .child(group("Variants", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Primary"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Secondary"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Ghost"), theme))
        ))
        // Danger tone — variant × tone combinations
        .child(group("Danger tone", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_danger().with_label("Danger primary"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_danger().with_label("Danger secondary"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_danger().with_label("Danger ghost"), theme))
        ))
        // With icons
        .child(group("With icons", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Create").with_leading_icon("+"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Open").with_trailing_icon("→"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Save").with_leading_icon("□").with_trailing_icon("✓"), theme))
        ))
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_size(ControlSize::Sm).with_label("Small"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_size(ControlSize::Md).with_label("Medium"), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_size(ControlSize::Lg).with_label("Large"), theme))
        ))
        // States
        .child(group("States", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Disabled").with_disabled(true), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Loading").with_loading(true), theme))
                .child(js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Disabled secondary").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
