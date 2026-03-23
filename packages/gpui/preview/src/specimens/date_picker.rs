use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::DatePickerSpec;
use pug_gpui_components::DatePicker;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let default_open = state.specimens.is_on("date-picker-default-open");
    let default_selected = state.specimens.text.get("date-picker-default-value").cloned();

    let prefilled_open = state.specimens.is_on("date-picker-prefilled-open");
    let prefilled_selected = state.specimens.text.get("date-picker-prefilled-value").cloned()
        .unwrap_or_else(|| "2026-03-14".to_string());

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
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
        // --- With default value ---
        .child(section_label("WITH DEFAULT VALUE", text_secondary))
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
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = DatePickerSpec::new();
            spec.placeholder = "Disabled".to_string();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled date picker".to_string());
            DatePicker::from_spec(spec, theme).with_id("disabled")
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
