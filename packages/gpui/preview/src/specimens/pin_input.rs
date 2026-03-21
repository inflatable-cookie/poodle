use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::PinInputSpec;
use pug_gpui_components::PinInput;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- 6-digit Code ---
        .child(section_label("6-DIGIT CODE", text_secondary))
        .child(
            PinInput::from_spec(
                PinInputSpec::new(6)
                    .with_aria_label("Verification code"),
                theme,
            )
        )
        // --- 4-digit Masked ---
        .child(section_label("4-DIGIT MASKED", text_secondary))
        .child(
            PinInput::from_spec(
                PinInputSpec::new(4)
                    .with_masked(true)
                    .with_aria_label("PIN"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            PinInput::from_spec(
                PinInputSpec::new(6)
                    .with_value("123")
                    .with_disabled(true),
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
