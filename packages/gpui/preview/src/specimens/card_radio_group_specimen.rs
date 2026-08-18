use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{CardRadioGroup, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::CardRadioGroupSpec;
use poodle_specs::{ChoiceOption, EyebrowSpec};
use std::sync::Arc;

fn card_radio_change(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
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

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Plan selection ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Plan selection"),
                    theme,
                ))
                .child({
                    let selected_plan = state.specimens.text.get("card-radio-plan").cloned();
                    let plan_items = vec![
                        ChoiceOption::new("free", "Free")
                            .with_description("Basic features for personal use. Up to 3 projects."),
                        ChoiceOption::new("pro", "Pro").with_description(
                            "Advanced features for professionals. Unlimited projects.",
                        ),
                        ChoiceOption::new("team", "Team").with_description(
                            "Collaboration tools for teams. Shared workspace included.",
                        ),
                        ChoiceOption::new("enterprise", "Enterprise")
                            .with_description("Custom solutions for large organizations.")
                            .with_disabled(true),
                    ];
                    let mut spec = CardRadioGroupSpec::new(plan_items).with_value("pro");
                    if let Some(ref val) = selected_plan {
                        spec = spec.with_value(val);
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            CardRadioGroup::from_spec(spec, theme)
                                .on_change(card_radio_change(state, "card-radio-plan")),
                        )
                        .when(selected_plan.is_some(), |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!(
                                        "Selected: {}",
                                        selected_plan.as_deref().unwrap_or("")
                                    )),
                            )
                        })
                }),
        )
        // --- Instance size ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Instance size"),
                    theme,
                ))
                .child({
                    let selected_size = state.specimens.text.get("card-radio-size").cloned();
                    let size_items = vec![
                        ChoiceOption::new("sm", "Small").with_description("1 CPU, 512 MB RAM"),
                        ChoiceOption::new("md", "Medium").with_description("2 CPU, 2 GB RAM"),
                        ChoiceOption::new("lg", "Large").with_description("4 CPU, 8 GB RAM"),
                    ];
                    let mut spec = CardRadioGroupSpec::new(size_items);
                    if let Some(ref val) = selected_size {
                        spec = spec.with_value(val);
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            CardRadioGroup::from_spec(spec, theme)
                                .on_change(card_radio_change(state, "card-radio-size")),
                        )
                        .when(selected_size.is_some(), |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!(
                                        "Selected: {}",
                                        selected_size.as_deref().unwrap_or("")
                                    )),
                            )
                        })
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
                .child(CardRadioGroup::from_spec(
                    CardRadioGroupSpec::new(vec![
                        ChoiceOption::new("sm", "Small").with_description("1 CPU, 512 MB RAM"),
                        ChoiceOption::new("md", "Medium").with_description("2 CPU, 2 GB RAM"),
                        ChoiceOption::new("lg", "Large").with_description("4 CPU, 8 GB RAM"),
                    ])
                    .with_value("md")
                    .with_disabled(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "card-radio-group",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                CardRadioGroup::from_spec(
                    CardRadioGroupSpec::new(vec![
                        ChoiceOption::new("a", "Option A").with_description("First option"),
                        ChoiceOption::new("b", "Option B").with_description("Second option"),
                    ])
                    .with_value("a"),
                    theme,
                )
                .with_size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                CardRadioGroup::from_spec(
                    CardRadioGroupSpec::new(vec![
                        ChoiceOption::new("a", "Option A").with_description("First option"),
                        ChoiceOption::new("b", "Option B").with_description("Second option"),
                    ])
                    .with_value("a"),
                    theme,
                )
                .with_density(density)
                .into_any_element()
            }),
    )
}
