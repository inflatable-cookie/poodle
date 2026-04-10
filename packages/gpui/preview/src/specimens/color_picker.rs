use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ColorPickerSpec, ColorInputMode, ControlDensity, ControlSize, EyebrowSpec};
use poodle_gpui_components::{ColorPicker, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let swatches = vec![
        "#ef4444".to_string(), "#f97316".to_string(), "#eab308".to_string(),
        "#22c55e".to_string(), "#3b82f6".to_string(), "#6366f1".to_string(),
        "#8b5cf6".to_string(), "#ec4899".to_string(),
    ];

    // --- Basic picker ---
    let basic_open = state.specimens.is_on("color-picker-basic-open");
    let basic_value = state.specimens.text.get("color-picker-basic-value")
        .cloned()
        .unwrap_or_else(|| "#6366f1".to_string());

    // --- With swatches ---
    let swatches_open = state.specimens.is_on("color-picker-swatches-open");
    let swatches_value = state.specimens.text.get("color-picker-swatches-value")
        .cloned()
        .unwrap_or_else(|| "#6366f1".to_string());

    // --- With alpha ---
    let alpha_open = state.specimens.is_on("color-picker-alpha-open");
    let alpha_value = state.specimens.text.get("color-picker-alpha-value")
        .cloned()
        .unwrap_or_else(|| "#3b82f6".to_string());

    // --- Default open ---
    let open_value = state.specimens.text.get("color-picker-open-value")
        .cloned()
        .unwrap_or_else(|| "#22c55e".to_string());

    div().flex().flex_col().gap(px(24.0)).max_w(px(420.0))
        // --- Basic picker ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic picker"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            ColorPicker::from_spec(
                                ColorPickerSpec::new()
                                    .with_value(&basic_value)
                                    .with_open(basic_open),
                                theme,
                            )
                            .with_id("basic")
                            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                                this.state.specimens.toggle("color-picker-basic-open");
                                cx.notify();
                            }))
                            .on_change(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert("color-picker-basic-value".to_string(), val.to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child(format!("Selected: {}", basic_value))
                        )
                )
        )

        // --- With swatches ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With swatches"), theme))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&swatches_value)
                            .with_open(swatches_open)
                            .with_swatches(swatches.clone()),
                        theme,
                    )
                    .with_id("swatches")
                    .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                        this.state.specimens.toggle("color-picker-swatches-open");
                        cx.notify();
                    }))
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("color-picker-swatches-value".to_string(), val.to_string());
                        cx.notify();
                    }))
                )
        )

        // --- With alpha ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With alpha"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            ColorPicker::from_spec(
                                ColorPickerSpec::new()
                                    .with_value(&alpha_value)
                                    .with_open(alpha_open)
                                    .with_show_alpha(true),
                                theme,
                            )
                            .with_id("alpha")
                            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                                this.state.specimens.toggle("color-picker-alpha-open");
                                cx.notify();
                            }))
                            .on_change(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert("color-picker-alpha-value".to_string(), val.to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child(format!("Selected: {}", alpha_value))
                        )
                )
        )

        // --- Default open, RGB mode ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default open, RGB mode"), theme))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&open_value)
                            .with_open(true)
                            .with_default_mode(ColorInputMode::Rgb),
                        theme,
                    )
                    .with_id("open")
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("color-picker-open-value".to_string(), val.to_string());
                        cx.notify();
                    }))
                )
        )

        // --- Preview only (no input) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Preview only (no input)"), theme))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&basic_value)
                            .with_show_input(false)
                            .with_open(true),
                        theme,
                    )
                    .with_id("preview")
                )
        )

        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value("#22c55e")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled")
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("size-xs").size(ControlSize::Xs))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("size-sm").size(ControlSize::Sm))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("size-md").size(ControlSize::Md))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("size-lg").size(ControlSize::Lg))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("size-xl").size(ControlSize::Xl))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("density-compact").with_density(ControlDensity::Compact))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("density-default").with_density(ControlDensity::Default))
                        .child(ColorPicker::from_spec(
                            ColorPickerSpec::new().with_value("#6366f1"), theme,
                        ).with_id("density-comfortable").with_density(ControlDensity::Comfortable))
                )
        )
}
