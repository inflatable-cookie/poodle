use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Checkbox, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CheckboxSpec, EyebrowSpec};
use std::sync::Arc;

fn checkbox_change(state: &AppState, key: &'static str) -> Arc<dyn Fn(bool) + Send + Sync> {
    let events = state.node_events.clone();
    let init_key = format!("{key}__init");
    Arc::new(move |checked| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetToggle {
            key: init_key.clone(),
            value: true,
        });
        events.push(NodeSpecimenEvent::SetToggle {
            key: key.to_string(),
            value: checked,
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child({
                    let items: &[(&str, &str, bool)] = &[
                        ("checkbox-email", "Enable email notifications", true),
                        ("checkbox-marketing", "Subscribe to marketing emails", false),
                        (
                            "checkbox-terms",
                            "I agree to the terms and conditions",
                            false,
                        ),
                    ];
                    let mut col = div().flex().flex_col().gap(px(10.0));
                    for &(key, label, default) in items {
                        let init_key = format!("{key}__init");
                        let checked = if !state.specimens.is_on(&init_key) {
                            default
                        } else {
                            state.specimens.is_on(key)
                        };
                        col = col.child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_checked(checked).with_label(label),
                                theme,
                            )
                            .with_id(key)
                            .on_change(checkbox_change(state, key)),
                        );
                    }
                    col
                }),
        )
        // --- States ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("States"),
                    theme,
                ))
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_disabled(true)
                            .with_label("Disabled unchecked"),
                        theme,
                    )
                    .with_id("cb-disabled-unchecked"),
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_disabled(true)
                            .with_label("Disabled checked"),
                        theme,
                    )
                    .with_id("cb-disabled-checked"),
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_mixed(true)
                            .with_label("Mixed / indeterminate"),
                        theme,
                    )
                    .with_id("cb-mixed"),
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_read_only(true)
                            .with_label("Read-only checked"),
                        theme,
                    )
                    .with_id("cb-readonly-checked"),
                ),
        )
        // --- Custom selected color ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom selected color"),
                    theme,
                ))
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_label("Billable feature")
                            .with_selected_color("#22c55e"),
                        theme,
                    )
                    .with_id("cb-custom-green"),
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_label("Requires moderation")
                            .with_selected_color("#f59e0b"),
                        theme,
                    )
                    .with_id("cb-custom-amber"),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "checkbox",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Checkbox::from_spec(CheckboxSpec::new().with_label("Accept terms"), theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Checkbox::from_spec(CheckboxSpec::new().with_label("Option"), theme)
                .with_id(format!("specimen-density-{:?}", density))
                .density(density)
                .into_any_element()
        },
    )
}
