use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, Eyebrow, FormActions};
use crate::specimens::specimen_axes::density_key;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, EyebrowSpec, FormActionAlign,
    FormActionDangerItem, FormActionsSpec,
};
use std::sync::Arc;

fn set_text_click(state: &AppState, value: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "form-action-last".to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state
        .specimens
        .text
        .get("form-action-last")
        .cloned()
        .unwrap_or_default();
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- End-aligned (default) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("End-aligned (default)"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(FormActionsSpec::new(), theme)
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Cancel"),
                                theme,
                            )
                            .with_id("fa-cancel-end")
                            .on_click(set_text_click(state, "Cancel")),
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save changes"),
                                theme,
                            )
                            .with_id("fa-save-end")
                            .on_click(set_text_click(state, "Save changes")),
                        ),
                ),
        )
        // --- Start-aligned ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Start-aligned"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new().with_align(FormActionAlign::Start),
                        theme,
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Back"),
                            theme,
                        )
                        .with_id("fa-back-start"),
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Continue"),
                            theme,
                        )
                        .with_id("fa-continue-start"),
                    ),
                ),
        )
        // --- Space between ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Space between"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new().with_align(FormActionAlign::Between),
                        theme,
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete"),
                            theme,
                        )
                        .with_id("fa-delete-between"),
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Save"),
                            theme,
                        )
                        .with_id("fa-save-between")
                        .on_click(set_text_click(state, "Save")),
                    ),
                ),
        )
        // --- Responsive danger actions (inline danger group + overflow menu) ---
        // `with_danger_action` renders the inline destructive content; supplying
        // `dangerItems` on the spec turns on the ghost ellipsis overflow trigger
        // (contract §2 danger menu / §8 Responsive Swap). Both render together
        // here since GPUI's render-immediate model has no container-query channel.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Responsive danger actions (overflow)"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new()
                            .with_danger_item(FormActionDangerItem::new("Discard draft"))
                            .with_danger_item(FormActionDangerItem::new("Reset form")),
                        theme,
                    )
                    .with_danger_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Discard draft"),
                            theme,
                        )
                        .with_id("fa-discard-danger"),
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("fa-cancel-danger"),
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Save changes"),
                            theme,
                        )
                        .with_id("fa-save-danger"),
                    ),
                ),
        )
        // --- Submitting state (loading primary, disabled cancel) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Submitting"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(FormActionsSpec::new(), theme)
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Cancel")
                                    .with_disabled(true),
                                theme,
                            )
                            .with_id("fa-cancel-busy"),
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Saving…")
                                    .with_loading(true),
                                theme,
                            )
                            .with_id("fa-save-busy"),
                        ),
                ),
        )
        // --- Footer-embedded (showTopSeparation=false) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Footer-embedded (no top separation)"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new().with_top_separation(false),
                        theme,
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("fa-cancel-footer"),
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Save changes"),
                            theme,
                        )
                        .with_id("fa-save-footer"),
                    ),
                ),
        )
        // --- Bordered separation (showTopBorder=true) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Bordered separation"),
                    theme,
                ))
                .child(
                    FormActions::from_spec(FormActionsSpec::new().with_top_border(true), theme)
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Cancel"),
                                theme,
                            )
                            .with_id("fa-cancel-border"),
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save changes"),
                                theme,
                            )
                            .with_id("fa-save-border"),
                        ),
                ),
        )
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action)),
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "form-actions",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            density_row(theme, density).into_any_element()
        }),
    )
}

/// One labelled row of the density ladder: a secondary + primary pair laid out
/// by a `FormActions` at the given density. Density changes the inline gap and
/// top separation only — child button size stays constant (contract §8).
fn density_row(theme: &GpuiThemeProvider, density: ControlDensity) -> Div {
    let label = density_key(density);
    div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(
            FormActions::from_spec(FormActionsSpec::new().with_density(density), theme)
                .with_action(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Cancel"),
                        theme,
                    )
                    .with_id(format!("fa-density-cancel-{label}")),
                )
                .with_action(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Save changes"),
                        theme,
                    )
                    .with_id(format!("fa-density-save-{label}")),
                ),
        )
}
