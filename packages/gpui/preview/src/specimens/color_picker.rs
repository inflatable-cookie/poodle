use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{ColorPicker, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ColorInputMode, ColorPickerSpec, EyebrowSpec};
use std::sync::{Arc, Mutex};

fn toggle_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move || {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(key.to_string()));
    })
}

fn change_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let swatches = vec![
        "#ef4444".to_string(),
        "#f97316".to_string(),
        "#eab308".to_string(),
        "#22c55e".to_string(),
        "#3b82f6".to_string(),
        "#6366f1".to_string(),
        "#8b5cf6".to_string(),
        "#ec4899".to_string(),
    ];

    // --- Basic picker ---
    let basic_open = state.specimens.is_on("color-picker-basic-open");
    let basic_value = state
        .specimens
        .text
        .get("color-picker-basic-value")
        .cloned()
        .unwrap_or_else(|| "#6366f1".to_string());

    // --- With swatches ---
    let swatches_open = state.specimens.is_on("color-picker-swatches-open");
    let swatches_value = state
        .specimens
        .text
        .get("color-picker-swatches-value")
        .cloned()
        .unwrap_or_else(|| "#6366f1".to_string());

    // --- With alpha ---
    let alpha_open = state.specimens.is_on("color-picker-alpha-open");
    let alpha_value = state
        .specimens
        .text
        .get("color-picker-alpha-value")
        .cloned()
        .unwrap_or_else(|| "#3b82f6".to_string());

    // --- Default open ---
    let open_value = state
        .specimens
        .text
        .get("color-picker-open-value")
        .cloned()
        .unwrap_or_else(|| "#22c55e".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        // --- Basic picker ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic picker"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            ColorPicker::from_spec(
                                ColorPickerSpec::new()
                                    .with_value(&basic_value)
                                    .with_open(basic_open),
                                theme,
                                "basic",
                            )
                            .with_id("basic")
                            .on_toggle(toggle_handler(
                                &state.node_events,
                                "color-picker-basic-open",
                            ))
                            .on_change(change_handler(
                                &state.node_events,
                                "color-picker-basic-value",
                            )),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Selected: {}", basic_value)),
                        ),
                ),
        )
        // --- With swatches ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With swatches"),
                    theme,
                ))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&swatches_value)
                            .with_open(swatches_open)
                            .with_swatches(swatches.clone()),
                        theme,
                        "swatches",
                    )
                    .with_id("swatches")
                    .on_toggle(toggle_handler(
                        &state.node_events,
                        "color-picker-swatches-open",
                    ))
                    .on_change(change_handler(
                        &state.node_events,
                        "color-picker-swatches-value",
                    )),
                ),
        )
        // --- With alpha ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With alpha"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            ColorPicker::from_spec(
                                ColorPickerSpec::new()
                                    .with_value(&alpha_value)
                                    .with_open(alpha_open)
                                    .with_show_alpha(true),
                                theme,
                                "alpha",
                            )
                            .with_id("alpha")
                            .on_toggle(toggle_handler(
                                &state.node_events,
                                "color-picker-alpha-open",
                            ))
                            .on_change(change_handler(
                                &state.node_events,
                                "color-picker-alpha-value",
                            )),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Selected: {}", alpha_value)),
                        ),
                ),
        )
        // --- Default open, RGB mode ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default open, RGB mode"),
                    theme,
                ))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&open_value)
                            .with_open(true)
                            .with_default_mode(ColorInputMode::Rgb),
                        theme,
                        "open",
                    )
                    .with_id("open")
                    .on_change(change_handler(
                        &state.node_events,
                        "color-picker-open-value",
                    )),
                ),
        )
        // --- Preview only (no input) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preview only (no input)"),
                    theme,
                ))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value(&basic_value)
                            .with_show_input(false)
                            .with_open(true),
                        theme,
                        "preview",
                    )
                    .with_id("preview"),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    ColorPicker::from_spec(
                        ColorPickerSpec::new()
                            .with_value("#22c55e")
                            .with_disabled(true),
                        theme,
                        "disabled",
                    )
                    .with_id("disabled"),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "color-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                ColorPicker::from_spec(
                    ColorPickerSpec::new().with_value("#6366f1"),
                    theme,
                    format!("specimen-size-{size:?}"),
                )
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                ColorPicker::from_spec(
                    ColorPickerSpec::new().with_value("#6366f1"),
                    theme,
                    format!("specimen-density-{density:?}"),
                )
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
            }),
    )
}
