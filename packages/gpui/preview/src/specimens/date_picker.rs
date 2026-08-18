use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DatePicker, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DatePickerSpec, EyebrowSpec};
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

fn select_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    open_key: &'static str,
    value_key: &'static str,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::Change {
            open_key: open_key.to_string(),
            value_key: value_key.to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let default_open = state.specimens.is_on("date-picker-default-open");
    let default_selected = state
        .specimens
        .text
        .get("date-picker-default-value")
        .cloned();

    let prefilled_open = state.specimens.is_on("date-picker-prefilled-open");
    let prefilled_selected = state
        .specimens
        .text
        .get("date-picker-prefilled-value")
        .cloned()
        .unwrap_or_else(|| "2026-03-14".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(256.0)) // 16rem
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date".to_string());
                    if let Some(ref val) = default_selected {
                        spec.value = Some(val.clone());
                    }
                    DatePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "date-picker-default-open",
                        ))
                        .on_select(select_handler(
                            &state.node_events,
                            "date-picker-default-open",
                            "date-picker-default-value",
                        ))
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!(
                            "Selected: {}",
                            default_selected.as_deref().unwrap_or("(none)")
                        )),
                ),
        )
        // --- With default value ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With default value"),
                    theme,
                ))
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.value = Some(prefilled_selected.clone());
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled date".to_string());
                    DatePicker::from_spec(spec, theme)
                        .with_id("with-value")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "date-picker-prefilled-open",
                        ))
                        .on_select(select_handler(
                            &state.node_events,
                            "date-picker-prefilled-open",
                            "date-picker-prefilled-value",
                        ))
                }),
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
                .child({
                    let mut spec = DatePickerSpec::new();
                    spec.placeholder = "Disabled".to_string();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date picker".to_string());
                    DatePicker::from_spec(spec, theme).with_id("disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                DatePicker::from_spec(DatePickerSpec::new(), theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                DatePicker::from_spec(DatePickerSpec::new(), theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
