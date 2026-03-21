use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonVariant, SplitButtonSpec};
use pug_gpui_components::SplitButton;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let last_action = state.specimens.text.get("split-btn-action")
        .cloned()
        .unwrap_or_else(|| String::from("(none)"));

    div().flex().flex_col().gap(px(16.0))
        // --- Primary variant ---
        .child(section_label("PRIMARY VARIANT", text_secondary))
        .child(
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Save"),
                theme,
            )
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.text.insert(
                    "split-btn-action".to_string(),
                    "click: Save".to_string(),
                );
                cx.notify();
            }))
            .on_dropdown(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.text.insert(
                    "split-btn-action".to_string(),
                    "dropdown: toggle".to_string(),
                );
                cx.notify();
            }))
        )
        // --- Secondary variant ---
        .child(section_label("SECONDARY VARIANT", text_secondary))
        .child(
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Export"),
                theme,
            )
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.text.insert(
                    "split-btn-action".to_string(),
                    "click: Export".to_string(),
                );
                cx.notify();
            }))
            .on_dropdown(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.text.insert(
                    "split-btn-action".to_string(),
                    "dropdown: toggle".to_string(),
                );
                cx.notify();
            }))
        )
        // --- Danger tone ---
        .child(section_label("DANGER TONE", text_secondary))
        .child(
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Delete"),
                theme,
            )
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.text.insert(
                    "split-btn-action".to_string(),
                    "click: Delete".to_string(),
                );
                cx.notify();
            }))
        )
        // --- Loading state ---
        .child(section_label("LOADING STATE", text_secondary))
        .child(
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Saving...")
                    .with_disabled(true),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Save")
                    .with_disabled(true),
                theme,
            )
        )
        // --- Last action ---
        .child(section_label("LAST ACTION", text_secondary))
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child(format!("Last action: {}", last_action))
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
