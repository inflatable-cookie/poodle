use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Radio};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, RadioSpec};
use std::sync::Arc;

fn radio_select(
    state: &AppState,
    group: &'static str,
    value: &'static str,
) -> Arc<dyn Fn(bool) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |checked| {
        if checked {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: group.to_string(),
                value: value.to_string(),
            });
        }
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state
        .specimens
        .text
        .get("radio-shipping")
        .cloned()
        .unwrap_or_else(|| "standard".to_string());

    let option = |value: &'static str, label: &'static str| {
        Radio::from_spec(
            RadioSpec::new()
                .with_name("shipping")
                .with_value(value)
                .with_label(label)
                .with_checked(selected == value),
            theme,
        )
        .with_id(format!("radio-{value}"))
        .on_checked_change(radio_select(state, "radio-shipping", value))
    };

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default — three-option native group"),
                    theme,
                ))
                .child(option("standard", "Standard shipping"))
                .child(option("express", "Express shipping"))
                .child(option("overnight", "Overnight shipping")),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("States"),
                    theme,
                ))
                .child(Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Disabled unchecked")
                        .with_disabled(true),
                    theme,
                ))
                .child(Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Disabled checked")
                        .with_checked(true)
                        .with_disabled(true),
                    theme,
                ))
                .child(Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Read-only checked")
                        .with_checked(true)
                        .with_read_only(true),
                    theme,
                ))
                .child(Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Custom selected color")
                        .with_checked(true)
                        .with_selected_color("#7c3aed"),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "radio",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Standard shipping")
                        .with_checked(true)
                        .with_size(size),
                    theme,
                )
                .with_id(format!("specimen-size-{:?}", size))
                .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                Radio::from_spec(
                    RadioSpec::new()
                        .with_label("Standard shipping")
                        .with_checked(true)
                        .with_density(density),
                    theme,
                )
                .with_id(format!("specimen-density-{:?}", density))
                .into_any_element()
            }),
    )
}
