use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, SplitButton};
use poodle_specs::{ButtonTone, ButtonVariant, EyebrowSpec, SplitButtonSpec, SplitMenuItem};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state
        .specimens
        .text
        .get("split-btn-action")
        .cloned()
        .unwrap_or_else(|| String::from("(none)"));

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Primary variant ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Primary variant"),
                    theme,
                ))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Save")
                            .with_items(vec![
                                SplitMenuItem::action("save-draft", "Save as draft"),
                                SplitMenuItem::action("save-template", "Save as template"),
                                SplitMenuItem::Separator,
                                SplitMenuItem::action("discard", "Discard changes"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("split-btn-action".to_string(), "click: Save".to_string());
                        cx.notify();
                    }))
                    .on_dropdown(cx.listener(
                        |this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert(
                                "split-btn-action".to_string(),
                                "dropdown: toggle".to_string(),
                            );
                            cx.notify();
                        },
                    )),
                ),
        )
        // --- Secondary variant ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Secondary variant"),
                    theme,
                ))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Export")
                            .with_items(vec![
                                SplitMenuItem::action("csv", "Export as CSV"),
                                SplitMenuItem::action("json", "Export as JSON"),
                                SplitMenuItem::action("pdf", "Export as PDF"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("split-btn-action".to_string(), "click: Export".to_string());
                        cx.notify();
                    }))
                    .on_dropdown(cx.listener(
                        |this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert(
                                "split-btn-action".to_string(),
                                "dropdown: toggle".to_string(),
                            );
                            cx.notify();
                        },
                    )),
                ),
        )
        // --- Danger tone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Danger tone"),
                    theme,
                ))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_tone(ButtonTone::Danger)
                            .with_label("Delete")
                            .with_items(vec![
                                SplitMenuItem::action("delete-selected", "Delete selected"),
                                SplitMenuItem::action("delete-all", "Delete all"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("split-btn-action".to_string(), "click: Delete".to_string());
                        cx.notify();
                    })),
                ),
        )
        // --- Variant x tone matrix ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Variant x tone (default / danger / success)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(matrix_row(theme, ButtonVariant::Primary))
                        .child(matrix_row(theme, ButtonVariant::Secondary))
                        .child(matrix_row(theme, ButtonVariant::Ghost)),
                ),
        )
        // --- Dropdown menu open ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Dropdown menu open (items + separator)"),
                    theme,
                ))
                .child(SplitButton::from_spec(
                    SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Save")
                        .with_items(vec![
                            SplitMenuItem::action("save-draft", "Save as draft"),
                            SplitMenuItem::action("save-template", "Save as template"),
                            SplitMenuItem::Separator,
                            SplitMenuItem::action("discard", "Discard changes"),
                        ]),
                    theme,
                )
                .open(true)),
        )
        // --- Loading state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading state"),
                    theme,
                ))
                .child(SplitButton::from_spec(
                    SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Saving...")
                        .with_loading(true),
                    theme,
                )),
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
                .child(SplitButton::from_spec(
                    SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Save")
                        .with_disabled(true),
                    theme,
                )),
        )
        // --- Submit semantics ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Submit semantics"),
                    theme,
                ))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Save changes")
                            .with_items(vec![
                                SplitMenuItem::action("save", "Save changes"),
                                SplitMenuItem::action("save-close", "Save & close"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "submit: Save changes".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_dropdown(cx.listener(
                        |this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert(
                                "split-btn-action".to_string(),
                                "dropdown: toggle".to_string(),
                            );
                            cx.notify();
                        },
                    )),
                ),
        )
        // --- Constrained scroll container ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Constrained scroll container"),
                    theme,
                ))
                .child(
                    div()
                        .max_h(px(120.0))
                        .overflow_hidden()
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .p(px(12.0))
                        .child(div().h(px(60.0))) // spacer
                        .child(
                            SplitButton::from_spec(
                                SplitButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Queue actions")
                                    .with_items(vec![
                                        SplitMenuItem::action("queue-retry", "Retry failed"),
                                        SplitMenuItem::action("queue-clear", "Clear queue"),
                                        SplitMenuItem::action("queue-export", "Export log"),
                                    ]),
                                theme,
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "split-btn-action".to_string(),
                                    "click: Queue actions".to_string(),
                                );
                                cx.notify();
                            }))
                            .on_dropdown(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "split-btn-action".to_string(),
                                        "dropdown: toggle".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
                        ),
                ),
        )
        // --- Last action ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Last action"),
                    theme,
                ))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Last action: {}", last_action)),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "split-button",
        examples,
        |size, theme: &GpuiThemeProvider| {
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(size)
                    .with_label("Action")
                    .with_items(vec![
                        SplitMenuItem::action("a", "Action A"),
                        SplitMenuItem::action("b", "Action B"),
                    ]),
                theme,
            )
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            SplitButton::from_spec(
                SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Action")
                    .with_items(vec![
                        SplitMenuItem::action("a", "Action A"),
                        SplitMenuItem::action("b", "Action B"),
                    ]),
                theme,
            )
            .density(density)
            .into_any_element()
        },
    )
}

/// One row of the variant x tone matrix: default / danger / success for a variant.
fn matrix_row(theme: &GpuiThemeProvider, variant: ButtonVariant) -> Div {
    let cell = |tone: ButtonTone, label: &str| {
        SplitButton::from_spec(
            SplitButtonSpec::new()
                .with_variant(variant)
                .with_tone(tone)
                .with_label(label)
                .with_items(vec![
                    SplitMenuItem::action("a", "Action A"),
                    SplitMenuItem::action("b", "Action B"),
                ]),
            theme,
        )
        .into_any_element()
    };

    div()
        .flex()
        .gap(px(8.0))
        .items_center()
        .child(cell(ButtonTone::Default, "Default"))
        .child(cell(ButtonTone::Danger, "Danger"))
        .child(cell(ButtonTone::Success, "Success"))
}
