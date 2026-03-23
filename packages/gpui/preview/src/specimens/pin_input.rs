use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::PinInputSpec;
use pug_gpui_components::PinInput;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let code_value = state.specimens.text.get("pin-input-code").cloned()
        .unwrap_or_default();
    let pin_value = state.specimens.text.get("pin-input-pin").cloned()
        .unwrap_or_default();
    let completed = state.specimens.is_on("pin-input-complete");

    div().flex().flex_col().gap(px(16.0))
        // --- 6-digit Code ---
        .child(section_label("6-DIGIT CODE", text_secondary))
        .child(
            PinInput::from_spec(
                PinInputSpec::new(6)
                    .with_value(&code_value)
                    .with_aria_label("Verification code"),
                theme,
            )
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("pin-input-code".to_string(), val.to_string());
                cx.notify();
            }))
            .on_complete(cx.listener(|this, _val: &str, _w, cx| {
                this.state.specimens.toggles.insert("pin-input-complete".to_string(), true);
                cx.notify();
            }))
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(color_to_hsla(if completed { text_primary } else { text_secondary }))
                .child(if completed { "✓ Code complete!".to_string() } else { format!("Entered: {}", code_value) })
        )
        // --- 4-digit Masked ---
        .child(section_label("4-DIGIT MASKED", text_secondary))
        .child(
            PinInput::from_spec(
                PinInputSpec::new(4)
                    .with_value(&pin_value)
                    .with_masked(true)
                    .with_aria_label("PIN"),
                theme,
            )
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("pin-input-pin".to_string(), val.to_string());
                cx.notify();
            }))
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
