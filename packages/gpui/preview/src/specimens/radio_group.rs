use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, RadioGroup};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ChoiceOption, EyebrowSpec, Orientation, RadioGroupSpec};
use std::sync::Arc;

fn radio_change(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
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

    let plan_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("free", "Free"),
        ChoiceOption::new("pro", "Pro"),
        ChoiceOption::new("enterprise", "Enterprise"),
    ];

    let size_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("sm", "Small"),
        ChoiceOption::new("md", "Medium"),
        ChoiceOption::new("lg", "Large"),
        ChoiceOption::new("xl", "Extra large"),
    ];

    // Read current selections from specimen state, defaulting to contract initial values
    let plan_value = state
        .specimens
        .text
        .get("radio-plan")
        .cloned()
        .unwrap_or_else(|| "pro".to_string());
    let size_value = state
        .specimens
        .text
        .get("radio-size")
        .cloned()
        .unwrap_or_else(|| "md".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Vertical (default) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Vertical (default)"),
                    theme,
                ))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(plan_options.clone()).with_value(plan_value.clone()),
                        theme,
                    )
                    .with_id("radio-plan")
                    .on_change(radio_change(state, "radio-plan")),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", plan_value)),
                ),
        )
        // --- Horizontal ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Horizontal"),
                    theme,
                ))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(size_options)
                            .with_value(size_value.clone())
                            .with_orientation(Orientation::Horizontal),
                        theme,
                    )
                    .with_id("radio-size")
                    .on_change(radio_change(state, "radio-size")),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", size_value)),
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
                .child({
                    let mut spec = RadioGroupSpec::new(plan_options.clone()).with_value("free");
                    spec.is_disabled = true;

                    RadioGroup::from_spec(spec, theme).with_id("radio-disabled")
                }),
        )
        // --- Disabled option ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled option"),
                    theme,
                ))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(vec![
                            ChoiceOption::new("free", "Free"),
                            ChoiceOption::new("pro", "Pro"),
                            ChoiceOption::new("enterprise", "Enterprise").with_disabled(true),
                        ])
                        .with_value("pro"),
                        theme,
                    )
                    .with_id("radio-disabled-option"),
                ),
        )
        // --- Custom selected color ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom selected color"),
                    theme,
                ))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(vec![
                            ChoiceOption::new("free", "Free"),
                            ChoiceOption::new("pro", "Pro"),
                            ChoiceOption::new("enterprise", "Enterprise"),
                        ])
                        .with_value(plan_value.clone())
                        .with_selected_color("#22c55e"),
                        theme,
                    )
                    .with_id("radio-custom-color")
                    .on_change(radio_change(state, "radio-plan")),
                ),
        )
        .into_any_element();

    let make_options = || {
        vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "radio-group",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                RadioGroup::from_spec(
                    RadioGroupSpec::new(make_options())
                        .with_value("pro")
                        .with_orientation(Orientation::Horizontal)
                        .with_size(size),
                    theme,
                )
                .with_id(format!("specimen-size-{:?}", size))
                .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                RadioGroup::from_spec(
                    RadioGroupSpec::new(make_options())
                        .with_value("pro")
                        .with_orientation(Orientation::Horizontal)
                        .with_density(density),
                    theme,
                )
                .with_id(format!("specimen-density-{:?}", density))
                .into_any_element()
            }),
    )
}
