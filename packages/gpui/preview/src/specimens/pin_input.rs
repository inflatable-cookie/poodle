use gpui::*;
use flint_adapter::ThemeProvider;
use flint_primitives::{PinInputSpec, EyebrowSpec};
use flint_gpui_components::{PinInput, Eyebrow};
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

    div().flex().flex_col().gap(px(24.0))
        // --- 6-digit code ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("6-digit code"), theme))
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
                        .child(if completed { "Code complete!".to_string() } else { format!("Entered: {}", code_value) })
                )
        )
        // --- 4-digit masked ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("4-digit masked"), theme))
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
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    PinInput::from_spec(
                        PinInputSpec::new(6)
                            .with_value("123")
                            .with_disabled(true),
                        theme,
                    )
                )
        )
}
