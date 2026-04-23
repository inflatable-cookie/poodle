//! Card specimen — cards with different variants and layouts.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::card::js_card;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CardLayout, CardSpec, CardVariant, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // Default variant
        .child(group("Default", secondary,
            js_card(&CardSpec::new(), theme, vec![
                label("Card title").text_color(text_primary).text_size(body_font),
                label("Card body content goes here.").text_color(secondary).text_size(body_font),
            ])
        ))
        // Outlined variant
        .child(group("Outlined", secondary,
            js_card(&CardSpec::new().with_variant(CardVariant::Outlined), theme, vec![
                label("Outlined card").text_color(text_primary).text_size(body_font),
                label("With visible border.").text_color(secondary).text_size(body_font),
            ])
        ))
        // Horizontal layout
        .child(group("Horizontal layout", secondary,
            js_card(&CardSpec::new().with_layout(CardLayout::Horizontal), theme, vec![
                div().w(rem_to_px(3.0)).h(rem_to_px(3.0)).rounded(rem_to_px(0.5)).bg(resolve_color(theme, "color.accent.base")),
                div().flex_col().gap(4.0)
                    .child(label("Horizontal card").text_color(text_primary).text_size(body_font))
                    .child(label("Content beside media.").text_color(secondary).text_size(body_font)),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
