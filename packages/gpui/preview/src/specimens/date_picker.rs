use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, DatePickerSpec, EyebrowSpec};
use poodle_gpui_components::{DatePicker, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let default_open = state.specimens.is_on("date-picker-default-open");
    let default_selected = state.specimens.text.get("date-picker-default-value").cloned();

    let prefilled_open = state.specimens.is_on("date-picker-prefilled-open");
    let prefilled_selected = state.specimens.text.get("date-picker-prefilled-value").cloned()
        .unwrap_or_else(|| "2026-03-14".to_string());

    div().flex().flex_col().gap(px(24.0)).max_w(px(256.0)) // 16rem
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date".to_string());
                    if let Some(ref val) = default_selected {
                        spec.value = Some(val.clone());
                    }
                    DatePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("date-picker-default-open");
                            cx.notify();
                        }))
                        .on_select(cx.listener(|this, date: &str, _w, cx| {
                            this.state.specimens.text.insert("date-picker-default-value".to_string(), date.to_string());
                            this.state.specimens.toggles.insert("date-picker-default-open".to_string(), false);
                            cx.notify();
                        }))
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Selected: {}", default_selected.as_deref().unwrap_or("(none)")))
                )
        )
        // --- With default value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default value"), theme))
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.value = Some(prefilled_selected.clone());
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled date".to_string());
                    DatePicker::from_spec(spec, theme)
                        .with_id("with-value")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("date-picker-prefilled-open");
                            cx.notify();
                        }))
                        .on_select(cx.listener(|this, date: &str, _w, cx| {
                            this.state.specimens.text.insert("date-picker-prefilled-value".to_string(), date.to_string());
                            this.state.specimens.toggles.insert("date-picker-prefilled-open".to_string(), false);
                            cx.notify();
                        }))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.placeholder = "Disabled".to_string();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date picker".to_string());
                    DatePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("size-xs").size(ControlSize::Xs))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("size-sm").size(ControlSize::Sm))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("size-md").size(ControlSize::Md))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("size-lg").size(ControlSize::Lg))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("size-xl").size(ControlSize::Xl))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("density-compact").with_density(ControlDensity::Compact))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("density-default").with_density(ControlDensity::Default))
                        .child(DatePicker::from_spec(DatePickerSpec::new(), theme)
                            .with_id("density-comfortable").with_density(ControlDensity::Comfortable))
                )
        )
}
