use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::TimeFieldSpec;
use pug_gpui_components::TimeField;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let default_value = state.specimens.text.get("time-field-default")
        .cloned()
        .unwrap_or_default();
    let meeting_value = state.specimens.text.get("time-field-meeting")
        .cloned()
        .unwrap_or_else(|| "14:30".to_string());

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child({
                    let mut spec = TimeFieldSpec::new();
                    if !default_value.is_empty() {
                        spec = spec.with_default_value(&default_value);
                    }
                    spec.aria_label = Some("Start time".to_string());
                    TimeField::from_spec(spec, theme)
                        .with_id("default")
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("time-field-default".to_string(), val.to_string());
                            cx.notify();
                        }))
                })
                .when(!default_value.is_empty(), |d| {
                    d.child(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child(format!("Value: {}", default_value))
                    )
                })
        )
        // --- With default value ---
        .child(section_label("WITH DEFAULT VALUE", text_secondary))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child({
                    let mut spec = TimeFieldSpec::new()
                        .with_default_value(&meeting_value);
                    spec.aria_label = Some("Meeting time".to_string());
                    TimeField::from_spec(spec, theme)
                        .with_id("with-value")
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("time-field-meeting".to_string(), val.to_string());
                            cx.notify();
                        }))
                })
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", meeting_value))
                )
        )
        // --- With min/max constraints ---
        .child(section_label("WITH MIN/MAX CONSTRAINTS", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new()
                .with_default_value("09:00");
            spec.min = Some("08:00".to_string());
            spec.max = Some("18:00".to_string());
            spec.aria_label = Some("Office hours".to_string());
            TimeField::from_spec(spec, theme).with_id("constrained")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new()
                .with_default_value("12:00");
            spec.is_disabled = true;
            TimeField::from_spec(spec, theme).with_id("disabled")
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
