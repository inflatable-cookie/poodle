use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{TimeFieldSpec, EyebrowSpec};
use poodle_gpui_components::{TimeField, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let default_value = state.specimens.text.get("time-field-default")
        .cloned()
        .unwrap_or_default();
    let meeting_value = state.specimens.text.get("time-field-meeting")
        .cloned()
        .unwrap_or_else(|| "14:30".to_string());

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
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
        )
        // --- With default value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default value"), theme))
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
        )
        // --- With min/max constraints ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With min/max constraints"), theme))
                .child({
                    let mut spec = TimeFieldSpec::new()
                        .with_default_value("09:00");
                    spec.min = Some("08:00".to_string());
                    spec.max = Some("18:00".to_string());
                    spec.aria_label = Some("Office hours".to_string());
                    TimeField::from_spec(spec, theme).with_id("constrained")
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = TimeFieldSpec::new()
                        .with_default_value("12:00");
                    spec.is_disabled = true;
                    TimeField::from_spec(spec, theme).with_id("disabled")
                })
        )
}
