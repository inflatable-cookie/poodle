use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::ColorPickerSpec;
use pug_gpui_components::ColorPicker;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let swatches = vec![
        "#ef4444".to_string(), "#f97316".to_string(), "#eab308".to_string(),
        "#22c55e".to_string(), "#06b6d4".to_string(), "#3b82f6".to_string(),
        "#6366f1".to_string(), "#a855f7".to_string(), "#ec4899".to_string(),
        "#f43f5e".to_string(), "#14b8a6".to_string(), "#8b5cf6".to_string(),
    ];

    // --- Interactive picker ---
    let interactive_open = state.specimens.is_on("color-picker-interactive-open");
    let interactive_value = state.specimens.text.get("color-picker-interactive-value")
        .cloned()
        .unwrap_or_else(|| "#6366f1".to_string());

    let interactive_spec = ColorPickerSpec::new()
        .with_value(&interactive_value)
        .with_open(interactive_open)
        .with_swatches(swatches.clone());

    // --- Default open picker ---
    let open_value = state.specimens.text.get("color-picker-open-value")
        .cloned()
        .unwrap_or_else(|| "#22c55e".to_string());

    let open_spec = ColorPickerSpec::new()
        .with_value(&open_value)
        .with_open(true)
        .with_swatches(swatches.clone());

    // --- Disabled ---
    let disabled_spec = ColorPickerSpec::new()
        .with_value("#22c55e")
        .with_disabled(true);

    div().flex().flex_col().gap(px(16.0)).max_w(px(420.0))
        // --- Interactive picker ---
        .child(section_label("INTERACTIVE PICKER", text_secondary))
        .child(
            ColorPicker::from_spec(interactive_spec, theme)
                .with_id("interactive")
                .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                    this.state.specimens.toggle("color-picker-interactive-open");
                    cx.notify();
                }))
                .on_change(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("color-picker-interactive-value".to_string(), val.to_string());
                    cx.notify();
                }))
        )
        // --- Default open ---
        .child(section_label("DEFAULT OPEN WITH SWATCHES", text_secondary))
        .child(
            ColorPicker::from_spec(open_spec, theme)
                .with_id("open")
                .on_change(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("color-picker-open-value".to_string(), val.to_string());
                    cx.notify();
                }))
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            ColorPicker::from_spec(disabled_spec, theme)
                .with_id("disabled")
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
