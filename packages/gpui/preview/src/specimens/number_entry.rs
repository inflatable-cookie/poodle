use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{NumberEntrySpec, ValidationState};
use pug_gpui_components::NumberEntry;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            NumberEntry::from_spec(
                NumberEntrySpec::new(1.0)
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_aria_label("Quantity"),
                theme,
            )
        )
        // --- With Steppers ---
        .child(section_label("WITH STEPPERS", text_secondary))
        .child(
            NumberEntry::from_spec(
                NumberEntrySpec::new(29.99)
                    .with_min(0.0)
                    .with_step(0.01)
                    .with_aria_label("Price"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            NumberEntry::from_spec(
                NumberEntrySpec::new(42.0)
                    .with_disabled(true),
                theme,
            )
        )
        // --- Invalid ---
        .child(section_label("INVALID", text_secondary))
        .child(
            NumberEntry::from_spec(
                NumberEntrySpec::new(-5.0)
                    .with_min(0.0)
                    .with_validation_state(ValidationState::Invalid),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
